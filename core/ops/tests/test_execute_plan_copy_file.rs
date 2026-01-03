use core_ops::Executor;
use hymod_core_plan::{Plan, Step};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_execute_plan_copy_file() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("source.txt");
    let dest = dir.path().join("dest.txt");

    fs::write(&src, "hello world").unwrap();

    let plan = Plan {
        steps: vec![Step::CopyFile {
            from: src.to_str().unwrap().to_string(),
            to: dest.to_str().unwrap().to_string(),
        }],
    };

    let executor = Executor::new(false);
    executor.execute(&plan).unwrap();

    assert_eq!(fs::read_to_string(dest).unwrap(), "hello world");
}
