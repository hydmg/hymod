use crate::traits::OpHandler;
use anyhow::{bail, Context, Result};
use hymod_core_plan::Step;
use std::process::Command;

pub struct UploadRsyncHandler;

impl OpHandler for UploadRsyncHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::UploadRsync {
            local,
            remote,
            opts,
        } = step
        {
            let status = Command::new("rsync")
                .arg(opts)
                .arg(local)
                .arg(remote)
                .status()
                .context("Failed to run rsync")?;
            if !status.success() {
                bail!("Rsync failed");
            }
            Ok(())
        } else {
            panic!("UploadRsyncHandler called with non-UploadRsync step");
        }
    }
}
