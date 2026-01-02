#[path = "../link/tests"]
mod link_tests {
    mod test_default_server_link_local;
    mod test_default_server_link_validates_local;
    mod test_error_artifact_missing;
    mod test_error_invalid_server_config;
    mod test_error_missing_mods_dir;
    mod test_error_missing_server_no_default;
    mod test_error_missing_server_root;
    mod test_error_server_is_remote;
    mod test_error_server_not_found;
    mod test_error_server_root_not_writable;
    mod test_golden_plan_link;
    mod test_link_artifact_missing;
    mod test_link_creates_mods_dir;
    mod test_link_creates_symlink;
    mod test_link_local_server;
    mod test_link_multiple_mods_same_server;
    mod test_link_overwrites_existing_symlink;
    mod test_link_server_not_found;
    mod test_link_ssh_server_rejected;
    mod test_link_symlink_points_to_artifact;
    mod test_link_updates_existing_symlink;
    mod test_link_with_default_server;
    mod test_link_with_server_name;
}
