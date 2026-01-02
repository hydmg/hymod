#[test]
fn test_server_yaml_missing_ssh_host() {
    panic!("Must return Error when kind=ssh but ssh.host or ssh.user is missing.");
}
