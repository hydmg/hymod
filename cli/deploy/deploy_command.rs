use clap::Args;

#[derive(Args, Debug)]
pub struct DeployCommand {
    pub server_name: Option<String>,
    #[arg(long)]
    pub transport: Option<String>,
    /// Restart server after upload (disabled by default)
    #[arg(long)]
    pub restart: bool,
    #[arg(long)]
    pub dry_run: bool,
    /// Path to the mod directory (default: current directory)
    #[arg(long, short)]
    pub path: Option<std::path::PathBuf>,
}

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for DeployCommand {
    fn run(&self, executor: &Executor) -> Result<()> {
        let args = features_deploy::DeployArgs {
            server_name: self.server_name.clone(),
            transport: self.transport.clone(),
            restart: self.restart,
            dry_run: self.dry_run,
            path: self.path.clone(),
        };
        let plan = features_deploy::generate_plan(args);

        if self.dry_run {
            let dry_executor = Executor::new(true);
            dry_executor.execute(&plan)?;
        } else {
            executor.execute(&plan)?;
        }
        Ok(())
    }
}
