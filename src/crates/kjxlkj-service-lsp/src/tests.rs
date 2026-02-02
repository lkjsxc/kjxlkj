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

    #[test]
    fn test_diagnostic_severity_values() {
        assert_eq!(DiagnosticSeverity::Error as u8, 1);
        assert_eq!(DiagnosticSeverity::Warning as u8, 2);
        assert_eq!(DiagnosticSeverity::Information as u8, 3);
        assert_eq!(DiagnosticSeverity::Hint as u8, 4);
    }

    #[test]
    fn test_diagnostic_source() {
        let range = Range::new(Position::new(1, 0), Position::new(1, 10));
        let mut diag = Diagnostic::new(range, "type error");
        diag.source = Some("rustc".into());
        assert_eq!(diag.source, Some("rustc".into()));
    }

    #[test]
    fn test_diagnostic_range() {
        let start = Position::new(5, 10);
        let end = Position::new(5, 20);
        let range = Range::new(start, end);
        let diag = Diagnostic::new(range, "test");
        assert_eq!(diag.range.start.line, 5);
        assert_eq!(diag.range.start.col, 10);
        assert_eq!(diag.range.end.col, 20);
    }

    #[test]
    fn test_lsp_client_path() {
        let path = PathBuf::from("/opt/lsp/server");
        let client = LspClient::new("custom", path.clone());
        assert_eq!(client.path(), &path);
    }
}

