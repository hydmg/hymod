use core_ops::Executor;
use hymod_core_plan::{Plan, Step};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_execute_plan_write_file() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("output.txt");

    let plan = Plan {
        steps: vec![Step::WriteFile {
            path: file.to_str().unwrap().to_string(),
            content: "payload".to_string(),
        }],
    };

    let executor = Executor::new(false);
    executor.execute(&plan).unwrap();

    assert_eq!(fs::read_to_string(file).unwrap(), "payload");
}
