//! Buffer change tracking.
//!
//! Tracks modified state and change history.

use std::time::SystemTime;

use crate::change_types::ChangeState;

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_change_tracker_reset() {
        let mut ct = ChangeTracker::new();
        ct.record_change();
        ct.reset();
        assert!(!ct.is_modified());
    }
}
