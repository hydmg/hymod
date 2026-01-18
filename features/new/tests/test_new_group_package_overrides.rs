use core_plan::Step;
use features_new::{generate_plan, NewArgs};
use std::io::Write;
use zip::write::FileOptions;

#[test]
fn test_new_group_package_overrides() {
    // 1. Create a mock skeleton zip
    let mut zip_buffer = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        // Add a file in the default package location
        // NOTE: The skeleton.rs logic strips the first component (root folder).
        // So we must put our files inside a root folder.
        zip.start_file("root/src/main/java/com/example/skeleton/Main.java", options)
            .unwrap();
        // Include the target string that should be replaced
        zip.write_all(b"package com.example.skeleton;\n\npublic class Main {}")
            .unwrap();

        // Add another file for control
        zip.start_file("root/README.md", options).unwrap();
        zip.write_all(b"# <MOD_NAME>").unwrap();

        zip.finish().unwrap();
    }

    // 2. Setup arguments with overrides
    // User wants: hymod new superping -> com.group.superping (if group is set)
    // We'll simulate group being passed or config default.
    let args = NewArgs {
        name: "superping".to_string(),
        path: None,
        group: Some("com.mygroup".to_string()),
        package: None, // Should derive to com.mygroup.superping
        no_ui_dir: false,
        author: None,
        version: None,
        desc: None,
        interactive: false,
    };

    // 3. Generate Plan
    let plan = generate_plan(args, &zip_buffer);

    // 4. Verify
    let mut found_java = false;
    let mut found_readme = false;

    for step in plan.steps {
        if let Step::WriteFile { path, content } = step {
            // Check correct directory
            if path.contains("src/main/java/com/mygroup/superping/Main.java") {
                found_java = true;
                // Check correct content replacement
                assert!(
                    content.contains("package com.mygroup.superping;"),
                    "Content package not replaced: {}",
                    content
                );
            }
            if path.contains("README.md") {
                found_readme = true;
                assert!(content.contains("# superping"));
            }
        }
    }

    assert!(found_java, "Did not find Main.java in the expected path: src/main/java/com/mygroup/superping/Main.java");
    assert!(found_readme, "Did not find README.md");
}
