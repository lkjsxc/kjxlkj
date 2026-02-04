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

    #[test]
    fn lsp_service_default() {
        let svc = LspService::default();
        // Verify default creates valid service
        let _ = svc;
    }

    #[test]
    fn lsp_service_struct_exists() {
        fn assert_type<T>(_: &T) {}
        let svc = LspService::new();
        assert_type::<LspService>(&svc);
    }
}
