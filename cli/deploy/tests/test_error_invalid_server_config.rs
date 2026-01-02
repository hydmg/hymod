#[test]
fn test_error_invalid_server_config() {
    panic!(
        "Execute: hymod deploy --server broken, verify error 'Invalid server config: parse error'."
    );
}
