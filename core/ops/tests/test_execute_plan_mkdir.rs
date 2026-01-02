use core_ops::Executor;
use hymod_core_plan::{Plan, Step};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_execute_plan_mkdir() {
    let dir = tempdir().unwrap();
    let new_dir = dir.path().join("subdir");

    let plan = Plan {
        steps: vec![Step::Mkdir {
            path: new_dir.to_str().unwrap().to_string(),
        }],
    };

    let executor = Executor::new(false);
    executor.execute(&plan).unwrap();

    assert!(new_dir.exists());
    assert!(fs::metadata(new_dir).unwrap().is_dir());
}
