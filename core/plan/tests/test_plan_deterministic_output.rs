use core_plan::{Plan, Step};

#[test]
fn test_plan_deterministic_output() {
    let plan = Plan {
        steps: vec![
            Step::RunProcess {
                cmd: "echo".to_string(),
                args: vec!["hello".to_string()],
                cwd: Some("/repo".to_string()),
            },
            Step::CheckFile {
                path: "/path/to/file".to_string(),
            },
        ],
    };

    // The Display implementation (for dry-run) is what we care about being stable/deterministic here
    let mut plan_string = String::new();
    for step in &plan.steps {
        use std::fmt::Write;
        writeln!(&mut plan_string, "{}", step).unwrap();
    }

    // We expect the exact format from our Display impl
    let expected = r#"RUN      echo hello    (cwd: /repo)
CHECK     /path/to/file
"#;

    assert_eq!(plan_string, expected);
}
