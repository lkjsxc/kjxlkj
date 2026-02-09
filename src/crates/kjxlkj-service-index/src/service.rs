//! Index service: manages file index and search.

use std::path::PathBuf;

use tokio::sync::{broadcast, mpsc};

use kjxlkj_core_types::ServiceResponse;

use crate::fuzzy;
use crate::scanner;

/// Index service for file and symbol searching.
pub struct IndexService {
    response_tx: mpsc::Sender<ServiceResponse>,
    /// Indexed file paths (relative).
    files: Vec<String>,
    /// Root directory.
    root: Option<PathBuf>,
}

impl IndexService {
    pub fn new(response_tx: mpsc::Sender<ServiceResponse>) -> Self {
        Self {
            response_tx,
            files: Vec::new(),
            root: None,
        }
    }

    /// Build the file index for a directory.
    pub fn index_directory(&mut self, root: PathBuf) {
        let paths = scanner::scan_directory(&root);
        self.files = paths
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        self.root = Some(root);
    }

    /// Search files using fuzzy matching.
    pub fn search(&self, query: &str) -> Vec<(String, i32)> {
        fuzzy::fuzzy_sort(query, &self.files)
    }

    /// Get all indexed files.
    pub fn all_files(&self) -> &[String] {
        &self.files
    }

    /// Run the service loop.
    pub async fn run(self, mut quit_rx: broadcast::Receiver<()>) {
        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                _ = tokio::time::sleep(
                    std::time::Duration::from_secs(3600)
                ) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_empty_index() {
        let (tx, _rx) = mpsc::channel(256);
        let svc = IndexService::new(tx);
        assert!(svc.search("test").is_empty());
    }
}
