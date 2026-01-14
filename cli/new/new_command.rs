use clap::Args;

#[derive(Args, Debug)]
pub struct NewCommand {
    /// Name of the new project
    pub name: String,

    /// Optional path to create the project in
    #[arg(long)]
    pub path: Option<String>,

    /// Project group ID
    #[arg(long)]
    pub group: Option<String>,

    /// Project package
    #[arg(long)]
    pub package: Option<String>,

    /// Skip UI directory generation
    #[arg(long)]
    pub no_ui_dir: bool,
}
