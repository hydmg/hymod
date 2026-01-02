use core_ops::Executor;
use hymod_core_plan::{Plan, Step};
use tempfile::tempdir;

#[test]
fn test_execute_plan_copy_missing_source() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("missing.txt");
    let dest = dir.path().join("dest.txt");

    let plan = Plan {
        steps: vec![Step::CopyFile {
            from: src.to_str().unwrap().to_string(),
            to: dest.to_str().unwrap().to_string(),
        }],
    };

    let executor = Executor::new(false);
    let result = executor.execute(&plan);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to copy"));
}

#[test]
#[cfg(unix)]
fn test_execute_plan_symlink_missing_source_allowed() {
    // Symlinks to non-existent targets are valid in Unix (broken links).
    // Hymod might want to allow this or fail?
    // PRD says "symlink the local server’s mods folder directly to the build artifact"
    // Usually we want the link to exist.
    // `ln -s` succeeds even if target is missing.
    // Let's verify our implementation allows it (or changes strategy if we want strictness).
    // Our impl uses `std::os::unix::fs::symlink` which allows broken links.

    let dir = tempdir().unwrap();
    let target = dir.path().join("missing_target.txt");
    let link = dir.path().join("link_to_missing.txt");

    let plan = Plan {
        steps: vec![Step::Symlink {
            from: target.to_str().unwrap().to_string(),
            to: link.to_str().unwrap().to_string(),
        }],
    };

    let executor = Executor::new(false);
    let result = executor.execute(&plan);

    assert!(result.is_ok());
    // If strictness is required later, we'd add a CheckFile step before this in the Planner.
}
