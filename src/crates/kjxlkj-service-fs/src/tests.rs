//! Tests for filesystem service.

mod ops_tests {
    use super::super::ops::{create_dir, exists, read_file, write_file};
    use std::fs;
    use tempfile::TempDir;

    fn setup() -> TempDir {
        TempDir::new().unwrap()
    }

    #[tokio::test]
    async fn test_write_read_file() {
        let dir = setup();
        let path = dir.path().join("test.txt");
        
        write_file(&path, "hello world").await.unwrap();
        let content = read_file(&path).await.unwrap();
        
        assert_eq!(content, "hello world");
    }

    #[tokio::test]
    async fn test_exists() {
        let dir = setup();
        let path = dir.path().join("exists.txt");
        
        assert!(!exists(&path).await);
        fs::write(&path, "x").unwrap();
        assert!(exists(&path).await);
    }

    #[tokio::test]
    async fn test_create_dir() {
        let dir = setup();
        let nested = dir.path().join("a/b/c");
        
        create_dir(&nested).await.unwrap();
        assert!(nested.exists());
    }

    #[tokio::test]
    async fn test_read_nonexistent() {
        let result = read_file("/nonexistent/path/file.txt").await;
        assert!(result.is_err());
    }
}
