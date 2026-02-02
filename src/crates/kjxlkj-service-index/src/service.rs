//! Index service.

use crate::{Finder, Index};
use std::path::PathBuf;

/// Index service.
pub struct IndexService {
    /// File index.
    index: Index,
    /// File finder.
    finder: Finder,
    /// Root path.
    root: Option<PathBuf>,
}

impl IndexService {
    /// Creates a new index service.
    pub fn new() -> Self {
        Self {
            index: Index::new(),
            finder: Finder::new(),
            root: None,
        }
    }

    /// Sets the root path.
    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
    }

    /// Returns the index.
    pub fn index(&self) -> &Index {
        &self.index
    }

    /// Returns the finder.
    pub fn finder(&self) -> &Finder {
        &self.finder
    }

    /// Runs the service.
    pub async fn run(self) {
        // Service loop
    }
}

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}
