use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[cfg(unix)]
fn make_fake_rsync(bin_dir: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let rsync_path = bin_dir.join("rsync");
    fs::write(&rsync_path, "#!/usr/bin/env sh\nexit 0\n").expect("failed to write fake rsync");
    let mut perms = fs::metadata(&rsync_path)
        .expect("failed to stat fake rsync")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&rsync_path, perms).expect("failed to chmod fake rsync");
}

#[cfg(unix)]
#[test]
fn test_transport_default_rsync_available() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let home_dir = TempDir::new().expect("failed to create home dir");
    let bin_dir = TempDir::new().expect("failed to create bin dir");

    make_fake_rsync(bin_dir.path());

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(temp_dir.path())
        .env("HOME", home_dir.path())
        .args(["new", "testmod"])
        .assert()
        .success();

    let project_dir = temp_dir.path().join("testmod");

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", home_dir.path())
        .args(["server", "add", "remote", "prod", "user@host:/path"])
        .assert()
        .success();

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(&project_dir)
        .env("HOME", home_dir.path())
        .env("PATH", bin_dir.path())
        .args(["deploy", "prod", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("UPLOAD(RSYNC)"));
}
