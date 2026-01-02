// Error Message Quality Tests (PRD §13)

#[test]
fn test_error_server_not_found() {
    panic!("Error must include: server name, 'not found in ~/.hymod/servers.d/', suggestion 'run: hymod server list'.");
}
