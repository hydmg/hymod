use crate::args::default_args::ServerDefaultArgs;
use core_config::server::load_server_config;

pub fn run(args: ServerDefaultArgs) {
    if load_server_config(&args.name).is_err() {
        eprintln!(
            "Server '{}' does not exist. Please add it first.",
            args.name
        );
        std::process::exit(1);
    }

    if let Err(e) = core_config::server::set_default_server(&args.name) {
        eprintln!("Failed to set default server: {}", e);
        std::process::exit(1);
    }

    println!("Default server set to '{}'", args.name);
}
