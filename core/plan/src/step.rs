#[derive(Debug, PartialEq)]
pub enum Step {
    RunProcess {
        cmd: String,
        args: Vec<String>,
        cwd: Option<String>,
    },
    WriteFile {
        path: String,
        content: String,
    }, // Content simplified for skeleton
    WriteBytes {
        path: String,
        content: Vec<u8>,
    },
    Mkdir {
        path: String,
    },
    CopyFile {
        from: String,
        to: String,
    },
    Symlink {
        from: String,
        to: String,
    },
    UploadRsync {
        local: String,
        remote: String,
        opts: String,
    },
    UploadScp {
        local: String,
        remote: String,
    },
    SshRun {
        host: String,
        user: String,
        cmd: String,
    },
    CheckFile {
        path: String,
    },
    CheckZip {
        path: String,
    },
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::RunProcess { cmd, args, cwd } => {
                let args_str = args.join(" ");
                let cwd_str = cwd
                    .as_ref()
                    .map(|c| format!("(cwd: {})", c))
                    .unwrap_or_default();
                write!(f, "RUN      {} {}    {}", cmd, args_str, cwd_str)
            }
            Step::WriteFile { path, .. } => write!(f, "WRITE     {}", path),
            Step::WriteBytes { path, .. } => write!(f, "WRITE(B)  {}", path),
            Step::Mkdir { path } => write!(f, "MKDIR     {}", path),
            Step::CopyFile { from, to } => write!(f, "COPY      {} -> {}", from, to),
            Step::Symlink { from, to } => write!(f, "SYMLINK   {} -> {}", to, from), // "link -> target" visual flow
            Step::UploadRsync {
                local,
                remote,
                opts: _,
            } => write!(f, "UPLOAD(RSYNC) {} -> {}", local, remote),
            Step::UploadScp { local, remote } => write!(f, "UPLOAD(SCP)   {} -> {}", local, remote),
            Step::SshRun {
                host: _,
                user: _,
                cmd,
            } => write!(f, "RUN       {}", cmd), // Simplified for dry-run visibility
            Step::CheckFile { path } => write!(f, "CHECK     {}", path),
            Step::CheckZip { path } => write!(f, "CHECK     {}", path),
        }
    }
}
