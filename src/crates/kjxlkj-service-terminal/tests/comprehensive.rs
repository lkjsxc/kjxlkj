//! Comprehensive tests for kjxlkj-service-terminal.

use kjxlkj_service_terminal::*;

mod terminal_service_tests {
    use super::*;

    #[test]
    fn test_terminal_service_new() {
        let svc = TerminalService::new();
        assert!(!svc.is_running());
    }

    #[test]
    fn test_terminal_service_default() {
        let svc = TerminalService::default();
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_terminal_service_start() {
        let mut svc = TerminalService::new();
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_terminal_service_stop() {
        let mut svc = TerminalService::new();
        svc.start().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_terminal_service_restart() {
        let mut svc = TerminalService::new();
        svc.start().await;
        svc.stop().await;
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_terminal_service_double_start() {
        let mut svc = TerminalService::new();
        svc.start().await;
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_terminal_service_double_stop() {
        let mut svc = TerminalService::new();
        svc.start().await;
        svc.stop().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }
}
