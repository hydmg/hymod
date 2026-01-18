use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_deploy_force_scp() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let home_dir = TempDir::new().expect("failed to create home dir");

    // Create a new project
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(temp_dir.path())
        .env("HOME", home_dir.path())
        .args(["new", "testmod"])
        .assert()
        .success();

    let project_dir = temp_dir.path().join("testmod");

    // Add a REMOTE server (dummy)
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", home_dir.path())
        .args(["server", "add", "remote", "prod", "user@host:/path"])
        .assert()
        .success();

    // Run deploy (default transport should be SCP)
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(&project_dir)
        .env("HOME", home_dir.path())
        .args(["deploy", "prod", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("UPLOAD(SCP)"));
}
