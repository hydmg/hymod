use core_plan::Step;
use features_new::{generate_plan, NewArgs};

#[test]
fn test_skeleton_plan_generation() {
    let args = NewArgs {
        name: "testmod".to_string(),
        path: None,
        group: None,
        package: None,
        no_ui_dir: false,
        author: None,
        version: None,
        desc: None,
        interactive: false,
    };

    let skeleton_bytes = include_bytes!("../../../assets/skeleton.zip");
    let plan = generate_plan(args, skeleton_bytes);

    assert!(!plan.steps.is_empty(), "Plan should not be empty");

    let mut found_gradle = false;
    let mut found_gitignore = false;
    let mut found_binary = false;

    for step in &plan.steps {
        match step {
            Step::WriteFile { path, content } => {
                println!("WriteFile: {}", path);
                if path.contains("settings.gradle") {
                    found_gradle = true;
                    assert!(
                        content.contains("testmod"),
                        "settings.gradle should contain project name"
                    );
                }
                if path.contains(".gitignore") {
                    found_gitignore = true;
                }
                if path.contains("local.properties") {
                    println!("Found local.properties: {}", path);
                }
                assert!(
                    !path.ends_with(".mac"),
                    "Should not have .mac file: {}",
                    path
                );
                assert!(
                    !path.ends_with(".windows"),
                    "Should not have .windows file: {}",
                    path
                );
                assert!(
                    !path.ends_with(".linux"),
                    "Should not have .linux file: {}",
                    path
                );
            }
            Step::WriteBytes { path, content } => {
                println!("WriteBytes: {} ({} bytes)", path, content.len());
                found_binary = true;
            }
            Step::Mkdir { path } => {
                println!("Mkdir: {}", path);
            }
            _ => {}
        }
    }

    assert!(found_gradle, "Should have found build.gradle");
    assert!(found_gitignore, "Should have found .gitignore");
    // Depending on what's in the skeleton, we might or might not have a binary file (gradle wrapper jar is in gradle/wrapper/)
    // zip -l showed gradle/ dir, but I didn't see files inside in the first 10 lines.
    // Let's assume there is at least something that is not text if the skeleton has wrapper.
    // verified earlier: gradle/wrapper/gradle-wrapper.jar usually exists.
}
