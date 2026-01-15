pub struct DeployArgs {
    pub server_name: Option<String>,
    pub transport: Option<String>,
    pub dry_run: bool,
}

use core_config::{get_default_server, load_server_config, ServerKind};
use core_plan::{Plan, Step};
use std::fs;
use std::path::PathBuf;

pub fn generate_plan(args: DeployArgs) -> core_plan::Plan {
    let server_name = args
        .server_name
        .or_else(|| get_default_server().ok().flatten())
        .expect("No server specified and no default configured");

    let config = load_server_config(&server_name).expect("Failed to load server config");

    let mut steps = Vec::new();

    // 1. Build
    let gradlew = if cfg!(windows) {
        "gradlew.bat"
    } else {
        "./gradlew"
    };
    steps.push(Step::RunProcess {
        cmd: gradlew.to_string(),
        args: vec!["build".to_string()],
        cwd: None,
    });

    // 2. Identify Artifact
    // Heuristic: Read gradle.properties to predict the name
    let jar_name = if let Ok(content) = fs::read_to_string("gradle.properties") {
        let mut name = "mod".to_string();
        let mut version = "1.0.0".to_string();
        for line in content.lines() {
            if let Some(stripped) = line.strip_prefix("mod_name=") {
                name = stripped.trim().to_string();
            }
            if let Some(stripped) = line.strip_prefix("mod_version=") {
                version = stripped.trim().to_string();
            }
        }
        format!("{}-{}.jar", name, version)
    } else {
        "mod.jar".to_string()
    };

    let source_path = format!("build/libs/{}", jar_name);
    // dest_path depends on kind

    // 3. Deploy
    match config.server.kind {
        ServerKind::Local => {
            let full_dest = PathBuf::from(&config.server.server_root)
                .join(&config.server.mods_dir)
                .join(&jar_name);

            steps.push(Step::CopyFile {
                from: source_path,
                to: full_dest.to_string_lossy().to_string(),
            });
        }
        ServerKind::Remote => {
            let remote_dest = format!("{}/{}", config.server.mods_dir, jar_name);
            if config.server.remote.is_some() {
                // Determine transport
                if args.transport.as_deref() == Some("scp") {
                    steps.push(Step::UploadScp {
                        local: source_path,
                        remote: remote_dest,
                    });
                } else {
                    // Default rsync
                    steps.push(Step::UploadRsync {
                        local: source_path,
                        remote: remote_dest,
                        opts: "-avz".to_string(), // Default opts
                    });
                }
            } else {
                // Should not happen if confirmed remote kind implies remote block, but safe to panic or error
                panic!("Remote server config missing remote block");
            }
        }
    }

    // 4. Restart
    steps.push(Step::RunProcess {
        cmd: config.server.restart.cmd.clone(),
        args: vec![],
        cwd: None,
    });

    Plan { steps }
}
