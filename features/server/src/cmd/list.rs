use crate::args::list_args::ServerListArgs;
use colored::*;
use core_config::server::{
    get_default_server_for_kind, list_servers, load_server_config, ServerConfig, ServerKind,
};

pub fn run(args: ServerListArgs) {
    let servers = match list_servers() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to list servers: {}", e);
            std::process::exit(1);
        }
    };

    let default_local = get_default_server_for_kind(&ServerKind::Local).unwrap_or(None);
    let default_remote = get_default_server_for_kind(&ServerKind::Remote).unwrap_or(None);

    let mut local_servers = Vec::new();
    let mut remote_servers = Vec::new();

    for name in servers {
        let config = match load_server_config(&name) {
            Ok(c) => c,
            Err(_) => continue,
        };

        match config.server.kind {
            ServerKind::Local => local_servers.push((name, config)),
            ServerKind::Remote => remote_servers.push((name, config)),
        }
    }

    // Filter if requested
    if let Some(ref filter) = args.filter_kind {
        if filter == "local" {
            remote_servers.clear();
        } else if filter == "remote" {
            local_servers.clear();
        }
    }

    if !local_servers.is_empty() {
        println!("\n{}", "Local Servers".bold().underline());
        print_local_table(&local_servers, default_local.as_deref().unwrap_or(""));
    }

    if !remote_servers.is_empty() {
        println!("\n{}", "Remote Servers".bold().underline());
        print_remote_table(&remote_servers, default_remote.as_deref().unwrap_or(""));
    }

    if local_servers.is_empty() && remote_servers.is_empty() {
        println!("No servers configured.");
    } else {
        println!(); // Extra newline at the end
    }
}

fn print_local_table(servers: &[(String, ServerConfig)], default_name: &str) {
    // Calculate widths
    let mut max_name = 4; // "Name"
    let mut max_path = 4; // "Path"

    for (name, config) in servers {
        max_name = std::cmp::max(max_name, name.len() + 2); // +2 for "* " marker
        max_path = std::cmp::max(max_path, config.server.server_root.len());
    }

    // Header
    println!(
        "{:<width_name$}  {:<width_path$}",
        "Name".bold(),
        "Path".bold(),
        width_name = max_name,
        width_path = max_path
    );

    // Separator (optional, skipping for clean look or could add dashes)

    // Rows
    for (name, config) in servers {
        let is_default = name == default_name;

        // Print Name column
        if is_default {
            let visible_len = name.len() + 2;
            let padding = max_name - visible_len;
            print!("{}{}", format!("* {}", name).green(), " ".repeat(padding));
        } else {
            let visible_len = name.len();
            let padding = max_name - visible_len;
            print!("{}{}", name, " ".repeat(padding));
        }

        let path = &config.server.server_root;
        println!("  {}", path);
    }
}

fn print_remote_table(servers: &[(String, ServerConfig)], default_name: &str) {
    // Columns: Name, Host, User, Port
    let mut max_name = 4;
    let mut max_host = 4;
    let mut max_user = 4;
    let mut max_port = 4;

    for (name, config) in servers {
        max_name = std::cmp::max(max_name, name.len() + 2);
        if let Some(remote) = &config.server.remote {
            max_host = std::cmp::max(max_host, remote.host.len());
            max_user = std::cmp::max(max_user, remote.user.len());
            max_port = std::cmp::max(max_port, remote.port.to_string().len());
        }
    }

    // Header
    println!(
        "{:<w_name$}  {:<w_host$}  {:<w_user$}  {:<w_port$}",
        "Name".bold(),
        "Host".bold(),
        "User".bold(),
        "Port".bold(),
        w_name = max_name,
        w_host = max_host,
        w_user = max_user,
        w_port = max_port
    );

    for (name, config) in servers {
        if let Some(remote) = &config.server.remote {
            // Name column
            let is_default = name == default_name;
            if is_default {
                let visible_len = name.len() + 2;
                let padding = max_name - visible_len;
                print!("{}{}", format!("* {}", name).green(), " ".repeat(padding));
            } else {
                let visible_len = name.len();
                let padding = max_name - visible_len;
                print!("{}{}", name, " ".repeat(padding));
            }

            println!(
                "  {:<w_host$}  {:<w_user$}  {:<w_port$}",
                remote.host,
                remote.user,
                remote.port,
                w_host = max_host,
                w_user = max_user,
                w_port = max_port
            );
        }
    }
}
