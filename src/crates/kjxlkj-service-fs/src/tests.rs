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

mod watcher_tests {
    use crate::FsWatcher;
    use std::path::PathBuf;

    #[test]
    fn test_watcher_new() {
        let watcher = FsWatcher::new();
        assert!(watcher.paths.is_empty());
    }

    #[test]
    fn test_watcher_watch() {
        let mut watcher = FsWatcher::new();
        watcher.watch(PathBuf::from("/home/user"));
        assert_eq!(watcher.paths.len(), 1);
    }

    #[test]
    fn test_watcher_unwatch() {
        let mut watcher = FsWatcher::new();
        let path = PathBuf::from("/home/user");
        watcher.watch(path.clone());
        watcher.unwatch(&path);
        assert!(watcher.paths.is_empty());
    }

    #[test]
    fn test_watcher_multiple() {
        let mut watcher = FsWatcher::new();
        watcher.watch(PathBuf::from("/a"));
        watcher.watch(PathBuf::from("/b"));
        watcher.watch(PathBuf::from("/c"));
        assert_eq!(watcher.paths.len(), 3);
    }
}

mod service_tests {
    use crate::FsService;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup() -> TempDir {
        TempDir::new().unwrap()
    }

    #[tokio::test]
    async fn test_service_save_load() {
        let dir = setup();
        let svc = FsService::new(dir.path().to_path_buf());
        let path = dir.path().join("test.txt");

        svc.save(&path, "content").await.unwrap();
        let loaded = svc.load(&path).await.unwrap();

        assert_eq!(loaded, "content");
    }

    #[test]
    fn test_service_root() {
        let root = PathBuf::from("/project");
        let svc = FsService::new(root.clone());
        assert_eq!(svc.root(), &root);
    }
}
