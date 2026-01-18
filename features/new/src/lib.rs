pub mod skeleton;
pub mod templates;

use core_config::UserConfig;
use dialoguer::{theme::ColorfulTheme, Input};
use std::collections::HashMap;

pub struct NewArgs {
    pub name: String,
    pub path: Option<String>,
    pub group: Option<String>,
    pub package: Option<String>,
    pub no_ui_dir: bool,
    pub author: Option<String>,
    pub version: Option<String>,
    pub desc: Option<String>,
    pub interactive: bool,
}

pub fn generate_plan(args: NewArgs, skeleton_bytes: &[u8]) -> core_plan::Plan {
    let config = UserConfig::load();

    let name = args.name.clone();

    // Resolve Author
    let author = resolve_val(
        "Author",
        args.interactive,
        args.author.as_deref(),
        config.author.as_deref(),
        Some("Your Name"),
    );

    // Resolve Group
    let group = resolve_val(
        "Group ID",
        args.interactive,
        args.group.as_deref(),
        config.group.as_deref(),
        Some("com.example"),
    );

    // Resolve Version
    let version = resolve_val(
        "Version",
        args.interactive,
        args.version.as_deref(),
        config.version.as_deref(),
        Some("0.1.0"),
    );

    // Resolve Description
    let desc = resolve_val(
        "Description",
        args.interactive,
        args.desc.as_deref(),
        config.desc.as_deref(),
        Some("A Hytale mod"),
    );

    // Derived: Username (from config only, not args for now)
    // If not set, we can leave it as <USERNAME> or default to system user?
    // Requirement says "properly used", implying replacement.
    let username = config.username.as_deref().unwrap_or("HytaleUser");

    // Prepare replacements
    let mut replacements = HashMap::new();
    replacements.insert("<MOD_NAME>".to_string(), name.clone());
    replacements.insert("<MOD_ID>".to_string(), name.clone()); // Assuming ID same as Name for simple case, or derive snake_case
    replacements.insert("<AUTHOR>".to_string(), author);
    replacements.insert("<GROUP_ID>".to_string(), group.clone());
    replacements.insert("<VERSION>".to_string(), version);
    replacements.insert("<DESCRIPTION>".to_string(), desc);
    replacements.insert("<USERNAME>".to_string(), username.to_string());

    // Derived: Package
    // If not provided, derive from group + name (slugified)
    // For now simple derivation:
    let package_default = format!("{}.{}", group, name.to_lowercase().replace("-", "_"));
    let package = if let Some(p) = args.package {
        p
    } else {
        package_default
    };
    replacements.insert("<PACKAGE>".to_string(), package.clone());
    replacements.insert("<PACKAGE_DIR>".to_string(), package.replace('.', "/"));
    replacements.insert("<MAIN_CLASS>".to_string(), format!("{}.Main", package));

    replacements.insert("com.example.skeleton".to_string(), package.clone());

    // Add these LAST to allow overrides if needed, though HashMap order is arbitrary in iteration.
    // Ideally we should do replacements in a deterministic or specific order, but for now simple string replacement
    // works if keys don't overlap in a way that breaks things.
    // "com.example.skeleton" is more specific than "com.example" so it should be fine if we don't have "com.example" rule.

    let steps = skeleton::generate_plan(&name, skeleton_bytes, &replacements);
    core_plan::Plan { steps }
}

fn resolve_val(
    prompt: &str,
    interactive: bool,
    flag: Option<&str>,
    config: Option<&str>,
    default: Option<&str>,
) -> String {
    // 1. Flag
    if let Some(v) = flag {
        return v.to_string();
    }

    let default_val = config.or(default).unwrap_or("");

    // 2. Interactive
    if interactive {
        return Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default_val.to_string())
            .interact_text()
            .unwrap_or_else(|_| default_val.to_string());
    }

    // 3. Config / Default
    default_val.to_string()
}
