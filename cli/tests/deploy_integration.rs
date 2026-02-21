#[path = "../deploy/tests"]
mod deploy_tests {

    mod test_deploy_dry_run;
    mod test_deploy_dry_run_with_transport;
    mod test_deploy_force_scp;
    mod test_deploy_full_pipeline;

    mod test_deploy_path_arg;
    mod test_deploy_remote_path_jar;

    mod test_deploy_with_default_server;
    mod test_deploy_with_server_name;

    mod test_transport_override_rsync;
    mod test_transport_override_scp;
}
