use crate::args::remove_args::ServerRemoveArgs;
use core_config::server::remove_server_config;

pub fn run(args: ServerRemoveArgs) {
    if let Err(e) = remove_server_config(&args.name) {
        eprintln!("Failed to remove server: {}", e);
        std::process::exit(1);
    }

    println!("Server '{}' removed successfully.", args.name);
}
