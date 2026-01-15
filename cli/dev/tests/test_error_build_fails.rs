use std::fs;
use std::process::Command;

#[test]
fn test_error_build_fails() {
    let temp_dir = tempfile::tempdir().expect("failed to create temp folder");
    let path = temp_dir.path().to_owned();

    // 1. Setup mock gradlew that FAILS
    let gradlew_path = path.join("gradlew");
    let script_content = r#"#!/bin/bash
echo "Simulating Compilation Error"
exit 1
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
    // 2. Run hymod dev
    let status = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hymod",
            "--",
            "dev",
            "--path",
            path.to_str().unwrap(),
        ])
        .current_dir(std::env::current_dir().unwrap())
        .output()
        .expect("failed to execute hymod dev");

    // 3. Assertions
    let stdout = String::from_utf8_lossy(&status.stdout);
    let stderr = String::from_utf8_lossy(&status.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(!status.status.success(), "hymod dev should have failed");
    assert!(
        stderr.contains("Gradle build failed") || stdout.contains("Gradle build failed"),
        "Did not report build failure correctly"
    );
}
