#[test]
fn test_error_artifact_dir_not_writable() {
    panic!("Execute: hymod build (with read-only build directory), verify error 'Cannot create build directory: permission denied'.");
}
