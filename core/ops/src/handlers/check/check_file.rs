use crate::traits::OpHandler;
use anyhow::{bail, Result};
use hymod_core_plan::Step;
use std::path::Path;

pub struct CheckFileHandler;

impl OpHandler for CheckFileHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::CheckFile { path } = step {
            if !Path::new(path).exists() {
                bail!("File not found: {}", path);
            }
            Ok(())
        } else {
            panic!("CheckFileHandler called with non-CheckFile step");
        }
    }
}
