#[test]
fn test_error_restart_fails() {
    panic!("Execute: hymod deploy --server prod (with bad restart cmd), verify error 'Restart failed'.");
}
