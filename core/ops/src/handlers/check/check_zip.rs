use crate::traits::OpHandler;
use anyhow::{bail, Result};
use hymod_core_plan::Step;
use std::path::Path;

pub struct CheckZipHandler;

impl OpHandler for CheckZipHandler {
    fn handle(&self, step: &Step) -> Result<()> {
        if let Step::CheckZip { path } = step {
            if !Path::new(path).exists() {
                bail!("Zip artifact not found: {}", path);
            }
            Ok(())
        } else {
            panic!("CheckZipHandler called with non-CheckZip step");
        }
    }
}
