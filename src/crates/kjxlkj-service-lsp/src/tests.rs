//! Tests for LSP service.

#[cfg(test)]
mod tests {
    use super::super::client::LspClient;
    use super::super::protocol::{Diagnostic, DiagnosticSeverity};
    use kjxlkj_core_types::{Position, Range};
    use std::path::PathBuf;

    #[test]
    fn test_lsp_client_new() {
        let client = LspClient::new("rust-analyzer", PathBuf::from("/usr/bin/rust-analyzer"));
        assert_eq!(client.name(), "rust-analyzer");
        assert!(!client.is_running());
    }

    #[tokio::test]
    async fn test_lsp_client_start() {
        let mut client = LspClient::new("test", PathBuf::from("/bin/test"));
        client.start().await.unwrap();
        assert!(client.is_running());
    }

    #[tokio::test]
    async fn test_lsp_client_stop() {
        let mut client = LspClient::new("test", PathBuf::from("/bin/test"));
        client.start().await.unwrap();
        client.stop().await;
        assert!(!client.is_running());
    }

    #[test]
    fn test_diagnostic_new() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        let diag = Diagnostic::new(range, "unused variable");
        assert_eq!(diag.message, "unused variable");
        assert!(diag.severity.is_none());
    }

    #[test]
    fn test_diagnostic_with_severity() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        let diag = Diagnostic::new(range, "error")
            .with_severity(DiagnosticSeverity::Error);
        assert_eq!(diag.severity, Some(DiagnosticSeverity::Error));
    }
}

