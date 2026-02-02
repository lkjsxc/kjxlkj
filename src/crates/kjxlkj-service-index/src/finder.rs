//! File finder.

use std::path::PathBuf;

/// File finder result.
#[derive(Debug, Clone)]
pub struct FinderResult {
    /// File path.
    pub path: PathBuf,
    /// Match score.
    pub score: i32,
}

/// Fuzzy file finder.
pub struct Finder {
    /// Files to search.
    files: Vec<PathBuf>,
}

impl Finder {
    /// Creates a new finder.
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Adds files.
    pub fn add_files(&mut self, files: impl IntoIterator<Item = PathBuf>) {
        self.files.extend(files);
    }

    /// Searches for files.
    pub fn search(&self, query: &str) -> Vec<FinderResult> {
        let query_lower = query.to_lowercase();
        self.files
            .iter()
            .filter_map(|path| {
                let name = path.file_name()?.to_str()?;
                if name.to_lowercase().contains(&query_lower) {
                    Some(FinderResult {
                        path: path.clone(),
                        score: 100,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Clears all files.
    pub fn clear(&mut self) {
        self.files.clear();
    }
}

impl Default for Finder {
    fn default() -> Self {
        Self::new()
    }
}
