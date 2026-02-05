//! Comprehensive tests for kjxlkj-service-lsp.

use kjxlkj_service_lsp::*;

mod lsp_service_tests {
    use super::*;

    #[test]
    fn test_lsp_service_new() {
        let svc = LspService::new();
        assert!(!svc.is_running());
    }

    #[test]
    fn test_lsp_service_default() {
        let svc = LspService::default();
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_lsp_service_start() {
        let mut svc = LspService::new();
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_lsp_service_stop() {
        let mut svc = LspService::new();
        svc.start().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_lsp_service_restart() {
        let mut svc = LspService::new();
        svc.start().await;
        svc.stop().await;
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_lsp_service_double_start() {
        let mut svc = LspService::new();
        svc.start().await;
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_lsp_service_double_stop() {
        let mut svc = LspService::new();
        svc.start().await;
        svc.stop().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }
}

mod extra_lsp_tests {
    use super::*;

    #[test]
    fn test_lsp_service_initial_not_running() {
        let svc = LspService::new();
        assert!(!svc.is_running());
    }

    #[test]
    fn test_lsp_service_default_not_running() {
        let svc = LspService::default();
        assert!(!svc.is_running());
    }
}
