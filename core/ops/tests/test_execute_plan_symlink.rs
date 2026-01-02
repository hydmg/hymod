use core_ops::Executor;
use hymod_core_plan::{Plan, Step};
use std::fs;
use tempfile::tempdir;

#[test]
#[cfg(unix)]
fn test_execute_plan_symlink() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("target.txt");
    let link = dir.path().join("link.txt");

    fs::write(&target, "content").unwrap();

    let plan = Plan {
        steps: vec![Step::Symlink {
            from: target.to_str().unwrap().to_string(),
            to: link.to_str().unwrap().to_string(),
        }],
    };

    let executor = Executor::new(false);
    executor.execute(&plan).unwrap();

    assert!(link.exists());
    assert!(fs::symlink_metadata(&link)
        .unwrap()
        .file_type()
        .is_symlink());
    assert_eq!(fs::read_to_string(link).unwrap(), "content");
}
