use crate::args::DevArgs;
use anyhow::{bail, Context, Result};
use colored::*;
use core_path::resolve_mod_artifact;
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn run(args: DevArgs) -> Result<()> {
    // 1. Resolve mod directory
    let mod_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to get current directory")?,
    };

    println!("{} {}", "Dev Loop:".blue().bold(), mod_dir.display());

    // 2. Run ./gradlew
    #[cfg(windows)]
    let gradle_wrapper = mod_dir.join("gradlew.bat");
    #[cfg(not(windows))]
    let gradle_wrapper = mod_dir.join("gradlew");

    if !gradle_wrapper.exists() {
        bail!(
            "No gradle wrapper found at {}. Is this a mod directory?",
            gradle_wrapper.display()
        );
    }
    // canonicalize causes path issues on Windows (\\?\) so we use the path as-is.
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
    let artifact = resolve_mod_artifact(&mod_dir);
    if !artifact.source_path.exists() {
        bail!(
            "Could not find a suitable .jar artifact in build/libs/ (expected {}).",
            artifact.source_path.display()
        );
    }

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
        let default_server_name =
            core_config::get_default_server_for_kind(&core_config::ServerKind::Local)
                .map_err(|e| anyhow::anyhow!("Failed to get default local server: {}", e))?
                .context(
                    "No target provided and no default local server configured. Use 'hymod server default local <NAME>' or provide a target argument.",
                )?;

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
        std::fs::create_dir_all(&destination_dir).with_context(|| {
            format!(
                "Failed to create destination directory: {}",
                destination_dir.display()
            )
        })?;
    }

    // 5. Copy artifact
    let target_name = artifact.target_file_name;

    let dest_file = destination_dir.join(&target_name);

    println!(
        "{} Deploying {} to: {}",
        ">>".green(),
        target_name,
        destination_dir.display()
    );
    std::fs::copy(&artifact.source_path, &dest_file)
        .with_context(|| format!("Failed to copy artifact to {}", destination_dir.display()))?;

    println!(
        "{} Deployed successfully: {}",
        "SUCCESS".green().bold(),
        dest_file.display().to_string().cyan()
    );

    Ok(())
}
