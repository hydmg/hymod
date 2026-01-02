#[test]
fn test_error_empty_name() {
    panic!("Execute: hymod new \"\", verify error 'Invalid mod name: cannot be empty'.");
}
