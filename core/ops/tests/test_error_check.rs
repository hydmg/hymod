use core_ops::Executor;
use hymod_core_plan::{Plan, Step};

#[test]
fn test_execute_plan_check_failure() {
    let plan = Plan {
        steps: vec![Step::CheckFile {
            path: "/non/existent/path/should/fail".to_string(),
        }],
    };

    let executor = Executor::new(false);
    let result = executor.execute(&plan);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "File not found: /non/existent/path/should/fail"
    );
}

#[test]
fn test_execute_plan_check_zip_failure() {
    let plan = Plan {
        steps: vec![Step::CheckZip {
            path: "/non/existent/artifact.zip".to_string(),
        }],
    };

    let executor = Executor::new(false);
    let result = executor.execute(&plan);

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Zip artifact not found"));
}
