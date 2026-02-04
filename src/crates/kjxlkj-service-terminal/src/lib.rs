//! Terminal service.
//!
//! Terminal/PTY service for running external commands.

use anyhow::Result;
use std::process::{Command, Output};

/// Terminal service for running external commands.
pub struct TerminalService;

impl TerminalService {
    /// Create a new Terminal service.
    pub fn new() -> Self {
        Self
    }

    /// Run a shell command and return output.
    pub fn run_command(&self, cmd: &str) -> Result<Output> {
        let output = Command::new("sh").arg("-c").arg(cmd).output()?;
        Ok(output)
    }

    /// Run a shell command and return first line of output.
    pub fn run_command_first_line(&self, cmd: &str) -> Result<String> {
        let output = self.run_command(cmd)?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stdout.lines().next().unwrap_or("").to_string();
        Ok(line)
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
    fn test_terminal_service() {
        let svc = TerminalService::new();
        let result = svc.run_command_first_line("echo hello");
        assert_eq!(result.unwrap(), "hello");
    }
}
