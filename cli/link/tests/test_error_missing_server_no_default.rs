#[test]
fn test_error_missing_server_no_default() {
    panic!("Execute: hymod link (no --server, no default), verify error 'No server specified and no default configured'.");
}
