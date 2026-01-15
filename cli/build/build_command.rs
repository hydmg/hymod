use clap::Args;

#[derive(Args, Debug)]
pub struct BuildCommand {
    #[arg(long)]
    pub release: bool,
}

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for BuildCommand {
    fn run(&self, executor: &Executor) -> Result<()> {
        let args = features_build::BuildArgs {
            release: self.release,
        };
        let plan = features_build::generate_plan(args);
        executor.execute(&plan)?;
        Ok(())
    }
}
