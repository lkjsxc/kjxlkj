//! LSP client service — language server protocol integration.

use std::path::PathBuf;

/// Configuration for an LSP server.
#[derive(Debug, Clone)]
pub struct LspServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub root_dir: PathBuf,
}

/// Represents the state of an LSP connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspConnectionState {
    Disconnected,
    Initializing,
    Ready,
    ShuttingDown,
}

/// An LSP service instance managing communication with a language server.
pub struct LspService {
    config: Option<LspServerConfig>,
    state: LspConnectionState,
}

impl LspService {
    pub fn new() -> Self {
        Self {
            config: None,
            state: LspConnectionState::Disconnected,
        }
    }

    /// Configure and prepare the service for a language server.
    pub fn configure(&mut self, config: LspServerConfig) {
        self.config = Some(config);
    }

    /// Current connection state.
    pub fn state(&self) -> LspConnectionState {
        self.state
    }

    /// Start the LSP server process and perform the initialize handshake.
    pub async fn start(&mut self) -> anyhow::Result<()> {
        let config = self.config.as_ref().ok_or_else(|| {
            anyhow::anyhow!("LSP service not configured")
        })?;
        tracing::info!(server = %config.name, "starting LSP server");
        self.state = LspConnectionState::Initializing;
        // Future: spawn process, perform initialize handshake
        self.state = LspConnectionState::Ready;
        Ok(())
    }

    /// Shut down the LSP server.
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        self.state = LspConnectionState::ShuttingDown;
        tracing::info!("shutting down LSP server");
        self.state = LspConnectionState::Disconnected;
        Ok(())
    }
}

impl Default for LspService {
    fn default() -> Self {
        Self::new()
    }
}
