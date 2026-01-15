use crate::command::CliCommand;
use anyhow::Result;
use clap::{Args, Subcommand};
use core_config::UserConfig;

#[derive(Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub command: ConfigSubcommand,
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Set a configuration value
    Set { key: String, value: String },
    /// Get a configuration value
    Get { key: String },
    /// List all configuration values
    List,
}

impl CliCommand for ConfigCommand {
    fn run(&self, _executor: &core_ops::Executor) -> Result<()> {
        let mut config = UserConfig::load();

        match &self.command {
            ConfigSubcommand::Set { key, value } => {
                config
                    .set(key, value.clone())
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
                config.save().map_err(|e| anyhow::anyhow!("{}", e))?;
                println!("Set {} = {}", key, value);
            }
            ConfigSubcommand::Get { key } => {
                if let Some(val) = config.get(key) {
                    println!("{}", val);
                } else {
                    println!("Key '{}' not found.", key);
                }
            }
            ConfigSubcommand::List => {
                println!("Global Configuration:");
                println!(
                    "  author: {}",
                    config.author.as_deref().unwrap_or("<unset>")
                );
                println!(
                    "  username: {}",
                    config.username.as_deref().unwrap_or("<unset>")
                );
                println!("  group: {}", config.group.as_deref().unwrap_or("<unset>"));
                println!(
                    "  version: {}",
                    config.version.as_deref().unwrap_or("<unset>")
                );
                println!(
                    "  description: {}",
                    config.desc.as_deref().unwrap_or("<unset>")
                );
            }
        }
        Ok(())
    }
}
