use clap::Args;

#[derive(Args, Debug)]
pub struct LinkCommand {
    pub server_name: Option<String>,
}

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for LinkCommand {
    fn run(&self, executor: &Executor) -> Result<()> {
        let args = features_link::LinkArgs {
            server_name: self.server_name.clone(),
        };
        let plan = features_link::generate_plan(args);
        executor.execute(&plan)?;
        Ok(())
    }
}
