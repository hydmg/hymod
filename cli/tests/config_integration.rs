//! Integration tests for the `hymod config` command.

use assert_cmd::Command;
use std::env;
use std::fs;
use tempfile::TempDir;

#[test]
fn config_set_get_and_list() {
    // Create a temporary home directory to avoid polluting the real user config.
    let temp_home = TempDir::new().expect("failed to create temp dir");
    env::set_var("HOME", temp_home.path());

    // Ensure the config file does not exist initially.
    let config_path = temp_home.path().join(".hymod").join("config.yaml");
    if config_path.exists() {
        fs::remove_file(&config_path).ok();
    }

    // Set a config value.
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .args(["config", "set", "author", "TestAuthor"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Set author = TestAuthor"));

    // Get the config value.
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .args(["config", "get", "author"])
        .assert()
        .success()
        .stdout(predicates::str::contains("TestAuthor"));

    // List all config values and ensure the author appears.
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .args(["config", "list"])
        .assert()
        .success()
        .stdout(predicates::str::contains("author: TestAuthor"));
}
