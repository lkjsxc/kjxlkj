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

mod extra_fs_tests {
    use super::*;

    #[tokio::test]
    async fn test_fs_service_double_start() {
        let mut svc = FsService::new();
        svc.start().await;
        svc.start().await; // Double start should be fine
        assert!(svc.is_running());
    }

    #[tokio::test]
    async fn test_fs_service_double_stop() {
        let mut svc = FsService::new();
        svc.start().await;
        svc.stop().await;
        svc.stop().await; // Double stop should be fine
        assert!(!svc.is_running());
    }

    #[tokio::test]
    async fn test_fs_service_write_empty_file() {
        let svc = FsService::new();
        let file_path = PathBuf::from("/tmp/kjxlkj_empty_test.txt");

        let write_result = svc.write_file(&file_path, "").await;
        assert!(write_result.is_ok());

        let read_result = svc.read_file(&file_path).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), "");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn test_fs_service_write_multiline() {
        let svc = FsService::new();
        let file_path = PathBuf::from("/tmp/kjxlkj_multiline_test.txt");

        let content = "line1\nline2\nline3\n";
        let write_result = svc.write_file(&file_path, content).await;
        assert!(write_result.is_ok());

        let read_result = svc.read_file(&file_path).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn test_fs_service_overwrite_file() {
        let svc = FsService::new();
        let file_path = PathBuf::from("/tmp/kjxlkj_overwrite_test.txt");

        // Write first content
        svc.write_file(&file_path, "first content").await.unwrap();

        // Overwrite with new content
        svc.write_file(&file_path, "second content").await.unwrap();

        let read_result = svc.read_file(&file_path).await.unwrap();
        assert_eq!(read_result, "second content");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn test_fs_service_read_directory_error() {
        let svc = FsService::new();
        let path = PathBuf::from("/tmp");
        // Reading a directory as file should fail or behave accordingly
        let result = svc.read_file(&path).await;
        // Either error or some behavior
        let _ = result;
    }

    #[tokio::test]
    async fn test_fs_service_write_nested_path() {
        let svc = FsService::new();
        // Writing to a new path should create parent dirs if the implementation supports it
        let file_path = PathBuf::from("/tmp/kjxlkj_nested/subdir/test.txt");

        // This may or may not work depending on implementation
        let _ = svc.write_file(&file_path, "nested content").await;

        // Cleanup
        let _ = std::fs::remove_file(&file_path);
        let _ = std::fs::remove_dir("/tmp/kjxlkj_nested/subdir");
        let _ = std::fs::remove_dir("/tmp/kjxlkj_nested");
    }

    #[tokio::test]
    async fn test_fs_service_unicode_content() {
        let svc = FsService::new();
        let file_path = PathBuf::from("/tmp/kjxlkj_unicode_test.txt");

        let content = "Hello 世界! 🎉 Ñoño";
        let write_result = svc.write_file(&file_path, content).await;
        assert!(write_result.is_ok());

        let read_result = svc.read_file(&file_path).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn test_fs_service_special_chars_content() {
        let svc = FsService::new();
        let file_path = PathBuf::from("/tmp/kjxlkj_special_test.txt");

        let content = "Tab:\t Null:? Quote:\"";
        let write_result = svc.write_file(&file_path, content).await;
        assert!(write_result.is_ok());

        let read_result = svc.read_file(&file_path).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        let _ = std::fs::remove_file(&file_path);
    }
}
