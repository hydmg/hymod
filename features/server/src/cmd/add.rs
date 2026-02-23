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

    let (server_root, remote_block) = match kind {
        ServerKind::Local => (args.uri.clone(), None),
        ServerKind::Remote => {
            let (user, host, parsed_server_root) = parse_remote_uri(&args.uri);
            (
                parsed_server_root,
                Some(RemoteBlock {
                    host,
                    user,
                    port: 22,
                    identity_file: None,
                    known_hosts_file: None,
                }),
            )
        }
    };

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

fn parse_remote_uri(uri: &str) -> (String, String, String) {
    let default_root = "/opt/hytale".to_string();

    let (user, host_and_maybe_path) = match uri.split_once('@') {
        Some((u, rest)) if !u.is_empty() => (u.to_string(), rest),
        _ => ("root".to_string(), uri),
    };

    let (host, server_root) = match host_and_maybe_path.split_once(":/") {
        Some((h, root_tail)) if !h.is_empty() => (h.to_string(), format!("/{}", root_tail)),
        _ => (host_and_maybe_path.to_string(), default_root),
    };

    if host.is_empty() {
        panic!("Invalid remote URI: host is required");
    }

    (user, host, server_root)
}

#[cfg(test)]
mod tests {
    use super::parse_remote_uri;

    #[test]
    fn parses_user_host_and_path() {
        let (user, host, server_root) =
            parse_remote_uri("root@170.205.24.203:/root/playground/hymodtest");
        assert_eq!(user, "root");
        assert_eq!(host, "170.205.24.203");
        assert_eq!(server_root, "/root/playground/hymodtest");
    }

    #[test]
    fn parses_user_and_host_without_path() {
        let (user, host, server_root) = parse_remote_uri("user@example.com");
        assert_eq!(user, "user");
        assert_eq!(host, "example.com");
        assert_eq!(server_root, "/opt/hytale");
    }

    #[test]
    fn parses_host_and_path_with_default_user() {
        let (user, host, server_root) = parse_remote_uri("170.205.24.203:/srv/hytale");
        assert_eq!(user, "root");
        assert_eq!(host, "170.205.24.203");
        assert_eq!(server_root, "/srv/hytale");
    }
}
