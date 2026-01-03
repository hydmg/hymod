use crate::traits::OpHandler;
use anyhow::{Context, Result};
use hymod_core_plan::Step;
use std::fs;

pub struct MkdirHandler;

impl OpHandler for MkdirHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::Mkdir { path } = step {
            fs::create_dir_all(path).context(format!("Failed to create directory {}", path))?;
            Ok(())
        } else {
            panic!("MkdirHandler called with non-Mkdir step");
        }
    }
}
