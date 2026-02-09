//! File change watcher using the notify crate.

use std::path::PathBuf;
use std::sync::mpsc as std_mpsc;

use tokio::sync::mpsc;

use kjxlkj_core_types::ServiceResponse;

/// File watcher that monitors paths for changes.
pub struct FileWatcher {
    response_tx: mpsc::Sender<ServiceResponse>,
    watched_paths: Vec<PathBuf>,
}

impl FileWatcher {
    pub fn new(response_tx: mpsc::Sender<ServiceResponse>) -> Self {
        Self {
            response_tx,
            watched_paths: Vec::new(),
        }
    }

    /// Add a path to watch.
    pub fn watch(&mut self, path: PathBuf) {
        self.watched_paths.push(path);
    }

    /// Remove a path from watching.
    pub fn unwatch(&mut self, path: &std::path::Path) {
        self.watched_paths.retain(|p| p.as_path() != path);
    }

    /// Start watching (simplified: polls on interval).
    pub async fn run(self, mut quit_rx: tokio::sync::broadcast::Receiver<()>) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));

        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                _ = interval.tick() => {
                    // In production, use notify::RecommendedWatcher.
                    // This is a placeholder polling loop.
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watch_and_unwatch() {
        let (tx, _rx) = mpsc::channel(256);
        let mut watcher = FileWatcher::new(tx);
        watcher.watch(PathBuf::from("/tmp/test.txt"));
        assert_eq!(watcher.watched_paths.len(), 1);
        watcher.unwatch(std::path::Path::new("/tmp/test.txt"));
        assert_eq!(watcher.watched_paths.len(), 0);
    }
}
