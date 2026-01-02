#[test]
fn test_error_server_is_local() {
    panic!(
        "Execute: hymod deploy --server local-dev, verify error 'Cannot deploy to local server'."
    );
}
