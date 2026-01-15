use crate::args::DevArgs;
use anyhow::{bail, Context, Result};
use colored::*;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn run(args: DevArgs) -> Result<()> {
    // 1. Resolve mod directory
    let mod_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to get current directory")?,
    };

    println!("{} {}", "Dev Loop:".blue().bold(), mod_dir.display());

    // 2. Run ./gradlew
    let gradle_wrapper = mod_dir.join("gradlew");
    if !gradle_wrapper.exists() {
        bail!(
            "No gradle wrapper found at {}. Is this a mod directory?",
            gradle_wrapper.display()
        );
    }
    let gradle_wrapper = gradle_wrapper
        .canonicalize()
        .context("Failed to canonicalize gradle wrapper path")?;

    // Ensure executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(&gradle_wrapper) {
            let mut perms = metadata.permissions();
            if perms.mode() & 0o111 == 0 {
                println!(
                    "{} Gradle wrapper not executable. Attempting to fix...",
                    "!!".yellow()
                );
                perms.set_mode(perms.mode() | 0o755);
                let _ = std::fs::set_permissions(&gradle_wrapper, perms);
            }
        }
    }

    println!("{} Running Gradle build...", ">>".green());

    let config = core_config::UserConfig::load();

    let mut cmd = Command::new(&gradle_wrapper);
    cmd.current_dir(&mod_dir).arg("build");

    if let Some(username) = config.username {
        cmd.arg(format!("-Pusername={}", username));
    }

    let status = cmd
        .status()
        .context("Failed to execute gradle wrapper. Check permissions.")?;

    if !status.success() {
        bail!("Gradle build failed");
    }

    // 3. Identify artifact
    // Look for jars in build/libs/ - excluding sources/javadoc jars if possible, but for simplicity pick the first likely candidate or all of them.
    // The requirement says "ONLY THE CORRECT ONE, NOT INCLUDED LIBRARIES".
    // Usually build/libs contains: mod-version.jar, mod-version-sources.jar, etc.
    // We should pick the main one. Typically the one that doesn't end in -sources.jar or -javadoc.jar.

    let build_libs = mod_dir.join("build").join("libs");
    if !build_libs.exists() {
        bail!("Build directory not found at {}. Gradle build may have failed silently or output elsewhere.", build_libs.display());
    }

    let mut artifact_path: Option<PathBuf> = None;

    for entry in WalkDir::new(&build_libs)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "jar") {
            let file_name = path.file_name().unwrap().to_string_lossy();
            if !file_name.contains("-sources")
                && !file_name.contains("-javadoc")
                && !file_name.contains("-plain")
            {
                artifact_path = Some(path.to_path_buf());
                break;
            }
        }
    }

    let artifact =
        artifact_path.context("Could not find a suitable .jar artifact in build/libs/")?;

    // 4. Resolve Target and deploy
    let destination_dir = if let Some(target_str) = args.target {
        // Check if target_str is a valid directory path
        let target_path = PathBuf::from(&target_str);
        if target_path.exists() && target_path.is_dir() {
            // Check for "mods" subdirectory
            let mods_sub = target_path.join("mods");
            if mods_sub.exists() && mods_sub.is_dir() {
                mods_sub
            } else {
                target_path
            }
        } else {
            // Assume it's a server name
            let server_cfg = core_config::load_server_config(&target_str).map_err(|e| {
                anyhow::anyhow!(
                    "Target '{}' is not a directory and not a known server. Error: {}",
                    target_str,
                    e
                )
            })?;

            if let core_config::ServerKind::Local = server_cfg.server.kind {
                PathBuf::from(server_cfg.server.server_root).join(server_cfg.server.mods_dir)
            } else {
                bail!("Dev command currently only supports local servers or paths.");
            }
        }
    } else {
        // Use default server
        let default_server_name = core_config::get_default_server()
            .map_err(|e| anyhow::anyhow!("Failed to get default server: {}", e))?
            .context("No target provided and no default server configured. Use 'hymod server set-default <NAME>' or provide a target argument.")?;

        let server_cfg = core_config::load_server_config(&default_server_name)
            .map_err(|e| anyhow::anyhow!("Failed to load server config: {}", e))?;
        if let core_config::ServerKind::Local = server_cfg.server.kind {
            PathBuf::from(server_cfg.server.server_root).join(server_cfg.server.mods_dir)
        } else {
            bail!(
                "Default server '{}' is remote. Dev command only supports local servers.",
                default_server_name
            );
        }
    };

    if !destination_dir.exists() {
        bail!(
            "Destination directory does not exist: {}",
            destination_dir.display()
        );
    }

    // 5. Copy artifact
    let file_name = artifact.file_name().context("Artifact has no file name")?;
    let dest_file = destination_dir.join(file_name);

    println!(
        "{} Deploying to: {}",
        ">>".green(),
        destination_dir.display()
    );
    std::fs::copy(&artifact, &dest_file)
        .with_context(|| format!("Failed to copy artifact to {}", destination_dir.display()))?;

    println!(
        "{} Deployed successfully: {}",
        "SUCCESS".green().bold(),
        dest_file.display().to_string().cyan()
    );

    Ok(())
}
