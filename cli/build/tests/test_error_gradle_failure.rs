#[ignore]
#[test]
fn test_error_gradle_failure() {
    panic!(
        "Execute: hymod build (with broken Java code), verify error 'Build failed: gradle error'."
    );
}
