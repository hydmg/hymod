#[test]
fn test_error_invalid_transport() {
    panic!("Execute: hymod deploy --server prod --transport ftp, verify error 'Invalid transport: must be rsync or scp'.");
}
