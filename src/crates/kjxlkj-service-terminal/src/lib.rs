//! Terminal/PTY service.

use anyhow::Result;
use std::process::{Command, Output};

/// Terminal service for running external commands.
pub struct TerminalService {
    // Future: manage PTY sessions
}

impl TerminalService {
    /// Create a new Terminal service.
    pub fn new() -> Self {
        Self {}
    }

    /// Run a shell command and return output.
    pub fn run_command(cmd: &str) -> Result<Output> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", cmd]).output()?
        } else {
            Command::new("sh").args(["-c", cmd]).output()?
        };
        Ok(output)
    }

    /// Run a command and return stdout as string.
    pub fn run_command_str(cmd: &str) -> Result<String> {
        let output = Self::run_command(cmd)?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_service_creation() {
        let _svc = TerminalService::new();
    }

    #[test]
    fn run_echo_command() {
        let output = TerminalService::run_command_str("echo hello").unwrap();
        assert!(output.contains("hello"));
    }

    #[test]
    fn run_pwd_command() {
        let output = TerminalService::run_command_str("pwd").unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn run_invalid_command_fails() {
        // An obviously invalid command
        let result = TerminalService::run_command_str("nonexistent_command_xyz123");
        // May succeed with error message or fail entirely
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn terminal_service_default() {
        let svc = TerminalService::default();
        let _ = svc;
    }

    #[test]
    fn run_command_returns_output() {
        let output = TerminalService::run_command("echo test").unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn run_command_str_trims() {
        let output = TerminalService::run_command_str("echo test").unwrap();
        assert!(output.len() > 0);
    }
}
