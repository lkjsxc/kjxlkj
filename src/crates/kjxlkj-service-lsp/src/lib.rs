//! Language Server Protocol client service.
//!
//! This crate provides LSP client functionality.
//! Currently a placeholder for future implementation.

/// LSP service state.
pub struct LspService {
    /// Whether the service is active.
    active: bool,
}

impl LspService {
    /// Create a new LSP service.
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Start the service.
    pub fn start(&mut self) {
        self.active = true;
    }

    /// Stop the service.
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Check if active.
    pub fn is_active(&self) -> bool {
        self.active
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

    #[test]
    fn lsp_service_lifecycle() {
        let mut svc = LspService::new();
        assert!(!svc.is_active());
        svc.start();
        assert!(svc.is_active());
        svc.stop();
        assert!(!svc.is_active());
    }
}
