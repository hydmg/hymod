#[ignore]
#[test]
fn test_error_path_is_file() {
    panic!("Execute: hymod new my-mod --path <existing_file>, verify error 'Path exists and is not a directory'.");
}
