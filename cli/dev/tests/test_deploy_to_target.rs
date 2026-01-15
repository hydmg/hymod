use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_dev_deploy_to_target() {
    // 1. Setup temp workspace
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let mod_dir = root.join("my-mod");
    let server_dir = root.join("my-server");

    fs::create_dir(&mod_dir).unwrap();
    fs::create_dir_all(server_dir.join("mods")).unwrap();

    // 2. Create mock gradlew
    let gradlew_path = mod_dir.join("gradlew");
    let gradlew_content = r#"#!/bin/sh
mkdir -p build/libs
echo "fake jar content" > build/libs/my-mod-1.0.jar
exit 0
"#;
    fs::write(&gradlew_path, gradlew_content).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&gradlew_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&gradlew_path, perms).unwrap();
    }

    // 3. Run hymod dev with target and path
    // Target is absolute path to server_dir
    let target_str = server_dir.to_str().unwrap();
    let path_str = mod_dir.to_str().unwrap();

    let mut cmd = Command::cargo_bin("hymod").unwrap();
    cmd.arg("dev").arg(target_str).arg("--path").arg(path_str);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Deployed successfully"))
        .stdout(predicate::str::contains(
            server_dir.join("mods").to_str().unwrap(),
        ));

    // 4. Verify artifact exists in target
    let installed_jar = server_dir.join("mods").join("my-mod-1.0.jar");
    assert!(installed_jar.exists(), "Jar was not copied to target");
}
