//! Compose verification command

use std::io;
use std::process::Command;

const FILE_ARGS: &[&str] = &[
    "compose",
    "-f",
    "docker-compose.yml",
    "-f",
    "docker-compose.verify.yml",
];

/// Run compose verification
pub fn verify() -> Result<(), Box<dyn std::error::Error>> {
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        run(&["build", "app", "verify", "visual-verify"])?;
        run(&["up", "-d", "postgres", "app"])?;
        run(&["run", "--rm", "verify"])?;
        run(&["run", "--rm", "visual-verify"])?;
        Ok(())
    })();

    let down = run(&["down", "-v"]);
    if let Err(error) = result {
        let _ = down;
        return Err(error);
    }
    down?;

    println!(r#"{{"command":"compose-verify","status":"pass"}}"#);
    Ok(())
}

fn run(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("docker").args(FILE_ARGS).args(args).status()?;
    if status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!("docker compose {args:?} failed")).into())
}
