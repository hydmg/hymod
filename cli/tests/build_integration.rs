#[path = "../build/tests"]
mod build_tests {
    mod test_build_dry_run;
    mod test_build_release_dry_run;
    mod test_error_artifact_dir_not_writable;
    mod test_error_gradle_failure;
    mod test_error_invalid_hymod_yaml;
    mod test_error_missing_required_fields;
    mod test_error_not_in_mod_directory;
}
