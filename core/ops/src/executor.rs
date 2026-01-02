use anyhow::{bail, Result};
use hymod_core_plan::{Plan, Step};
use std::collections::HashMap;
use std::mem::discriminant;
use std::mem::Discriminant;

use crate::handlers;
use crate::traits::OpHandler;

pub struct Executor {
    dry_run: bool,
    handlers: HashMap<Discriminant<Step>, Box<dyn OpHandler>>,
}

impl Executor {
    pub fn new(dry_run: bool) -> Self {
        let mut executor = Self {
            dry_run,
            handlers: HashMap::new(),
        };
        executor.register_defaults();
        executor
    }

    fn register_defaults(&mut self) {
        // Run
        self.register(
            Step::RunProcess {
                cmd: String::new(),
                args: vec![],
                cwd: None,
            },
            Box::new(handlers::run_process::RunProcessHandler),
        );

        // FS
        self.register(
            Step::Mkdir {
                path: String::new(),
            },
            Box::new(handlers::fs::mkdir::MkdirHandler),
        );
        self.register(
            Step::WriteFile {
                path: String::new(),
                content: String::new(),
            },
            Box::new(handlers::fs::write_file::WriteFileHandler),
        );
        self.register(
            Step::CopyFile {
                from: String::new(),
                to: String::new(),
            },
            Box::new(handlers::fs::copy_file::CopyFileHandler),
        );
        self.register(
            Step::Symlink {
                from: String::new(),
                to: String::new(),
            },
            Box::new(handlers::fs::symlink::SymlinkHandler),
        );

        // Net
        self.register(
            Step::UploadRsync {
                local: String::new(),
                remote: String::new(),
                opts: String::new(),
            },
            Box::new(handlers::net::upload_rsync::UploadRsyncHandler),
        );
        self.register(
            Step::UploadScp {
                local: String::new(),
                remote: String::new(),
            },
            Box::new(handlers::net::upload_scp::UploadScpHandler),
        );
        self.register(
            Step::SshRun {
                host: String::new(),
                user: String::new(),
                cmd: String::new(),
            },
            Box::new(handlers::net::ssh_run::SshRunHandler),
        );

        // Check
        self.register(
            Step::CheckFile {
                path: String::new(),
            },
            Box::new(handlers::check::check_file::CheckFileHandler),
        );
        self.register(
            Step::CheckZip {
                path: String::new(),
            },
            Box::new(handlers::check::check_zip::CheckZipHandler),
        );
    }

    fn register(&mut self, step_variant: Step, handler: Box<dyn OpHandler>) {
        self.handlers.insert(discriminant(&step_variant), handler);
    }

    pub fn execute(&self, plan: &Plan) -> Result<()> {
        if self.dry_run {
            println!("PLAN");
            for (i, step) in plan.steps.iter().enumerate() {
                println!("  {}. {}", i + 1, step);
            }
            return Ok(());
        }

        for step in &plan.steps {
            self.execute_step(step)?;
        }
        Ok(())
    }

    fn execute_step(&self, step: &Step) -> Result<()> {
        let key = discriminant(step);
        if let Some(handler) = self.handlers.get(&key) {
            handler.handle(step)
        } else {
            bail!("No handler registered for step: {:?}", step);
        }
    }
}
