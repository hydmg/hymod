#[test]
fn test_deploy_default_rsync() {
    panic!("Plan must contain UploadRsync(cache -> user@host:/mods/mod.zip) and SshRun(systemctl restart).");
}
