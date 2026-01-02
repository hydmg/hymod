pub struct NewArgs {
    pub name: String,
    pub path: Option<String>,
    pub group: Option<String>,
    pub package: Option<String>,
    pub no_ui_dir: bool,
}

pub fn generate_plan(args: NewArgs) -> core_plan::Plan {
    todo!()
}
