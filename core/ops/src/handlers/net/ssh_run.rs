use crate::traits::OpHandler;
use anyhow::{bail, Context, Result};
use hymod_core_plan::Step;
use std::process::Command;

pub struct SshRunHandler;

impl OpHandler for SshRunHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::SshRun { host, user, cmd } = step {
            let status = Command::new("ssh")
                .arg(format!("{}@{}", user, host))
                .arg(cmd)
                .status()
                .context("Failed to run ssh command")?;
            if !status.success() {
                bail!("Ssh command failed");
            }
            Ok(())
        } else {
            panic!("SshRunHandler called with non-SshRun step");
        }
    }
}
