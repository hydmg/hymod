#[test]
fn test_dev_no_default_server_error() {
    panic!("Must return Error: 'No default server configured, please specify --server <name>' when no default.");
}
