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

use crate::command::CliCommand;
use anyhow::Result;
use core_ops::Executor;

impl CliCommand for NewCommand {
    fn run(&self, executor: &Executor) -> Result<()> {
        let args = features_new::NewArgs {
            name: self.name.clone(),
            path: self.path.clone(),
            group: self.group.clone(),
            package: self.package.clone(),
            no_ui_dir: self.no_ui_dir,
        };
        const SKELETON_BYTES: &[u8] = include_bytes!("../../assets/skeleton.zip");
        let plan = features_new::generate_plan(args, SKELETON_BYTES);
        executor.execute(&plan)?;
        Ok(())
    }
}
