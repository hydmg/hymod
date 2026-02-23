use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_server_add_local() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "local", "myserver", "/tmp/myserver"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Server 'myserver' added successfully",
        ));

    let config_path = temp_home
        .path()
        .join(".hymod")
        .join("servers.d")
        .join("myserver.yaml");
    assert!(config_path.exists());

    let content = fs::read_to_string(config_path).unwrap();
    assert!(content.contains("name: myserver"));
    assert!(content.contains("kind: local"));
    assert!(content.contains("server_root: /tmp/myserver"));
}

#[test]
fn test_server_add_remote() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "remote", "myremote", "user@example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Server 'myremote' added successfully",
        ));

    let config_path = temp_home
        .path()
        .join(".hymod")
        .join("servers.d")
        .join("myremote.yaml");
    assert!(config_path.exists());

    let content = fs::read_to_string(config_path).unwrap();
    assert!(content.contains("name: myremote"));
    assert!(content.contains("kind: remote"));
    assert!(content.contains("user: user"));
    assert!(content.contains("host: example.com"));
}

#[test]
fn test_server_add_remote_with_path_parses_server_root() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args([
            "server",
            "add",
            "remote",
            "hymodtest",
            "root@170.205.24.203:/root/playground/hymodtest",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Server 'hymodtest' added successfully",
        ));

    let config_path = temp_home
        .path()
        .join(".hymod")
        .join("servers.d")
        .join("hymodtest.yaml");
    assert!(config_path.exists());

    let content = fs::read_to_string(config_path).unwrap();
    assert!(content.contains("kind: remote"));
    assert!(content.contains("user: root"));
    assert!(content.contains("host: 170.205.24.203"));
    assert!(content.contains("server_root: /root/playground/hymodtest"));
}

#[test]
fn test_server_already_exists() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    // Add first time
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "local", "myserver", "/tmp/path"])
        .assert()
        .success();

    // Add second time
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "local", "myserver", "/tmp/path"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Server 'myserver' already exists"));
}

#[test]
fn test_server_list() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "local", "s1", "/tmp/s1"])
        .assert()
        .success();

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "remote", "s2", "u@h"])
        .assert()
        .success();

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("s1 [local]"))
        .stdout(predicate::str::contains("s2 [remote]"));
}

#[test]
fn test_server_default() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "local", "s1", "/tmp/s1"])
        .assert()
        .success();

    // Set default
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "default", "local", "s1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default local server set to 's1'"));

    // Check list highlight
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("* s1 [local]"));
}

#[test]
fn test_server_default_separate_local_and_remote() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "local", "local1", "/tmp/local1"])
        .assert()
        .success();

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "remote", "remote1", "user@example.com"])
        .assert()
        .success();

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "default", "local", "local1"])
        .assert()
        .success();

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "default", "remote", "remote1"])
        .assert()
        .success();

    let default_local = temp_home
        .path()
        .join(".hymod")
        .join("servers.d")
        .join("default.local");
    let default_remote = temp_home
        .path()
        .join(".hymod")
        .join("servers.d")
        .join("default.remote");

    assert_eq!(fs::read_to_string(default_local).unwrap().trim(), "local1");
    assert_eq!(fs::read_to_string(default_remote).unwrap().trim(), "remote1");
}

#[test]
fn test_server_remove() {
    let temp_home = TempDir::new().expect("failed to create temp dir");

    // Add server
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "add", "local", "removetest", "/tmp/removetest"])
        .assert()
        .success();

    let config_path = temp_home
        .path()
        .join(".hymod")
        .join("servers.d")
        .join("removetest.yaml");
    assert!(config_path.exists());

    // Remove server
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "remove", "removetest"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Server 'removetest' removed successfully",
        ));

    // Verify removed
    assert!(!config_path.exists());

    // Remove non-existent
    Command::cargo_bin("hymod")
        .expect("binary not found")
        .env("HOME", temp_home.path())
        .args(["server", "remove", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Server configuration 'nonexistent' not found",
        ));
}
