use clap::Args;

#[derive(Args, Debug)]
pub struct BuildCommand {
    #[arg(long)]
    pub release: bool,
}
