fn main() {
    // Only run if we are in the crate directory (sanity check, though build.rs always runs there)
    // We need to run the shell script at scripts/update_skeleton.sh relative to root

    // Rerun if the script changes
    println!("cargo:rerun-if-changed=scripts/update_skeleton.sh");

    // Execute the script
    // Note: This relies on a *nix environment (bash).
    // If we wanted cross-platform we'd do more, but user is on Mac.
    let status = std::process::Command::new("./scripts/update_skeleton.sh").status();

    match status {
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
}
