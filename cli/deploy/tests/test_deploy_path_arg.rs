use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_deploy_path_arg() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let home_dir = TempDir::new().expect("failed to create home dir");

    // Create a new project "testmod"
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(temp_dir.path())
        .env("HOME", home_dir.path())
        .args(["new", "testmod"])
        .assert()
        .success();

    let project_dir = temp_dir.path().join("testmod");

    // Create another directory "outside"
    let outside_dir = temp_dir.path().join("outside");
    std::fs::create_dir(&outside_dir).unwrap();

    // Add a server
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", home_dir.path())
        .args(["server", "add", "local", "prod", "/tmp/server"])
        .assert()
        .success();

    // Run deploy FROM OUTSIDE, pointing to project_dir via --path
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(&outside_dir) // cwd is outside
        .env("HOME", home_dir.path())
        .args([
            "deploy",
            "prod",
            "--path",
            project_dir.to_str().unwrap(),
            "--dry-run",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("PLAN"))
        .stdout(predicate::str::contains("RUN      ./gradlew build"))
        .stdout(predicate::str::contains("mod-1.0.0.jar")); // Artifact identification should work
}
