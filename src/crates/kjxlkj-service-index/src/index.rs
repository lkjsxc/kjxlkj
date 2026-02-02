//! File index.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// An entry in the file index.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    /// Full path.
    pub path: PathBuf,
    /// File name.
    pub name: String,
    /// Relative path from root.
    pub relative: String,
    /// File extension.
    pub extension: Option<String>,
    /// Is directory.
    pub is_dir: bool,
}

impl IndexEntry {
    /// Creates a new index entry.
    pub fn new(path: PathBuf, root: &Path) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();

        let relative = path
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| path.to_string_lossy().into_owned());

        let extension = path
            .extension()
            .map(|s| s.to_string_lossy().into_owned());

        let is_dir = path.is_dir();

        Self {
            path,
            name,
            relative,
            extension,
            is_dir,
        }
    }
}

/// File index for fast lookups.
#[derive(Debug, Default)]
pub struct FileIndex {
    /// Root directory.
    root: PathBuf,
    /// All indexed entries.
    entries: Vec<IndexEntry>,
    /// Index by name for quick lookup.
    by_name: HashMap<String, Vec<usize>>,
}

impl FileIndex {
    /// Creates a new file index.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            entries: Vec::new(),
            by_name: HashMap::new(),
        }
    }

    /// Returns the root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Adds an entry to the index.
    pub fn add(&mut self, path: PathBuf) {
        let entry = IndexEntry::new(path, &self.root);
        let name = entry.name.to_lowercase();
        let idx = self.entries.len();
        self.entries.push(entry);
        self.by_name.entry(name).or_default().push(idx);
    }

    /// Returns all entries.
    pub fn entries(&self) -> &[IndexEntry] {
        &self.entries
    }

    /// Finds entries by exact name.
    pub fn find_by_name(&self, name: &str) -> Vec<&IndexEntry> {
        self.by_name
            .get(&name.to_lowercase())
            .map(|indices| indices.iter().map(|&i| &self.entries[i]).collect())
            .unwrap_or_default()
    }

    /// Clears the index.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.by_name.clear();
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
