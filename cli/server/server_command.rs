use crate::command::CliCommand;
use anyhow::Result;
use clap::Subcommand;
use core_ops::Executor;

#[derive(Subcommand, Debug)]
pub enum ServerCommand {
    /// List all configured servers
    List {
        /// Filter by kind (local or remote)
        #[arg(index = 1)]
        kind: Option<String>,
    },

    /// Add a new server configuration
    Add {
        /// Type of server (local or remote)
        #[arg(index = 1)]
        kind: String, // "local" or "remote"

        /// Name of the server
        #[arg(index = 2)]
        name: String,

        /// Path (for local) or URL (for remote, e.g., user@host)
        #[arg(index = 3)]
        uri: String,
    },

    /// Set default server
    Default {
        /// Type of server (local/ssh)
        #[arg(index = 1)]
        kind: String,

        /// Name of the server
        #[arg(index = 2)]
        name: String,
    },

    /// Remove a server configuration
    Remove {
        /// Name of the server to remove
        #[arg(index = 1)]
        name: String,
    },

    /// Get server path and details
    Get {
        /// Name of the server
        #[arg(index = 1)]
        name: String,
    },
}

impl CliCommand for ServerCommand {
    fn run(&self, _executor: &Executor) -> Result<()> {
        let cmd = match self {
            ServerCommand::List { kind } => features_server::ServerCommand::List(
                features_server::args::list_args::ServerListArgs {
                    filter_kind: kind.clone(),
                },
            ),
            ServerCommand::Add { kind, name, uri } => features_server::ServerCommand::Add(
                features_server::args::add_args::ServerAddArgs {
                    name: name.clone(),
                    kind: kind.clone(),
                    uri: uri.clone(),
                    server_root: None,
                },
            ),
            ServerCommand::Default { kind, name } => features_server::ServerCommand::Default(
                features_server::args::default_args::ServerDefaultArgs {
                    kind: kind.clone(),
                    name: name.clone(),
                },
            ),
            ServerCommand::Remove { name } => features_server::ServerCommand::Remove(
                features_server::args::remove_args::ServerRemoveArgs { name: name.clone() },
            ),
            ServerCommand::Get { name } => features_server::ServerCommand::Get(
                features_server::args::get_args::ServerGetArgs { name: name.clone() },
            ),
        };
        features_server::execute(cmd);
        Ok(())
    }
}
