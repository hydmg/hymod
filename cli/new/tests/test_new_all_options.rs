use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_new_all_options() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let temp_path = temp_dir.path().to_str().unwrap();

    let project_name = "test_overrides";
    let project_path = temp_dir.path().join(project_name);

    Command::cargo_bin("hymod")
        .expect("binary not found")
        .current_dir(temp_path)
        .args([
            "new",
            project_name,
            "--group",
            "com.override",
            "--package",
            "com.override.pkg",
            "--author",
            "OverrideAuthor",
            "--version",
            "1.2.3",
            "--desc",
            "OverrideDesc",
        ])
        .assert()
        .success();

    // Verify hymod.yaml content (if generated, or specific files)
    // Checking manifest.json as per skeleton
    let manifest_path = project_path.join("src/main/resources/manifest.json");
    let manifest_content = fs::read_to_string(&manifest_path).expect("failed to read manifest");

    assert!(manifest_content.contains("\"Name\": \"OverrideAuthor\"")); // Author maps to Name in skeleton currently?
                                                                        // Wait, skeleton map: <AUTHOR> -> Name in Authors array
                                                                        // "Authors": [ { "Name": "<AUTHOR>" } ]

    // Check main class
    let main_java = project_path.join("src/main/java/com/override/pkg/Main.java");
    assert!(main_java.exists());
    let main_content = fs::read_to_string(&main_java).expect("failed to read main.java");
    assert!(main_content.contains("package com.override.pkg;"));

    // Check version? Skeleton has <VERSION> mapped to "Version"
    assert!(manifest_content.contains("\"Version\": \"1.2.3\""));

    // Check desc? Skeleton <DESCRIPTION> mapped to "Description"
    assert!(manifest_content.contains("\"Description\": \"OverrideDesc\""));
}
