#[test]
fn test_error_invalid_name_special_chars() {
    panic!(
        "Execute: hymod new my@mod, verify error 'Invalid mod name: contains invalid characters'."
    );
}
