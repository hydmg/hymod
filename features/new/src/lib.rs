pub mod skeleton;
pub mod templates;

pub struct NewArgs {
    pub name: String,
    pub path: Option<String>,
    pub group: Option<String>,
    pub package: Option<String>,
    pub no_ui_dir: bool,
}

pub fn generate_plan(args: NewArgs, skeleton_bytes: &[u8]) -> core_plan::Plan {
    let steps = skeleton::generate_plan(&args.name, skeleton_bytes);
    core_plan::Plan { steps }
}
