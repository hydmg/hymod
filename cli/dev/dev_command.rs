use clap::Args;

#[derive(Args, Debug)]
pub struct DevCommand {
    pub server_name: Option<String>,
    #[arg(long)]
    pub watch: bool,
    #[arg(long)]
    pub restart_cmd: Option<String>,
}
