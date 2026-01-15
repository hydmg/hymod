use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_dev_runs_build() {
    let temp_dir = tempfile::tempdir().expect("failed to create temp folder");
    let path = temp_dir.path().to_owned();

    // 1. Setup mock gradlew
    let gradlew_path = path.join("gradlew");
    let script_content = r#"#!/bin/bash
mkdir -p build/libs
touch build/libs/test-mod-1.0.jar
echo "Mock Gradle Build Success"
"#;
    fs::write(&gradlew_path, script_content).expect("failed to write mock gradlew");

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&gradlew_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&gradlew_path, perms).unwrap();
    }

    // 2. Run hymod dev
    // We use `cargo run --bin hymod -- dev <path> --path <path>` (deploy to self)
    let status = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hymod",
            "--",
            "dev",
            path.to_str().unwrap(),
            "--path",
            path.to_str().unwrap(),
        ])
        .current_dir(std::env::current_dir().unwrap()) // Ensure we run from repo root
        .output()
        .expect("failed to execute hymod dev");

    // 3. Assertions
    let stdout = String::from_utf8_lossy(&status.stdout);
    let stderr = String::from_utf8_lossy(&status.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(status.status.success(), "hymod dev failed");
    assert!(
        stdout.contains("Mock Gradle Build Success"),
        "Did not run gradle wrapper"
    );
    assert!(
        stdout.contains("Deployed successfully"),
        "Did not report successful deployment"
    );
    assert!(
        stdout.contains("test-mod-1.0.jar"),
        "Did not find correct artifact"
    );
}
