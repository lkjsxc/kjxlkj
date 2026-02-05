//! Comprehensive tests for kjxlkj-service-git.

use kjxlkj_service_git::*;

mod git_service_tests {
    use super::*;

    #[test]
    fn test_git_service_new() {
        let svc = GitService::new();
        assert!(!svc.is_running());
    }

    #[test]
    fn test_git_service_default() {
        let svc = GitService::default();
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_git_service_start() {
        let mut svc = GitService::new();
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_git_service_stop() {
        let mut svc = GitService::new();
        svc.start().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_git_service_restart() {
        let mut svc = GitService::new();
        svc.start().await;
        assert!(svc.is_running());
        svc.stop().await;
        assert!(!svc.is_running());
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_git_service_double_start() {
        let mut svc = GitService::new();
        svc.start().await;
        assert!(svc.is_running());
        svc.start().await; // Should be idempotent
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_git_service_double_stop() {
        let mut svc = GitService::new();
        svc.start().await;
        svc.stop().await;
        assert!(!svc.is_running());
        svc.stop().await; // Should be idempotent
        assert!(!svc.is_running());
    }
}
