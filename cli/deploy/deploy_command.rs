use clap::Args;

#[derive(Args, Debug)]
pub struct DeployCommand {
    pub server_name: Option<String>,
    #[arg(long)]
    pub transport: Option<String>,
    #[arg(long)]
    pub dry_run: bool,
}
