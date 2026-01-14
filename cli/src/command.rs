use anyhow::Result;
use core_ops::Executor;

pub trait CliCommand {
    fn run(&self, executor: &Executor) -> Result<()>;
}
