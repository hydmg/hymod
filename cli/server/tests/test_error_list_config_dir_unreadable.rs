#[test]
fn test_error_list_config_dir_unreadable() {
    panic!("Execute: hymod server list (unreadable ~/.hymod/servers.d), verify error 'Cannot read servers directory'.");
}
