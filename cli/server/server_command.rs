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
