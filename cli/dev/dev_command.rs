use clap::Args;

#[derive(Args, Debug)]
pub struct DevCommand {
    /// Optional target server name or path to deploy to
    #[arg(index = 1)]
    pub target: Option<String>,

    /// Path to the mod directory (default: current directory)
    #[arg(long, short)]
    pub path: Option<std::path::PathBuf>,
}

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for DevCommand {
    fn run(&self, _executor: &Executor) -> Result<()> {
        let args = features_dev::DevArgs {
            target: self.target.clone(),
            path: self.path.clone(),
        };
        features_dev::run(args)?;
        Ok(())
    }
}
