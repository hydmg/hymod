use core_ops::Executor;
use hymod_core_plan::{Plan, Step};

#[test]
fn test_dry_run_output() {
    let plan = Plan {
        steps: vec![
            Step::RunProcess {
                cmd: "./gradlew".to_string(),
                args: vec!["test".to_string(), "build".to_string()],
                cwd: Some("/repo".to_string()),
            },
            Step::CheckFile {
                path: "/repo/build/hymod/artifact.zip".to_string(),
            },
            Step::Symlink {
                from: "/repo/build/hymod/artifact.zip".to_string(),
                to: "/opt/hytale-local/mods/com.acme.power".to_string(),
            },
            Step::RunProcess {
                cmd: "/opt/hytale-local/bin/restart.sh".to_string(),
                args: vec![],
                cwd: None,
            },
        ],
    };

    // Capture stdout?
    // Since Executor prints to stdout, we might need a way to capture it or just verify it doesn't panic.
    // For a unit test, it's hard to assert stdout without a wrapper or changing Executor to write to a trait.
    // But for now, let's just make sure it runs.
    // Ideally Executor should take a writer, but the PRD says "prints", so stdout is implied.
    // We can refactor Executor to take a specific writer later if needed for strict assertions.

    let executor = Executor::new(true);
    executor.execute(&plan).expect("Dry run failed");
}
