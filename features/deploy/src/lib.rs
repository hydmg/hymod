pub struct DeployArgs {
    pub server_name: Option<String>,
    pub transport: Option<String>, // "rsync" | "scp"
    pub dry_run: bool,
}

pub fn generate_plan(args: DeployArgs) -> core_plan::Plan {
    todo!()
}
