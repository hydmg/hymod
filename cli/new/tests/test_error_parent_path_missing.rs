#[ignore]
#[test]
fn test_error_parent_path_missing() {
    panic!("Execute: hymod new my-mod --path /nonexistent/parent/dir, verify error 'Parent directory does not exist'.");
}
