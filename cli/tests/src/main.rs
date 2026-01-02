use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hymod")]
#[command(about = "Hytale mod development workflow tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new mod project
    New {
        name: String,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        group: Option<String>,
        #[arg(long)]
        package: Option<String>,
        #[arg(long)]
        no_ui_dir: bool,
    },

    /// Build the current mod
    Build {
        #[arg(long)]
        release: bool,
    },

    /// Link mod to a local server
    Link { server_name: Option<String> },

    /// Development workflow with auto-rebuild
    Dev {
        server_name: Option<String>,
        #[arg(long)]
        watch: bool,
        #[arg(long)]
        restart_cmd: Option<String>,
    },

    /// Deploy mod to remote server
    Deploy {
        server_name: Option<String>,
        #[arg(long)]
        transport: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },

    /// Manage server configurations
    Server {
        #[command(subcommand)]
        cmd: ServerSubcommand,
    },
    /* Store and Status features removed - uncomment when implementing
    /// Manage artifact store
    Store {
        #[command(subcommand)]
        cmd: StoreSubcommand,
    },

    /// Check mod and server status
    Status { server_name: Option<String> },
    */
}

#[derive(Subcommand)]
enum ServerSubcommand {
    /// List all configured servers
    List,
    /// Add a new server configuration
    Add { name: String },
    /// Show server configuration details
    Show { name: String },
    /// Test server connectivity
    Test { name: String },
}

#[derive(Subcommand)]
enum StoreSubcommand {
    /// List versions for a mod
    List { mod_id: String },
    /// Get path to a specific mod version
    Path { mod_id: String, version: String },
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Route to appropriate feature module
    match cli.command {
        Commands::New {
            name,
            path,
            group,
            package,
            no_ui_dir,
        } => {
            let args = features_new::NewArgs {
                name,
                path,
                group,
                package,
                no_ui_dir,
            };
            let _plan = features_new::generate_plan(args);
            // TODO: Pass plan to core::ops::execute()
        }

        Commands::Build { release } => {
            let args = features_build::BuildArgs { release };
            let _plan = features_build::generate_plan(args);
            // TODO: Pass plan to core::ops::execute()
        }

        Commands::Link { server_name } => {
            let args = features_link::LinkArgs { server_name };
            let _plan = features_link::generate_plan(args);
            // TODO: Pass plan to core::ops::execute()
        }

        Commands::Dev {
            server_name,
            watch,
            restart_cmd,
        } => {
            let args = features_dev::DevArgs {
                server_name,
                watch,
                restart_cmd,
            };
            features_dev::run_loop(args);
        }

        Commands::Deploy {
            server_name,
            transport,
            dry_run,
        } => {
            let args = features_deploy::DeployArgs {
                server_name,
                transport,
                dry_run,
            };
            let _plan = features_deploy::generate_plan(args);
            // TODO: Pass plan to core::ops::execute()
        }

        Commands::Server { cmd } => {
            let server_cmd = match cmd {
                ServerSubcommand::List => features_server::ServerCommand::List,
                ServerSubcommand::Add { name } => features_server::ServerCommand::Add { name },
                ServerSubcommand::Show { name } => features_server::ServerCommand::Show { name },
                ServerSubcommand::Test { name } => features_server::ServerCommand::Test { name },
            };
            features_server::execute(server_cmd);
        } /* Store and Status features removed - uncomment when implementing
          Commands::Store { cmd } => {
              let store_cmd = match cmd {
                  StoreSubcommand::List { mod_id } => features_store::StoreCommand::List { mod_id },
                  StoreSubcommand::Path { mod_id, version } => {
                      features_store::StoreCommand::Path { mod_id, version }
                  }
              };
              features_store::execute(store_cmd);
          }

          Commands::Status { server_name } => {
              let args = features_status::StatusArgs { server_name };
              features_status::execute(args);
          }
          */
    }

    todo!("Wire up clap routing and plan execution");
}
