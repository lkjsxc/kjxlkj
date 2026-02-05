//! Comprehensive tests for kjxlkj-services.

use kjxlkj_services::*;

mod supervisor_tests {
    use super::*;

    #[test]
    fn test_supervisor_new() {
        let supervisor = ServiceSupervisor::new();
        // Verify all services are accessible
        assert!(!supervisor.fs.is_running());
        assert!(!supervisor.git.is_running());
        assert!(!supervisor.index.is_running());
        assert!(!supervisor.lsp.is_running());
        assert!(!supervisor.terminal.is_running());
    }

    #[test]
    fn test_supervisor_default() {
        let supervisor = ServiceSupervisor::default();
        assert!(!supervisor.fs.is_running());
    }

    #[tokio::test]
    async fn test_supervisor_start_all() {
        let mut supervisor = ServiceSupervisor::new();
        supervisor.start_all().await;
        assert!(supervisor.fs.is_running());
        assert!(supervisor.git.is_running());
        assert!(supervisor.index.is_running());
        assert!(supervisor.lsp.is_running());
        assert!(supervisor.terminal.is_running());
    }

    #[tokio::test]
    async fn test_supervisor_stop_all() {
        let mut supervisor = ServiceSupervisor::new();
        supervisor.start_all().await;
        supervisor.stop_all().await;
        assert!(!supervisor.fs.is_running());
        assert!(!supervisor.git.is_running());
        assert!(!supervisor.index.is_running());
        assert!(!supervisor.lsp.is_running());
        assert!(!supervisor.terminal.is_running());
    }

    #[tokio::test]
    async fn test_supervisor_restart() {
        let mut supervisor = ServiceSupervisor::new();
        supervisor.start_all().await;
        assert!(supervisor.fs.is_running());
        supervisor.stop_all().await;
        assert!(!supervisor.fs.is_running());
        supervisor.start_all().await;
        assert!(supervisor.fs.is_running());
    }
}
