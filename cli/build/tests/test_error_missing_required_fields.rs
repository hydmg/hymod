#[ignore]
#[test]
fn test_error_missing_required_fields() {
    panic!("Execute: hymod build (with hymod.yaml missing modId), verify error 'Invalid hymod.yaml: missing field modId'.");
}
