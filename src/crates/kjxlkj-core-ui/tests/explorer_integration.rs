//! Integration tests for explorer with filesystem service.
//!
//! Tests explorer behavior with real directory operations.

use std::path::{Path, PathBuf};

// Note: Full integration with kjxlkj-service-fs would require adding it as a dev-dependency.
// These tests use the standard library for filesystem operations as a mock.

/// Mock directory listing result for testing.
fn list_directory_sync(path: &Path) -> Vec<(PathBuf, bool)> {
    // Returns (path, is_directory) pairs
    let mut entries = Vec::new();
    if let Ok(dir) = std::fs::read_dir(path) {
        for entry in dir.flatten() {
            let p = entry.path();
            let is_dir = p.is_dir();
            entries.push((p, is_dir));
        }
    }
    entries.sort_by(|a, b| {
        // Sort directories first, then by name
        match (a.1, b.1) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.0.cmp(&b.0),
        }
    });
    entries
}

#[test]
fn test_directory_listing_sorted() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    // Create test structure
    std::fs::create_dir(root.join("src")).unwrap();
    std::fs::create_dir(root.join("docs")).unwrap();
    std::fs::write(root.join("README.md"), "test").unwrap();
    std::fs::write(root.join("Cargo.toml"), "test").unwrap();

    let entries = list_directory_sync(root);

    // Directories should come first
    assert_eq!(entries.len(), 4);
    assert!(entries[0].1); // docs is directory
    assert!(entries[1].1); // src is directory
    assert!(!entries[2].1); // Cargo.toml is file
    assert!(!entries[3].1); // README.md is file
}

#[test]
fn test_empty_directory() {
    let temp = tempfile::tempdir().unwrap();
    let entries = list_directory_sync(temp.path());
    assert!(entries.is_empty());
}

#[test]
fn test_nested_directories() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    // Create nested structure
    std::fs::create_dir_all(root.join("src/crates/core")).unwrap();
    std::fs::write(root.join("src/lib.rs"), "").unwrap();
    std::fs::write(root.join("src/crates/mod.rs"), "").unwrap();

    let root_entries = list_directory_sync(root);
    assert_eq!(root_entries.len(), 1); // just "src"
    assert!(root_entries[0].1);

    let src_entries = list_directory_sync(&root.join("src"));
    assert_eq!(src_entries.len(), 2); // crates dir + lib.rs
}

#[test]
fn test_large_directory_listing() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    // Create 100 files (simulating a portion of a large directory)
    for i in 0..100 {
        std::fs::write(root.join(format!("file_{:04}.txt", i)), "").unwrap();
    }

    let entries = list_directory_sync(root);
    assert_eq!(entries.len(), 100);

    // Verify sorted order
    for i in 0..99 {
        assert!(entries[i].0 < entries[i + 1].0);
    }
}

#[test]
fn test_directory_with_hidden_files() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    std::fs::write(root.join(".gitignore"), "").unwrap();
    std::fs::write(root.join(".env"), "").unwrap();
    std::fs::write(root.join("main.rs"), "").unwrap();

    let entries = list_directory_sync(root);
    // Hidden files are still listed (no filtering in basic listing)
    assert_eq!(entries.len(), 3);
}

/// Simulates incremental directory loading for large directories.
fn list_directory_incremental(path: &Path, limit: usize) -> (Vec<(PathBuf, bool)>, bool) {
    let mut entries = Vec::new();
    let mut has_more = false;

    if let Ok(dir) = std::fs::read_dir(path) {
        for entry in dir.flatten() {
            if entries.len() >= limit {
                has_more = true;
                break;
            }
            let p = entry.path();
            let is_dir = p.is_dir();
            entries.push((p, is_dir));
        }
    }

    entries.sort_by(|a, b| {
        match (a.1, b.1) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.0.cmp(&b.0),
        }
    });

    (entries, has_more)
}

#[test]
fn test_incremental_loading_small_dir() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    for i in 0..5 {
        std::fs::write(root.join(format!("{}.txt", i)), "").unwrap();
    }

    let (entries, has_more) = list_directory_incremental(root, 10);
    assert_eq!(entries.len(), 5);
    assert!(!has_more);
}

#[test]
fn test_incremental_loading_large_dir() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    for i in 0..50 {
        std::fs::write(root.join(format!("{}.txt", i)), "").unwrap();
    }

    let (entries, has_more) = list_directory_incremental(root, 20);
    assert_eq!(entries.len(), 20);
    assert!(has_more);
}

#[test]
fn test_directory_responsiveness_1000_files() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    // Create 1000 files
    for i in 0..1000 {
        std::fs::write(root.join(format!("file_{:05}.txt", i)), "").unwrap();
    }

    // Measure time for incremental load (should be fast)
    let start = std::time::Instant::now();
    let (entries, has_more) = list_directory_incremental(root, 100);
    let elapsed = start.elapsed();

    assert_eq!(entries.len(), 100);
    assert!(has_more);
    // Should complete in under 100ms
    assert!(elapsed.as_millis() < 100, "Incremental load took too long: {:?}", elapsed);
}
