//! Terminal service for kjxlkj editor.
//!
//! Provides embedded terminal functionality.

use kjxlkj_services::{Service, ServiceMessage};
use std::future::Future;
use std::pin::Pin;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Terminal session ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(u32);

impl TerminalId {
    /// Create a new terminal ID.
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the ID value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Terminal session.
pub struct TerminalSession {
    /// Session ID.
    pub id: TerminalId,
    /// Shell process.
    child: Option<Child>,
    /// Output buffer.
    output: Vec<String>,
}

impl TerminalSession {
    /// Create a new terminal session.
    pub fn new(id: TerminalId) -> Self {
        Self {
            id,
            child: None,
            output: Vec::new(),
        }
    }

    /// Start the shell.
    pub async fn start(&mut self, shell: &str) -> Result<(), std::io::Error> {
        let child = Command::new(shell)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.child = Some(child);
        Ok(())
    }

    /// Write to the terminal.
    pub async fn write(&mut self, data: &str) -> Result<(), std::io::Error> {
        if let Some(ref mut child) = self.child {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(data.as_bytes()).await?;
                stdin.flush().await?;
            }
        }
        Ok(())
    }

    /// Kill the terminal.
    pub async fn kill(&mut self) -> Result<(), std::io::Error> {
        if let Some(ref mut child) = self.child {
            child.kill().await?;
        }
        self.child = None;
        Ok(())
    }

    /// Check if terminal is running.
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            child.try_wait().ok().flatten().is_none()
        } else {
            false
        }
    }

    /// Get output.
    pub fn output(&self) -> &[String] {
        &self.output
    }
}

/// Terminal service.
pub struct TerminalService {
    /// Service name.
    name: String,
    /// Default shell.
    shell: String,
}

impl TerminalService {
    /// Create a new terminal service.
    pub fn new() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        Self {
            name: "terminal".to_string(),
            shell,
        }
    }

    /// Get the default shell.
    pub fn shell(&self) -> &str {
        &self.shell
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for TerminalService {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(
        self: Box<Self>,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            info!("Terminal service started");

            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => {
                        info!("Terminal service shutting down");
                        break;
                    }
                    ServiceMessage::Custom(cmd) => {
                        debug!(%cmd, "Received command");
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_service_new() {
        let service = TerminalService::new();
        assert_eq!(service.name(), "terminal");
    }

    #[test]
    fn test_terminal_id() {
        let id = TerminalId::new(1);
        assert_eq!(id.value(), 1);
    }

    #[test]
    fn test_terminal_session_new() {
        let session = TerminalSession::new(TerminalId::new(1));
        assert_eq!(session.id, TerminalId::new(1));
        assert!(session.output().is_empty());
    }

    #[test]
    fn test_terminal_id_equality() {
        let id1 = TerminalId::new(1);
        let id2 = TerminalId::new(1);
        let id3 = TerminalId::new(2);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_terminal_id_clone() {
        let id = TerminalId::new(5);
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_terminal_id_debug() {
        let id = TerminalId::new(42);
        let debug = format!("{:?}", id);
        assert!(debug.contains("42"));
    }

    #[test]
    fn test_terminal_service_default() {
        let service = TerminalService::default();
        assert_eq!(service.name(), "terminal");
    }

    #[test]
    fn test_terminal_session_not_running() {
        let mut session = TerminalSession::new(TerminalId::new(1));
        assert!(!session.is_running());
    }
}
