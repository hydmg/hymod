#[derive(Debug, PartialEq)]
pub enum Step {
    RunProcess { cmd: String, args: Vec<String>, cwd: Option<String> },
    WriteFile { path: String, content: String }, // Content simplified for skeleton
    Mkdir { path: String },
    CopyFile { from: String, to: String },
    Symlink { from: String, to: String },
    UploadRsync { local: String, remote: String, opts: String },
    UploadScp { local: String, remote: String },
    SshRun { host: String, user: String, cmd: String },
}

#[derive(Debug)]
pub struct Plan {
    pub steps: Vec<Step>,
}
