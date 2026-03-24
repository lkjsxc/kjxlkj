//! Compose verification command

use std::process::Command;

/// Run compose verification
pub fn verify() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("docker")
        .args(["compose", "--profile", "verify", "run", "--rm", "verify"])
        .status()?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    println!(r#"{{"command":"compose-verify","status":"pass"}}"#);
    Ok(())
}
