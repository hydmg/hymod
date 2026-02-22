use crate::command::CliCommand;
use clap::{CommandFactory, Parser, Subcommand};
use std::ffi::OsString;

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

fn print_hymod_logo() {
    let o = "\x1b[38;5;208m"; // Orange
    let g = "\x1b[1;30m"; // Dark Grey
    let b = "\x1b[38;5;67m"; // Slate Blue
    let n = "\x1b[0m"; // Reset

    println!("{}   .^\\.      {} _                                 _ ", o, b);
    println!(
        "{}  / _  \\     {}| |__   _   _  _ __ ___    ___   __| |",
        o, b
    );
    println!(
        "{} /_/ \\__\\    {}| '_ \\ | | | || '_ ` _ \\  / _ \\ / _` |",
        o, b
    );
    println!(
        "{} \\ \\  _ /    {}| | | || |_| || | | | | || (_) | (_| |",
        g, b
    );
    println!(
        "{}  \\ \\/ /     {}|_| |_| \\__, ||_| |_| |_| \\___/ \\__,_|",
        g, b
    );
    println!("{}   \\__/               {}|___/                         {}", g, b, n);
}

fn should_show_logo(args: &[OsString]) -> bool {
    if args.len() == 1 {
        return true;
    }

    matches!(
        args.get(1).and_then(|arg| arg.to_str()),
        Some("help") | Some("-h") | Some("--help")
    ) || args
        .iter()
        .skip(1)
        .any(|arg| matches!(arg.to_str(), Some("-h") | Some("--help")))
}

fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();
    if should_show_logo(&args) {
        print_hymod_logo();
    }

    if args.len() == 1 {
        let mut cmd = Cli::command();
        if let Err(e) = cmd.print_help() {
            eprintln!("Error printing help: {}", e);
            std::process::exit(1);
        }
        println!();
        return;
    }

    // Parse command-line arguments
    let cli = Cli::parse_from(args);

    // Initialize Executor (defaulting dry_run to false for now, or TODO: add global flag)
    let executor = core_ops::Executor::new(false);

    // Route to appropriate feature module
    if let Err(e) = cli.command.run(&executor) {
        eprintln!("Error executing command: {}", e);
        std::process::exit(1);
    }
}
