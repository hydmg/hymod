use clap::Args;

#[derive(Args, Debug)]
pub struct DevCommand {
    pub server_name: Option<String>,
    #[arg(long)]
    pub watch: bool,
    #[arg(long)]
    pub restart_cmd: Option<String>,
}

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for DevCommand {
    fn run(&self, _executor: &Executor) -> Result<()> {
        let args = features_dev::DevArgs {
            server_name: self.server_name.clone(),
            watch: self.watch,
            restart_cmd: self.restart_cmd.clone(),
        };
        features_dev::run_loop(args);
        Ok(())
    }
}
