use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_deploy_dry_run() {
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

    // Add a server
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", home_dir.path())
        .args(["server", "add", "local", "prod", "/tmp/server"])
        .assert()
        .success();

    // Run deploy with dry-run
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(&project_dir)
        .env("HOME", home_dir.path())
        .args(["deploy", "prod", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PLAN"))
        .stdout(predicate::str::contains("RUN      ./gradlew build"))
        .stdout(predicate::str::contains(
            "build/libs/mod-1.0.0.jar -> /tmp/server/mods/mod-1.0.0.jar",
        ));
    // Ideally checking that it DIDN'T do something, but hard to prove negative without checking filesystem, which dry-run shouldn't touch.
    // But verifying the plan output is printed is a good start.
}
