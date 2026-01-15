use crate::command::CliCommand;
use clap::{Parser, Subcommand};

// Declare CLI command modules
#[path = "../build/mod.rs"]
mod build;
mod command;
#[path = "../config/mod.rs"]
mod config;
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

    /// Manage global configuration
    Config(config::ConfigCommand),
}

impl CliCommand for Commands {
    fn run(&self, executor: &core_ops::Executor) -> anyhow::Result<()> {
        match self {
            Commands::New(cmd) => cmd.run(executor),
            Commands::Build(cmd) => cmd.run(executor),
            Commands::Link(cmd) => cmd.run(executor),
            Commands::Dev(cmd) => cmd.run(executor),
            Commands::Deploy(cmd) => cmd.run(executor),
            Commands::Server(cmd) => cmd.run(executor),
            Commands::Config(cmd) => cmd.run(executor),
        }
    }
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Initialize Executor (defaulting dry_run to false for now, or TODO: add global flag)
    let executor = core_ops::Executor::new(false);

    // Route to appropriate feature module
    if let Err(e) = cli.command.run(&executor) {
        eprintln!("Error executing command: {}", e);
        std::process::exit(1);
    }
}
// change main
