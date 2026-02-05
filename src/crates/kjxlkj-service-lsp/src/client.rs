//! LSP client service (placeholder).

/// LSP client service.
pub struct LspService {
    /// Whether the service is running.
    running: bool,
}

impl Default for LspService {
    fn default() -> Self {
        Self::new()
    }
}

impl LspService {
    /// Create new LSP service.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Start the service.
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the service.
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_service() {
        let mut svc = LspService::new();
        assert!(!svc.is_running());
        svc.start();
        assert!(svc.is_running());
        svc.stop();
        assert!(!svc.is_running());
    }
}
