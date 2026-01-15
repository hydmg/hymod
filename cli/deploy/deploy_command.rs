use clap::Args;

#[derive(Args, Debug)]
pub struct DeployCommand {
    pub server_name: Option<String>,
    #[arg(long)]
    pub transport: Option<String>,
    #[arg(long)]
    pub dry_run: bool,
}

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for DeployCommand {
    fn run(&self, executor: &Executor) -> Result<()> {
        let args = features_deploy::DeployArgs {
            server_name: self.server_name.clone(),
            transport: self.transport.clone(),
            dry_run: self.dry_run,
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
