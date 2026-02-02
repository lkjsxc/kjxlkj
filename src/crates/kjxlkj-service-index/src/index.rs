//! File index.

use std::path::PathBuf;
use std::collections::HashMap;

/// File index.
pub struct Index {
    /// Files by path.
    files: HashMap<PathBuf, FileEntry>,
}

/// Index entry for a file.
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File path.
    pub path: PathBuf,
    /// File size.
    pub size: u64,
    /// Last modified.
    pub modified: u64,
}

impl Index {
    /// Creates a new index.
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Adds a file.
    pub fn add(&mut self, entry: FileEntry) {
        self.files.insert(entry.path.clone(), entry);
    }

    /// Gets a file.
    pub fn get(&self, path: &PathBuf) -> Option<&FileEntry> {
        self.files.get(path)
    }

    /// Removes a file.
    pub fn remove(&mut self, path: &PathBuf) {
        self.files.remove(path);
    }

    /// Returns all files.
    pub fn files(&self) -> impl Iterator<Item = &FileEntry> {
        self.files.values()
    }

    /// Returns the file count.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}
