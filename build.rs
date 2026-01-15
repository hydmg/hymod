fn main() {
    // Update skeleton on EVERY build to ensure latest version
    // No cargo:rerun-if-changed for skeleton = always runs

    // Execute the script
    // Use "sh" on Windows (Git Bash), "bash" on Linux/macOS
    #[cfg(target_os = "windows")]
    let shell = "sh";
    #[cfg(not(target_os = "windows"))]
    let shell = "bash";

    let status_skeleton = std::process::Command::new(shell)
        .arg("./scripts/update_skeleton.sh")
        .status();

    match status_skeleton {
        Ok(s) => {
            if !s.success() {
                // If the script fails, we might want to fail the build or just warn?
                // Failing is safer to ensure we don't build with a broken skeleton.
                panic!("Failed to update skeleton: exit code {:?}", s.code());
            }
        }
        Err(e) => {
            panic!("Failed to execute update_skeleton.sh: {}", e);
        }
    }

    // Bump version using VERSION file
    // We do NOT modify Cargo.toml to avoid infinite rebuild loops (Cargo detects manifest changes).

    // Rerun if the script changes
    println!("cargo:rerun-if-changed=scripts/bump_version.sh");
    // Watch source directories so build.rs runs when code changes
    println!("cargo:rerun-if-changed=cli");
    println!("cargo:rerun-if-changed=core");
    println!("cargo:rerun-if-changed=features");
    // Also watch the script itself
    println!("cargo:rerun-if-changed=scripts/bump_version.sh");

    #[cfg(target_os = "windows")]
    let shell = "sh";
    #[cfg(not(target_os = "windows"))]
    let shell = "bash";

    let status_bump = std::process::Command::new(shell)
        .arg("./scripts/bump_version.sh")
        .status();
    match status_bump {
        Ok(s) => {
            if !s.success() {
                panic!("Failed to bump version: exit code {:?}", s.code());
            }
        }
        Err(e) => {
            panic!("Failed to execute bump_version.sh: {}", e);
        }
    }

    // Read the NEW version from Cargo.toml so we can compile it in
    // Note: Cargo parses Cargo.toml BEFORE running build.rs, so the env var CARGO_PKG_VERSION
    // will hold the *old* version unless we override it.

    // Quick and dirty read of line starting with version = "..."
    if let Ok(cargo_toml) = std::fs::read_to_string("Cargo.toml") {
        for line in cargo_toml.lines() {
            if line.starts_with("version = \"") {
                let v = line.trim().replace("version = \"", "").replace("\"", "");
                println!("cargo:rustc-env=CARGO_PKG_VERSION={}", v);
                break;
            }
        }
    }
}
