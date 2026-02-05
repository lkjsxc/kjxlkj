//! LSP client service.

use async_trait::async_trait;
use tokio::sync::mpsc;

/// LSP service for language server communication.
pub struct LspService {
    running: bool,
}

impl LspService {
    /// Create a new LSP service.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Start the service.
    pub async fn start(&mut self) {
        self.running = true;
        tracing::info!("LSP service started");
    }

    /// Stop the service.
    pub async fn stop(&mut self) {
        self.running = false;
        tracing::info!("LSP service stopped");
    }

    /// Check if the service is running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for LspService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lsp_service() {
        let mut svc = LspService::new();
        assert!(!svc.is_running());
        svc.start().await;
        assert!(svc.is_running());
        svc.stop().await;
        assert!(!svc.is_running());
    }
}
