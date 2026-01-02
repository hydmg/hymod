#[path = "../server/tests"]
mod server_tests {
    mod test_error_add_already_exists;
    mod test_error_add_empty_name;
    mod test_error_add_invalid_name_spaces;
    mod test_error_add_invalid_name_special_chars;
    mod test_error_add_missing_name;
    mod test_error_list_config_dir_unreadable;
    mod test_error_show_invalid_config;
    mod test_error_show_missing_name;
    mod test_error_show_not_found;
    mod test_error_test_local_path_missing;
    mod test_error_test_missing_name;
    mod test_error_test_not_found;
    mod test_error_test_remote_unreachable;
    mod test_server_add_creates_servers_dir;
    mod test_server_add_creates_stub_config;
    mod test_server_add_new;
    mod test_server_list_shows_kind;
    mod test_server_show_existing;
    mod test_server_show_local;
    mod test_server_show_remote;
    mod test_server_test_local_valid;
    mod test_server_test_remote_valid;
}
