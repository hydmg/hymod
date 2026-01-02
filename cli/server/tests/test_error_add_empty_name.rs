#[test]
fn test_error_add_empty_name() {
    panic!("Execute: hymod server add \"\", verify error 'Invalid server name: cannot be empty'.");
}
