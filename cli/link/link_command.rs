use clap::Args;

#[derive(Args, Debug)]
pub struct LinkCommand {
    pub server_name: Option<String>,
}
