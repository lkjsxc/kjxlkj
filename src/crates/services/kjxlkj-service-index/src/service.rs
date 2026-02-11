//! Index service implementation.

use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use tracing::info;

/// Index service error.
#[derive(Debug, Error)]
pub enum IndexError {
    #[error("Index operation failed: {0}")]
    Operation(String),
    #[error("Path not indexed: {0}")]
    NotIndexed(String),
}

/// Indexed file entry.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    /// File path relative to workspace root.
    pub path: PathBuf,
    /// File size in bytes.
    pub size: u64,
    /// Last modification time (Unix timestamp).
    pub modified: u64,
}

/// Index service for file search and symbol indexing.
pub struct IndexService {
    /// Workspace root.
    root: Option<PathBuf>,
    /// Indexed files by path.
    files: HashMap<PathBuf, IndexEntry>,
    /// Whether indexing is active.
    indexing: bool,
}

impl IndexService {
    /// Create a new Index service.
    pub fn new() -> Self {
        Self {
            root: None,
            files: HashMap::new(),
            indexing: false,
        }
    }

    /// Initialize indexing for a workspace.
    pub fn init(&mut self, root: PathBuf) {
        info!("Initializing index service for {:?}", root);
        self.root = Some(root);
    }

    /// Set indexing state.
    pub fn set_indexing(&mut self, state: bool) {
        self.indexing = state;
    }

    /// Check if currently indexing.
    pub fn is_indexing(&self) -> bool {
        self.indexing
    }

    /// Add a file to the index.
    pub fn add_file(&mut self, path: PathBuf, size: u64, modified: u64) {
        let entry = IndexEntry { path: path.clone(), size, modified };
        self.files.insert(path, entry);
    }

    /// Remove a file from the index.
    pub fn remove_file(&mut self, path: &PathBuf) {
        self.files.remove(path);
    }

    /// Get indexed file count.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Search files by fuzzy pattern.
    pub fn search_files(&self, pattern: &str) -> Vec<&IndexEntry> {
        let pattern_lower = pattern.to_lowercase();
        self.files.values()
            .filter(|e| {
                let filename = e.path.file_name()
                    .map(|s| s.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                filename.contains(&pattern_lower)
            })
            .collect()
    }
}

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_service_creation() {
        let service = IndexService::new();
        assert_eq!(service.file_count(), 0);
    }

    #[test]
    fn test_add_and_search_files() {
        let mut service = IndexService::new();
        service.init(PathBuf::from("/workspace"));
        
        service.add_file(PathBuf::from("src/main.rs"), 1000, 1234567890);
        service.add_file(PathBuf::from("src/lib.rs"), 500, 1234567890);
        service.add_file(PathBuf::from("Cargo.toml"), 200, 1234567890);
        
        assert_eq!(service.file_count(), 3);
        
        let results = service.search_files("main");
        assert_eq!(results.len(), 1);
        
        let results = service.search_files(".rs");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_indexing_state() {
        let mut service = IndexService::new();
        assert!(!service.is_indexing());
        
        service.set_indexing(true);
        assert!(service.is_indexing());
        
        service.set_indexing(false);
        assert!(!service.is_indexing());
    }
}
