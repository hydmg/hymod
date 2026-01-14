use clap::{Parser, Subcommand};

const SKELETON_BYTES: &[u8] = include_bytes!("../../assets/skeleton.zip");

// Declare CLI command modules
#[path = "../build/mod.rs"]
mod build;
#[path = "../deploy/mod.rs"]
mod deploy;
#[path = "../dev/mod.rs"]
mod dev;
#[path = "../link/mod.rs"]
mod link;
#[path = "../new/mod.rs"]
mod new;
#[path = "../server/mod.rs"]
mod server;

#[derive(Parser)]
#[command(name = "hymod")]
#[command(about = "Hytale Mod Development Kit", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new mod project
    New(new::NewCommand),

    /// Build the current mod
    Build(build::BuildCommand),

    /// Link mod to a local server
    Link(link::LinkCommand),

    /// Development workflow with auto-rebuild
    Dev(dev::DevCommand),

    /// Deploy mod to remote server
    Deploy(deploy::DeployCommand),

    /// Manage server configurations
    #[command(subcommand)]
    Server(server::ServerCommand),
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Initialize Executor (defaulting dry_run to false for now, or TODO: add global flag)
    let executor = core_ops::Executor::new(false);

    // Route to appropriate feature module
    match cli.command {
        Commands::New(cmd) => {
            let args = features_new::NewArgs {
                name: cmd.name,
                path: cmd.path,
                group: cmd.group,
                package: cmd.package,
                no_ui_dir: cmd.no_ui_dir,
            };
            let plan = features_new::generate_plan(args, SKELETON_BYTES);
            if let Err(e) = executor.execute(&plan) {
                eprintln!("Error executing plan: {}", e);
                std::process::exit(1);
            }
        }

        Commands::Build(cmd) => {
            let args = features_build::BuildArgs {
                release: cmd.release,
            };
            let _plan = features_build::generate_plan(args);
            // TODO: Pass plan to core::ops::execute()
        }

        Commands::Link(cmd) => {
            let args = features_link::LinkArgs {
                server_name: cmd.server_name,
            };
            let _plan = features_link::generate_plan(args);
            // TODO: Pass plan to core::ops::execute()
        }

        Commands::Dev(cmd) => {
            let args = features_dev::DevArgs {
                server_name: cmd.server_name,
                watch: cmd.watch,
                restart_cmd: cmd.restart_cmd,
            };
            features_dev::run_loop(args);
        }

        Commands::Deploy(cmd) => {
            let args = features_deploy::DeployArgs {
                server_name: cmd.server_name,
                transport: cmd.transport,
                dry_run: cmd.dry_run,
            };
            let _plan = features_deploy::generate_plan(args);
            // TODO: Pass plan to core::ops::execute()
        }

        Commands::Server(cmd) => {
            let server_cmd = match cmd {
                server::ServerCommand::List => features_server::ServerCommand::List,
                server::ServerCommand::Add { name } => features_server::ServerCommand::Add { name },
                server::ServerCommand::Show { name } => {
                    features_server::ServerCommand::Show { name }
                }
                server::ServerCommand::Test { name } => {
                    features_server::ServerCommand::Test { name }
                }
            };
            features_server::execute(server_cmd);
        }
    }
}
