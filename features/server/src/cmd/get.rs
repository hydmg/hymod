use crate::args::get_args::ServerGetArgs;
use core_config::server::load_server_config;

pub fn run(args: ServerGetArgs) {
    match load_server_config(&args.name) {
        Ok(config) => {
            println!("{}", config.server.server_root);
        }
        Err(_) => {
            eprintln!("Server '{}' not found", args.name);
            std::process::exit(1);
        }
    }
}
