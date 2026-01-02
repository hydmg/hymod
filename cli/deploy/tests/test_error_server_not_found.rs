#[test]
fn test_error_server_not_found() {
    panic!(
        "Execute: hymod deploy --server nonexistent, verify error 'Server nonexistent not found'."
    );
}
