#[test]
fn test_error_missing_restart_cmd() {
    panic!(
        "Execute: hymod dev --server no-restart, verify error 'Server config missing restart_cmd'."
    );
}
