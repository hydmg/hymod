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
    let logo = include_str!("ascii-art.txt");
    print!("{}", colorize_logo(logo));
    println!();
}

fn colorize_logo(logo: &str) -> String {
    const RESET: &str = "\x1b[0m";
    const NAVY: &str = "\x1b[38;5;60m";
    const ORANGE_BRIGHT: &str = "\x1b[38;5;214m";
    const ORANGE: &str = "\x1b[38;5;208m";
    const STEEL: &str = "\x1b[38;5;245m";
    const STEEL_DARK: &str = "\x1b[38;5;240m";
    const STEEL_DEEP: &str = "\x1b[38;5;236m";

    let mut out = String::with_capacity(logo.len() + 512);

    for (row_idx, line) in logo.lines().enumerate() {
        let row = row_idx + 1;
        let wordmark_start = find_wordmark_start(line);
        let mut current_color = "";

        for (col_idx, ch) in line.chars().enumerate() {
            let col = col_idx + 1;

            if ch == ' ' {
                if !current_color.is_empty() {
                    out.push_str(RESET);
                    current_color = "";
                }
                out.push(' ');
                continue;
            }

            let is_wordmark = match wordmark_start {
                Some(start) => col >= start,
                None => false,
            };

            let target_color = if is_wordmark {
                NAVY
            } else if row <= 4 {
                ORANGE_BRIGHT
            } else if row <= 8 {
                ORANGE
            } else if row <= 12 {
                STEEL
            } else if row <= 16 {
                STEEL_DARK
            } else {
                STEEL_DEEP
            };

            if current_color != target_color {
                out.push_str(target_color);
                current_color = target_color;
            }
            out.push(ch);
        }

        if !current_color.is_empty() {
            out.push_str(RESET);
        }
        out.push('\n');
    }

    out
}

fn find_wordmark_start(line: &str) -> Option<usize> {
    let mut seen_non_space = false;
    let mut run_start = 0usize;
    let mut run_len = 0usize;

    for (col_idx, ch) in line.chars().enumerate() {
        let col = col_idx + 1;

        if ch == ' ' {
            if run_len == 0 {
                run_start = col;
            }
            run_len += 1;
            continue;
        }

        if run_len >= 3 && seen_non_space && run_start > 20 {
            return Some(col);
        }

        run_len = 0;
        seen_non_space = true;
    }

    None
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
