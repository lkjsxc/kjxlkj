//! Diff mode support.
//!
//! Provides diff viewing and navigation for buffers.

pub use crate::diff_types::{DiffHunk, DiffKind};

/// Diff state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct DiffState {
    hunks: Vec<DiffHunk>,
    current: usize,
    enabled: bool,
}

impl DiffState {
    /// Creates a new diff state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the hunks.
    pub fn set_hunks(&mut self, hunks: Vec<DiffHunk>) {
        self.hunks = hunks;
        self.current = 0;
    }

    /// Returns the hunks.
    pub fn hunks(&self) -> &[DiffHunk] {
        &self.hunks
    }

    /// Returns the current hunk.
    pub fn current_hunk(&self) -> Option<&DiffHunk> {
        self.hunks.get(self.current)
    }

    /// Returns the current hunk index.
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Moves to the next hunk.
    pub fn next_hunk(&mut self) -> Option<&DiffHunk> {
        if self.current + 1 < self.hunks.len() {
            self.current += 1;
        }
        self.current_hunk()
    }

    /// Moves to the previous hunk.
    pub fn prev_hunk(&mut self) -> Option<&DiffHunk> {
        if self.current > 0 {
            self.current -= 1;
        }
        self.current_hunk()
    }

    /// Enables diff mode.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables diff mode.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns whether diff mode is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Clears the diff state.
    pub fn clear(&mut self) {
        self.hunks.clear();
        self.current = 0;
    }

    /// Returns the hunk containing the given line (1-based).
    pub fn hunk_at_line(&self, line: usize) -> Option<&DiffHunk> {
        self.hunks
            .iter()
            .find(|h| line >= h.new_start && line < h.new_start + h.new_count.max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hunks() -> Vec<DiffHunk> {
        vec![
            DiffHunk::new(1, 0, 1, 2),
            DiffHunk::new(5, 3, 7, 3),
            DiffHunk::new(10, 2, 12, 0),
        ]
    }

    #[test]
    fn test_diff_state_navigation() {
        let mut state = DiffState::new();
        state.set_hunks(sample_hunks());
        assert_eq!(state.current_index(), 0);
        state.next_hunk();
        assert_eq!(state.current_index(), 1);
        state.prev_hunk();
        assert_eq!(state.current_index(), 0);
    }

    #[test]
    fn test_diff_state_hunk_at_line() {
        let mut state = DiffState::new();
        state.set_hunks(sample_hunks());
        let hunk = state.hunk_at_line(1).unwrap();
        assert_eq!(hunk.kind, DiffKind::Add);
    }

    #[test]
    fn test_diff_state_enable_disable() {
        let mut state = DiffState::new();
        assert!(!state.is_enabled());
        state.enable();
        assert!(state.is_enabled());
    }

    #[test]
    fn test_diff_state_clear() {
        let mut state = DiffState::new();
        state.set_hunks(sample_hunks());
        state.clear();
        assert!(state.hunks().is_empty());
    }
}
