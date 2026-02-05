//! Comprehensive tests for kjxlkj-service-fs.

use kjxlkj_service_fs::*;
use std::path::PathBuf;

mod fs_service_tests {
    use super::*;

    #[test]
    fn test_fs_service_new() {
        let svc = FsService::new();
        assert!(!svc.is_running());
    }

    #[test]
    fn test_fs_service_default() {
        let svc = FsService::default();
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_fs_service_start() {
        let mut svc = FsService::new();
        svc.start().await;
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_fs_service_stop() {
        let mut svc = FsService::new();
        svc.start().await;
        svc.stop().await;
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_fs_service_read_file() {
        let svc = FsService::new();
        // Try to read a file that exists
        let path = PathBuf::from("/home/lkjsxc/repos/kjxlkj/Cargo.toml");
        let result = svc.read_file(&path).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("[workspace]"));
    }

    #[tokio::test]
    async fn test_fs_service_read_nonexistent() {
        let svc = FsService::new();
        let path = PathBuf::from("/nonexistent/file/path/test.txt");
        let result = svc.read_file(&path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fs_service_write_and_read() {
        let svc = FsService::new();
        let file_path = PathBuf::from("/tmp/kjxlkj_test_file.txt");
        
        // Write file
        let content = "Hello, World!";
        let write_result = svc.write_file(&file_path, content).await;
        assert!(write_result.is_ok());
        
        // Read file back
        let read_result = svc.read_file(&file_path).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);
        
        // Cleanup
        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn test_fs_service_restart() {
        let mut svc = FsService::new();
        svc.start().await;
        assert!(svc.is_running());
        svc.stop().await;
        assert!(!svc.is_running());
        svc.start().await;
        assert!(svc.is_running());
    }
}
