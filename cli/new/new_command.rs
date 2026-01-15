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

    /// Mod author
    #[arg(long)]
    pub author: Option<String>,

    /// Mod version
    #[arg(long, default_value = "0.1.0")]
    // Keep explicit default in CLI help? Or better rely on logic.
    // Wait, if I set default here it will always be Some. Better to use logic default if None.
    pub version: Option<String>,

    /// Mod description
    #[arg(long)]
    pub desc: Option<String>,

    /// Enable interactive mode
    #[arg(short, long)]
    pub interactive: bool,
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
            author: self.author.clone(),
            version: self.version.clone(),
            desc: self.desc.clone(),
            interactive: self.interactive,
        };
        // Ensure skeleton.zip is found relative to the binary or embedded properly
        // include_bytes! macro looks relative to the file it's in.
        // This file is cli/new/new_command.rs.
        // Assets is at root/assets.
        // So ../../assets/skeleton.zip.
        const SKELETON_BYTES: &[u8] = include_bytes!("../../assets/skeleton.zip");
        let plan = features_new::generate_plan(args, SKELETON_BYTES);
        executor.execute(&plan)?;
        Ok(())
    }
}
