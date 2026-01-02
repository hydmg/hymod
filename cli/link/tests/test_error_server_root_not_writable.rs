#[test]
fn test_error_server_root_not_writable() {
    panic!("Execute: hymod link --server local-dev (read-only server_root), verify error 'Cannot create symlink: permission denied'.");
}
