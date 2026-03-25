//! Compose verification command

use std::process::Command;

/// Run compose verification
pub fn verify() -> Result<(), Box<dyn std::error::Error>> {
    for service in ["verify", "visual-verify"] {
        let status = Command::new("docker")
            .args(["compose", "--profile", "verify", "run", "--rm", service])
            .status()?;

        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    println!(r#"{{"command":"compose-verify","status":"pass"}}"#);
    Ok(())
}
