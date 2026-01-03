use core_ops::Executor;
use hymod_core_plan::{Plan, Step};

#[test]
fn test_execute_plan_run_process_failure() {
    let plan = Plan {
        steps: vec![Step::RunProcess {
            cmd: "non_existent_command_12345".to_string(),
            args: vec![],
            cwd: None,
        }],
    };

    let executor = Executor::new(false);
    let result = executor.execute(&plan);

    assert!(result.is_err());
    // The error message depends on OS but usually mentions 'No such file or directory' or 'not found'
}

#[test]
fn test_execute_plan_run_process_exit_code_failure() {
    let plan = Plan {
        steps: vec![Step::RunProcess {
            cmd: "false".to_string(), // 'false' returns exit code 1
            args: vec![],
            cwd: None,
        }],
    };

    let executor = Executor::new(false);
    let result = executor.execute(&plan);

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Command failed: false"));
}
