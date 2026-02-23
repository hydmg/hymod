pub struct DeployArgs {
    pub server_name: Option<String>,
    pub transport: Option<String>,
    pub restart: bool,
    pub dry_run: bool,
    pub path: Option<PathBuf>,
}

use core_config::{get_default_server_for_kind, load_server_config, ServerKind};
use core_path::resolve_mod_artifact;
use core_plan::{Plan, Step};
use std::path::Path;
use std::path::PathBuf;

pub fn generate_plan(args: DeployArgs) -> core_plan::Plan {
    let server_name = args
        .server_name
        .or_else(|| {
            get_default_server_for_kind(&ServerKind::Remote)
                .ok()
                .flatten()
        })
        .expect("No server specified and no default remote server configured");

    let config = load_server_config(&server_name).expect("Failed to load server config");

    // Resolve mod directory
    let mod_dir = args
        .path
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

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
        cwd: Some(mod_dir.to_string_lossy().to_string()),
    });

    // 2. Identify Artifact
    let artifact = resolve_mod_artifact(&mod_dir);
    let jar_name = artifact.target_file_name;
    let source_path = artifact.source_path.to_string_lossy().to_string();

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
            if let Some(remote) = config.server.remote.as_ref() {
                let remote_mods_dir = PathBuf::from(&config.server.server_root)
                    .join(&config.server.mods_dir)
                    .to_string_lossy()
                    .to_string();

                // Ensure remote destination directory exists before upload.
                steps.push(Step::SshRun {
                    host: remote.host.clone(),
                    user: remote.user.clone(),
                    cmd: format!("mkdir -p {}", remote_mods_dir),
                });

                let remote_file_path = PathBuf::from(&config.server.server_root)
                    .join(&config.server.mods_dir)
                    .join(&jar_name)
                    .to_string_lossy()
                    .to_string();
                let remote_dest = format!("{}@{}:{}", remote.user, remote.host, remote_file_path);
                let transport = resolve_transport(args.transport.as_deref());

                if transport == "rsync" {
                    steps.push(Step::UploadRsync {
                        local: source_path,
                        remote: remote_dest,
                        opts: "-avz".to_string(),
                    });
                } else {
                    // Default SCP
                    steps.push(Step::UploadScp {
                        local: source_path,
                        remote: remote_dest,
                    });
                }
            } else {
                panic!("Remote server config missing remote block");
            }
        }
    }

    // 4. Restart (opt-in)
    if args.restart {
        steps.push(Step::RunProcess {
            cmd: config.server.restart.cmd.clone(),
            args: vec![],
            cwd: None,
        });
    }

    Plan { steps }
}

fn resolve_transport(explicit: Option<&str>) -> &'static str {
    match explicit {
        Some("rsync") => "rsync",
        Some("scp") => "scp",
        Some(other) => panic!("Invalid transport: must be rsync or scp (got '{other}')"),
        None => {
            if is_command_available("rsync") {
                "rsync"
            } else {
                "scp"
            }
        }
    }
}

fn is_command_available(cmd: &str) -> bool {
    let Some(path_var) = std::env::var_os("PATH") else {
        return false;
    };

    std::env::split_paths(&path_var).any(|dir| command_exists_in_dir(&dir, cmd))
}

fn command_exists_in_dir(dir: &Path, cmd: &str) -> bool {
    if cfg!(windows) {
        for candidate in [cmd.to_string(), format!("{cmd}.exe"), format!("{cmd}.bat")] {
            if dir.join(candidate).is_file() {
                return true;
            }
        }
        false
    } else {
        dir.join(cmd).is_file()
    }
}
