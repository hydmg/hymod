pub struct DeployArgs {
    pub server_name: Option<String>,
    pub transport: Option<String>,
    pub dry_run: bool,
}

pub fn generate_plan(args: DeployArgs) -> core_plan::Plan {
    let _ = args;
    todo!()
}
