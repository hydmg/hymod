use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ServerCommand {
    /// List all configured servers
    List,
    /// Add a new server configuration
    Add { name: String },
    /// Show server configuration details
    Show { name: String },
    /// Test server connectivity
    Test { name: String },
}

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for ServerCommand {
    fn run(&self, _executor: &Executor) -> Result<()> {
        let cmd = match self {
            ServerCommand::List => features_server::ServerCommand::List,
            ServerCommand::Add { name } => {
                features_server::ServerCommand::Add { name: name.clone() }
            }
            ServerCommand::Show { name } => {
                features_server::ServerCommand::Show { name: name.clone() }
            }
            ServerCommand::Test { name } => {
                features_server::ServerCommand::Test { name: name.clone() }
            }
        };
        features_server::execute(cmd);
        Ok(())
    }
}
