#[test]
fn test_hymod_yaml_missing_required_field() {
    panic!("Must return Error when mod.id, mod.version, or mod.entrypoint is missing.");
}
