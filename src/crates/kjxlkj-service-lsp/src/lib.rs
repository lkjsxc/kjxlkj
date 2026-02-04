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

    #[test]
    fn lsp_service_multiple_instances() {
        let svc1 = LspService::new();
        let svc2 = LspService::default();
        let _ = (&svc1, &svc2);
    }

    #[test]
    fn lsp_service_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<LspService>();
    }

    #[test]
    fn lsp_service_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<LspService>();
    }
}
