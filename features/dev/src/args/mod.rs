use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DevArgs {
    pub target: Option<String>,
    pub path: Option<PathBuf>,
}
