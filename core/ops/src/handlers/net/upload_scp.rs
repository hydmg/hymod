use crate::traits::OpHandler;
use anyhow::{bail, Context, Result};
use hymod_core_plan::Step;
use std::process::Command;

pub struct UploadScpHandler;

impl OpHandler for UploadScpHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::UploadScp { local, remote } = step {
            let status = Command::new("scp")
                .arg(local)
                .arg(remote)
                .status()
                .context("Failed to run scp")?;
            if !status.success() {
                bail!("Scp failed");
            }
            Ok(())
        } else {
            panic!("UploadScpHandler called with non-UploadScp step");
        }
    }
}
