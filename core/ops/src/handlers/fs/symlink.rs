use crate::traits::OpHandler;
use anyhow::{Context, Result};
use hymod_core_plan::Step;
use std::fs;
use std::path::Path;

pub struct SymlinkHandler;

impl OpHandler for SymlinkHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::Symlink { from, to } = step {
            if let Some(parent) = Path::new(to).parent() {
                fs::create_dir_all(parent)?;
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::symlink;
                if Path::new(to).exists() || fs::symlink_metadata(to).is_ok() {
                    fs::remove_file(to).ok();
                }
                symlink(from, to).context(format!("Failed to symlink {} -> {}", from, to))?;
                Ok(())
            }
            #[cfg(not(unix))]
            {
                anyhow::bail!("Symlinks only supported on Unix for now");
            }
        } else {
            panic!("SymlinkHandler called with non-Symlink step");
        }
    }
}
