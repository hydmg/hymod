use crate::traits::OpHandler;
use anyhow::{Context, Result};
use hymod_core_plan::Step;
use std::fs;
use std::path::Path;

pub struct CopyFileHandler;

impl OpHandler for CopyFileHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::CopyFile { from, to } = step {
            if let Some(parent) = Path::new(to).parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(from, to).context(format!("Failed to copy {} to {}", from, to))?;
            Ok(())
        } else {
            panic!("CopyFileHandler called with non-CopyFile step");
        }
    }
}
