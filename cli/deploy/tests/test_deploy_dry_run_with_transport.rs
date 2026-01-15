use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_deploy_dry_run_with_transport() {
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

    // Add a REMOTE server
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", home_dir.path())
        .args(["server", "add", "remote", "prod", "user@example.com"])
        .assert()
        .success();

    // Run deploy with dry-run and transport rsync
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(&project_dir)
        .env("HOME", home_dir.path())
        .args(["deploy", "prod", "--transport", "rsync", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PLAN"))
        .stdout(predicate::str::contains("RUN      ./gradlew build"))
        .stdout(predicate::str::contains(
            "UPLOAD    build/libs/mod-1.0.0.jar -> mods/mod-1.0.0.jar",
        ));
}
