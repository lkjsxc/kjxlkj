//! Change tracking types.

use std::time::SystemTime;

/// Change tracking state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeState {
    /// No changes since save.
    Saved,
    /// Modified since last save.
    Modified,
    /// Modified and recovered from crash.
    Recovered,
}

impl Default for ChangeState {
    fn default() -> Self {
        Self::Saved
    }
}

/// External file change detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileChange {
    /// File unchanged.
    Unchanged,
    /// File modified externally.
    Modified,
    /// File deleted externally.
    Deleted,
    /// File permissions changed.
    PermissionChanged,
}

/// File modification tracker.
#[derive(Debug, Clone)]
pub struct FileTracker {
    /// Last known modification time.
    pub mtime: Option<SystemTime>,
    /// Last known file size.
    pub size: Option<u64>,
    /// Detected change.
    pub change: FileChange,
}

impl Default for FileTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl FileTracker {
    /// Creates new tracker.
    pub fn new() -> Self {
        Self {
            mtime: None,
            size: None,
            change: FileChange::Unchanged,
        }
    }

    /// Updates tracked state from file.
    pub fn update(&mut self, mtime: SystemTime, size: u64) {
        if let Some(old_mtime) = self.mtime {
            if mtime != old_mtime {
                self.change = FileChange::Modified;
            }
        }
        self.mtime = Some(mtime);
        self.size = Some(size);
    }

    /// Marks file as deleted.
    pub fn mark_deleted(&mut self) {
        self.change = FileChange::Deleted;
    }

    /// Clears change flag.
    pub fn clear_change(&mut self) {
        self.change = FileChange::Unchanged;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_state_default() {
        assert_eq!(ChangeState::default(), ChangeState::Saved);
    }

    #[test]
    fn test_file_tracker_new() {
        let ft = FileTracker::new();
        assert_eq!(ft.change, FileChange::Unchanged);
    }

    #[test]
    fn test_file_tracker_update() {
        let mut ft = FileTracker::new();
        let now = SystemTime::now();
        ft.update(now, 100);
        assert!(ft.mtime.is_some());
    }
}
