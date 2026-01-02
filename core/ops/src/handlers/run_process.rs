use crate::traits::OpHandler;
use anyhow::{Context, Result};
use hymod_core_plan::Step;
use std::process::Command;

pub struct RunProcessHandler;

impl OpHandler for RunProcessHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::RunProcess { cmd, args, cwd } = step {
            let mut command = Command::new(cmd);
            command.args(args);
            if let Some(dir) = cwd {
                command.current_dir(dir);
            }
            let status = command
                .status()
                .context(format!("Failed to execute {}", cmd))?;
            if !status.success() {
                anyhow::bail!("Command failed: {} {}", cmd, args.join(" "));
            }
            Ok(())
        } else {
            panic!(
                "RunProcessHandler called with non-RunProcess step: {:?}",
                step
            );
        }
    }
}
