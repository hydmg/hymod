#[path = "../dev/tests"]
mod dev_tests {
    mod test_default_server_dev_local;
    mod test_default_server_dev_validates_local;
    mod test_deploy_to_target;
    mod test_dev_default_server_from_config;
    mod test_dev_full_pipeline;
    mod test_dev_no_default_server_error;
    mod test_dev_oneshot_flow;
    mod test_dev_override_restart;
    mod test_dev_restart_command_from_server_config;
    mod test_dev_runs_build;
    mod test_dev_runs_link;
    mod test_dev_runs_restart;
    mod test_dev_watch_mode_init;
    mod test_dev_with_default_server;
    mod test_dev_with_server_name;
    mod test_error_build_fails;
    mod test_error_invalid_server_config;
    mod test_error_link_fails;
    mod test_error_missing_restart_cmd;
    mod test_error_missing_server_no_default;
    mod test_error_restart_fails;
    mod test_error_server_is_remote;
    mod test_error_server_not_found;
    mod test_golden_plan_dev;
}
