//! Diff mode support.
//!
//! Provides diff viewing and navigation for buffers.

/// A diff hunk.
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Start line in the old version (1-based).
    pub old_start: usize,
    /// Number of lines in old version.
    pub old_count: usize,
    /// Start line in the new version (1-based).
    pub new_start: usize,
    /// Number of lines in new version.
    pub new_count: usize,
    /// Hunk type.
    pub kind: DiffKind,
}

/// Kind of diff change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffKind {
    /// Lines added.
    Add,
    /// Lines deleted.
    Delete,
    /// Lines changed.
    Change,
}

impl DiffHunk {
    /// Creates a new diff hunk.
    pub fn new(old_start: usize, old_count: usize, new_start: usize, new_count: usize) -> Self {
        let kind = if old_count == 0 {
            DiffKind::Add
        } else if new_count == 0 {
            DiffKind::Delete
        } else {
            DiffKind::Change
        };

        Self {
            old_start,
            old_count,
            new_start,
            new_count,
            kind,
        }
    }

    /// Returns the hunk header in unified diff format.
    pub fn header(&self) -> String {
        format!(
            "@@ -{},{} +{},{} @@",
            self.old_start, self.old_count, self.new_start, self.new_count
        )
    }
}

/// Diff state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct DiffState {
    /// Hunks in the diff.
    hunks: Vec<DiffHunk>,
    /// Current hunk index.
    current: usize,
    /// Whether diff mode is enabled.
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
        self.hunks.iter().find(|h| {
            line >= h.new_start && line < h.new_start + h.new_count.max(1)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hunks() -> Vec<DiffHunk> {
        vec![
            DiffHunk::new(1, 0, 1, 2),   // Add
            DiffHunk::new(5, 3, 7, 3),   // Change
            DiffHunk::new(10, 2, 12, 0), // Delete
        ]
    }

    #[test]
    fn test_diff_hunk_kind() {
        let add = DiffHunk::new(1, 0, 1, 2);
        assert_eq!(add.kind, DiffKind::Add);

        let delete = DiffHunk::new(1, 2, 1, 0);
        assert_eq!(delete.kind, DiffKind::Delete);

        let change = DiffHunk::new(1, 2, 1, 3);
        assert_eq!(change.kind, DiffKind::Change);
    }

    #[test]
    fn test_diff_hunk_header() {
        let hunk = DiffHunk::new(1, 5, 1, 7);
        assert_eq!(hunk.header(), "@@ -1,5 +1,7 @@");
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

        let hunk = state.hunk_at_line(8).unwrap();
        assert_eq!(hunk.kind, DiffKind::Change);
    }

    #[test]
    fn test_diff_state_enable_disable() {
        let mut state = DiffState::new();
        assert!(!state.is_enabled());

        state.enable();
        assert!(state.is_enabled());

        state.disable();
        assert!(!state.is_enabled());
    }

    #[test]
    fn test_diff_state_clear() {
        let mut state = DiffState::new();
        state.set_hunks(sample_hunks());
        state.next_hunk();

        state.clear();
        assert!(state.hunks().is_empty());
        assert_eq!(state.current_index(), 0);
    }
}
