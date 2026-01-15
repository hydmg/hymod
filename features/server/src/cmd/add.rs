use crate::args::add_args::ServerAddArgs;
use core_config::server::{
    load_server_config, save_server_config, RemoteBlock, RestartBlock, ServerBlock, ServerConfig,
    ServerKind,
};

pub fn run(args: ServerAddArgs) {
    if load_server_config(&args.name).is_ok() {
        eprintln!("Server '{}' already exists.", args.name);
        std::process::exit(1);
    }

    let kind = match args.kind.as_str() {
        "local" => ServerKind::Local,
        "remote" | "ssh" => ServerKind::Remote,
        _ => {
            eprintln!(
                "Invalid server kind: {}. Must be 'local' or 'remote'.",
                args.kind
            );
            std::process::exit(1);
        }
    };

    let mut server_root = "/opt/hytale".to_string(); // Default
    let mut remote_block = None;

    match kind {
        ServerKind::Local => {
            server_root = args.uri.clone();
        }
        ServerKind::Remote => {
            // Check if URI is user@host or just host
            let parts: Vec<&str> = args.uri.split('@').collect();
            let (user, host) = if parts.len() == 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                ("root".to_string(), args.uri.clone()) // Default user root? Or current user?
            };

            remote_block = Some(RemoteBlock {
                host,
                user,
                port: 22,
                identity_file: None,
                known_hosts_file: None,
            });
            // server_root stays default or we could parse it from URI if standard schemes supported it (ssh://user@host/path)
            // For now, prompt implies just connection info.
        }
    }

    let config = ServerConfig {
        server: ServerBlock {
            name: args.name.clone(),
            kind,
            server_root,
            mods_dir: "mods".to_string(),
            restart: RestartBlock {
                cmd: "systemctl restart hytale".to_string(),
            },
            remote: remote_block,
            upload: None,
        },
    };

    if let Err(e) = save_server_config(&config) {
        eprintln!("Failed to save server config: {}", e);
        std::process::exit(1);
    }

    println!("Server '{}' added successfully.", args.name);
}
