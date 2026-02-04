//! File indexing service for kjxlkj editor.
//!
//! Provides file and symbol indexing for navigation.

use kjxlkj_services::{Service, ServiceMessage};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::sync::mpsc;
use tracing::{debug, info};

/// Indexed file entry.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    /// File path.
    pub path: PathBuf,
    /// File name.
    pub name: String,
    /// File extension.
    pub extension: Option<String>,
    /// File size.
    pub size: u64,
    /// Last modified.
    pub modified: Option<std::time::SystemTime>,
}

/// File index.
#[derive(Debug, Default)]
pub struct FileIndex {
    /// Indexed files by path.
    files: HashMap<PathBuf, IndexEntry>,
    /// Files by name for quick lookup.
    by_name: HashMap<String, Vec<PathBuf>>,
}

impl FileIndex {
    /// Create a new file index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a file to the index.
    pub fn add(&mut self, entry: IndexEntry) {
        let name = entry.name.clone();
        let path = entry.path.clone();
        self.files.insert(path.clone(), entry);
        self.by_name.entry(name).or_default().push(path);
    }

    /// Remove a file from the index.
    pub fn remove(&mut self, path: &PathBuf) {
        if let Some(entry) = self.files.remove(path) {
            if let Some(paths) = self.by_name.get_mut(&entry.name) {
                paths.retain(|p| p != path);
            }
        }
    }

    /// Find files by name prefix.
    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&IndexEntry> {
        let prefix_lower = prefix.to_lowercase();
        self.files
            .values()
            .filter(|e| e.name.to_lowercase().starts_with(&prefix_lower))
            .collect()
    }

    /// Find files by fuzzy match.
    pub fn find_fuzzy(&self, query: &str) -> Vec<&IndexEntry> {
        let query_lower = query.to_lowercase();
        let mut matches: Vec<_> = self
            .files
            .values()
            .filter_map(|e| {
                let name_lower = e.name.to_lowercase();
                let score = fuzzy_score(&query_lower, &name_lower)?;
                Some((e, score))
            })
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        matches.into_iter().map(|(e, _)| e).collect()
    }

    /// Get file count.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if index is empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

/// Simple fuzzy matching score.
fn fuzzy_score(query: &str, target: &str) -> Option<i32> {
    let mut score = 0;
    let mut query_chars = query.chars().peekable();
    let mut last_match_idx = -1i32;

    for (idx, c) in target.chars().enumerate() {
        if let Some(&q) = query_chars.peek() {
            if c == q {
                query_chars.next();
                // Consecutive matches are worth more
                if last_match_idx == idx as i32 - 1 {
                    score += 2;
                } else {
                    score += 1;
                }
                last_match_idx = idx as i32;
            }
        }
    }

    if query_chars.peek().is_none() {
        Some(score)
    } else {
        None
    }
}

/// Index service.
pub struct IndexService {
    /// Service name.
    name: String,
}

impl IndexService {
    /// Create a new index service.
    pub fn new() -> Self {
        Self {
            name: "index".to_string(),
        }
    }
}

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for IndexService {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(
        self: Box<Self>,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            info!("Index service started");
            let _index = FileIndex::new();

            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => {
                        info!("Index service shutting down");
                        break;
                    }
                    ServiceMessage::Custom(cmd) => {
                        debug!(%cmd, "Received command");
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_service_new() {
        let service = IndexService::new();
        assert_eq!(service.name(), "index");
    }

    #[test]
    fn test_file_index_add() {
        let mut index = FileIndex::new();
        let entry = IndexEntry {
            path: PathBuf::from("/test/file.rs"),
            name: "file.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 100,
            modified: None,
        };
        index.add(entry);
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn test_fuzzy_score() {
        assert!(fuzzy_score("frs", "file.rs").is_some());
        assert!(fuzzy_score("xyz", "file.rs").is_none());
    }

    #[test]
    fn test_find_by_prefix() {
        let mut index = FileIndex::new();
        index.add(IndexEntry {
            path: PathBuf::from("/test/file.rs"),
            name: "file.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 100,
            modified: None,
        });
        index.add(IndexEntry {
            path: PathBuf::from("/test/foo.rs"),
            name: "foo.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 50,
            modified: None,
        });

        let results = index.find_by_prefix("f");
        assert_eq!(results.len(), 2);

        let results = index.find_by_prefix("fo");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_file_index_remove() {
        let mut index = FileIndex::new();
        let path = PathBuf::from("/test/file.rs");
        index.add(IndexEntry {
            path: path.clone(),
            name: "file.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 100,
            modified: None,
        });
        assert_eq!(index.len(), 1);
        index.remove(&path);
        assert_eq!(index.len(), 0);
    }

    #[test]
    fn test_file_index_is_empty() {
        let index = FileIndex::new();
        assert!(index.is_empty());
    }

    #[test]
    fn test_find_fuzzy() {
        let mut index = FileIndex::new();
        index.add(IndexEntry {
            path: PathBuf::from("/test/main.rs"),
            name: "main.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 100,
            modified: None,
        });
        let results = index.find_fuzzy("mr");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_index_entry_clone() {
        let entry = IndexEntry {
            path: PathBuf::from("/test/file.rs"),
            name: "file.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 100,
            modified: None,
        };
        let cloned = entry.clone();
        assert_eq!(entry.name, cloned.name);
    }

    #[test]
    fn test_index_service_default() {
        let service = IndexService::default();
        assert_eq!(service.name(), "index");
    }

    #[test]
    fn test_file_index_new_is_empty() {
        let index = FileIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
    }

    #[test]
    fn test_fuzzy_score_exact_match() {
        let score = fuzzy_score("file.rs", "file.rs");
        assert!(score.is_some());
        assert!(score.unwrap() > 0);
    }

    #[test]
    fn test_fuzzy_score_partial() {
        let score = fuzzy_score("file", "file.rs");
        assert!(score.is_some());
    }

    #[test]
    fn test_find_by_prefix_case() {
        let mut index = FileIndex::new();
        index.add(IndexEntry {
            path: PathBuf::from("/test/File.rs"),
            name: "File.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 100,
            modified: None,
        });
        // Case-insensitive prefix search
        let results = index.find_by_prefix("file");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_index_entry_no_extension() {
        let entry = IndexEntry {
            path: PathBuf::from("/test/Makefile"),
            name: "Makefile".to_string(),
            extension: None,
            size: 50,
            modified: None,
        };
        assert!(entry.extension.is_none());
    }

    #[test]
    fn test_file_index_multiple_remove() {
        let mut index = FileIndex::new();
        let path1 = PathBuf::from("/test/a.rs");
        let path2 = PathBuf::from("/test/b.rs");
        
        index.add(IndexEntry {
            path: path1.clone(),
            name: "a.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 10,
            modified: None,
        });
        index.add(IndexEntry {
            path: path2.clone(),
            name: "b.rs".to_string(),
            extension: Some("rs".to_string()),
            size: 20,
            modified: None,
        });
        
        assert_eq!(index.len(), 2);
        index.remove(&path1);
        assert_eq!(index.len(), 1);
        index.remove(&path2);
        assert_eq!(index.len(), 0);
    }
}
