#[test]
fn test_error_add_invalid_name_special_chars() {
    panic!("Execute: hymod server add my@server, verify error 'Invalid server name: contains invalid characters'.");
}
