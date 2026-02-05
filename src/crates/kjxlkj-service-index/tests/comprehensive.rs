//! Comprehensive tests for kjxlkj-service-index.

use kjxlkj_service_index::*;

mod index_service_tests {
    use super::*;

    #[test]
    fn test_index_service_new() {
        let svc = IndexService::new();
        assert!(!svc.is_running());
    }

    #[test]
    fn test_index_service_default() {
        let svc = IndexService::default();
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_index_service_start() {
        let mut svc = IndexService::new();
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_index_service_stop() {
        let mut svc = IndexService::new();
        svc.start().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_index_service_restart() {
        let mut svc = IndexService::new();
        svc.start().await;
        svc.stop().await;
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_index_service_double_start() {
        let mut svc = IndexService::new();
        svc.start().await;
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_index_service_double_stop() {
        let mut svc = IndexService::new();
        svc.start().await;
        svc.stop().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }
}
