use std::process::Command;

use clap::Subcommand;
use serde_json::json;

#[derive(Subcommand)]
pub enum ComposeCmd {
    Verify,
}

pub fn run(command: ComposeCmd) -> anyhow::Result<()> {
    match command {
        ComposeCmd::Verify => {
            let status = Command::new("docker")
                .args(["compose", "--profile", "verify", "run", "--rm", "verify"])
                .status()?;
            if status.success() {
                println!("{}", json!({"ok": true, "command": "compose_verify"}));
                Ok(())
            } else {
                anyhow::bail!("compose_verify_failed")
            }
        }
    }
}
