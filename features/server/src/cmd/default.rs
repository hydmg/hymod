use crate::args::default_args::ServerDefaultArgs;
use core_config::server::{load_server_config, ServerKind};

pub fn run(args: ServerDefaultArgs) {
    let expected_kind = match args.kind.as_str() {
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

    let config = match load_server_config(&args.name) {
        Ok(c) => c,
        Err(_) => {
            eprintln!(
                "Server '{}' does not exist. Please add it first.",
                args.name
            );
            std::process::exit(1);
        }
    };

    if config.server.kind != expected_kind {
        eprintln!(
            "Server '{}' is {:?}, not {:?}.",
            args.name, config.server.kind, expected_kind
        );
        std::process::exit(1);
    }

    if let Err(e) = core_config::server::set_default_server_for_kind(&expected_kind, &args.name) {
        eprintln!("Failed to set default server: {}", e);
        std::process::exit(1);
    }

    println!("Default {} server set to '{}'", args.kind, args.name);
}
