//! LSP client.

use std::path::PathBuf;

/// LSP client for a language server.
pub struct LspClient {
    /// Server name.
    name: String,
    /// Server command.
    command: PathBuf,
    /// Running flag.
    running: bool,
}

impl LspClient {
    /// Creates a new client.
    pub fn new(name: impl Into<String>, command: PathBuf) -> Self {
        Self {
            name: name.into(),
            command,
            running: false,
        }
    }

    /// Starts the server.
    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        Ok(())
    }

    /// Stops the server.
    pub async fn stop(&mut self) {
        self.running = false;
    }

    /// Returns true if running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Returns the server name.
    pub fn name(&self) -> &str {
        &self.name
    }
}
