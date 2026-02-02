//! Buffer change tracking.
//!
//! Tracks modified state and change history.

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

/// Buffer change tracker.
#[derive(Debug, Clone)]
pub struct ChangeTracker {
    /// Current state.
    pub state: ChangeState,
    /// Tick at last save.
    saved_tick: u64,
    /// Current tick.
    current_tick: u64,
    /// Last modified time.
    pub modified_time: Option<SystemTime>,
    /// Number of changes since save.
    change_count: usize,
}

impl Default for ChangeTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeTracker {
    /// Creates new tracker.
    pub fn new() -> Self {
        Self {
            state: ChangeState::Saved,
            saved_tick: 0,
            current_tick: 0,
            modified_time: None,
            change_count: 0,
        }
    }

    /// Records a change.
    pub fn record_change(&mut self) {
        self.current_tick += 1;
        self.change_count += 1;
        self.modified_time = Some(SystemTime::now());
        self.state = ChangeState::Modified;
    }

    /// Marks as saved.
    pub fn mark_saved(&mut self) {
        self.saved_tick = self.current_tick;
        self.change_count = 0;
        self.state = ChangeState::Saved;
    }

    /// Marks as recovered.
    pub fn mark_recovered(&mut self) {
        self.state = ChangeState::Recovered;
    }

    /// Returns whether modified.
    pub fn is_modified(&self) -> bool {
        self.current_tick != self.saved_tick
    }

    /// Returns the current tick.
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Returns changes since save.
    pub fn changes_since_save(&self) -> usize {
        self.change_count
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        *self = Self::new();
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
    fn test_change_tracker_new() {
        let ct = ChangeTracker::new();
        assert!(!ct.is_modified());
    }

    #[test]
    fn test_change_tracker_record() {
        let mut ct = ChangeTracker::new();
        ct.record_change();
        assert!(ct.is_modified());
        assert_eq!(ct.changes_since_save(), 1);
    }

    #[test]
    fn test_change_tracker_save() {
        let mut ct = ChangeTracker::new();
        ct.record_change();
        ct.mark_saved();
        assert!(!ct.is_modified());
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
