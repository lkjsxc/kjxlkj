//! LSP client service (placeholder).

/// LSP service (placeholder for future implementation).
pub struct LspService {
    // Future: manage LSP client connections
}

impl LspService {
    /// Create a new LSP service.
    pub fn new() -> Self {
        Self {}
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
    fn lsp_service_creation() {
        let _svc = LspService::new();
    }
}
