use core_ops::Executor;
use hymod_core_plan::{Plan, Step};

#[test]
fn test_execute_plan_run_process() {
    let plan = Plan {
        steps: vec![Step::RunProcess {
            cmd: "echo".to_string(),
            args: vec!["hello".to_string()],
            cwd: None,
        }],
    };

    let executor = Executor::new(false);
    // This should succeed if echo is in path
    executor.execute(&plan).unwrap();
}
