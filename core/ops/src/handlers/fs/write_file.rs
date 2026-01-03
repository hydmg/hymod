use crate::traits::OpHandler;
use anyhow::{Context, Result};
use hymod_core_plan::Step;
use std::fs;
use std::path::Path;

pub struct WriteFileHandler;

impl OpHandler for WriteFileHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::WriteFile { path, content } = step {
            if let Some(parent) = Path::new(path).parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, content).context(format!("Failed to write file {}", path))?;
            Ok(())
        } else {
            panic!("WriteFileHandler called with non-WriteFile step");
        }
    }
}
