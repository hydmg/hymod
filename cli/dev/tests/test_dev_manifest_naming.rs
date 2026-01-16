use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_dev_manifest_naming() {
    let temp = TempDir::new().unwrap();
    let mod_dir = temp.path().join("my-mod");
    fs::create_dir(&mod_dir).unwrap();

    // 1. Setup manifest.json
    let resources_dir = mod_dir.join("src/main/resources");
    fs::create_dir_all(&resources_dir).unwrap();
    fs::write(
        resources_dir.join("manifest.json"),
        r#"{
            "Group": "hydmg",
            "Name": "custom-mod-name",
            "Version": "1.2.3",
            "Description": "A test mod",
            "Authors": [{"Name": "tester"}],
            "Main": "hydmg.test.Main"
        }"#,
    )
    .unwrap();

    // 2. Setup mock gradlew
    let gradlew_path = mod_dir.join("gradlew");
    let build_libs = mod_dir.join("build/libs");
    fs::create_dir_all(&build_libs).unwrap();

    // Create a dummy build artifact
    fs::write(
        build_libs.join("original-build-artifact.jar"),
        "dummy content",
    )
    .unwrap();

    // Create a mock gradlew script that succeeds
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::write(&gradlew_path, "#!/bin/sh\nexit 0").unwrap();
        let mut perms = fs::metadata(&gradlew_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&gradlew_path, perms).unwrap();
    }
    #[cfg(windows)]
    {
        fs::write(&gradlew_path, "@echo off\nexit 0").unwrap();
    }

    // 3. Setup target directory (mock server)
    let target_dir = temp.path().join("target_server/mods");
    fs::create_dir_all(&target_dir).unwrap();

    // 4. Run hymod dev
    let mut cmd = Command::cargo_bin("hymod").unwrap();
    cmd.current_dir(&mod_dir)
        .arg("dev")
        .arg(target_dir.parent().unwrap()); // Pass server root (parent of mods) as target

    cmd.assert().success();

    // 5. Verify the file in target_dir is named "custom-mod-name-1.2.3.jar"
    let expected_jar = target_dir.join("custom-mod-name-1.2.3.jar");
    assert!(
        expected_jar.exists(),
        "Expected jar not found: {}. Found: {:?}",
        expected_jar.display(),
        fs::read_dir(&target_dir)
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect::<Vec<_>>()
    );
}
