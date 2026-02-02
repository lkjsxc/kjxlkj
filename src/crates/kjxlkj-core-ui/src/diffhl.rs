//! Diff highlighting for kjxlkj editor.
//!
//! Highlights added, changed, and removed lines in the gutter.

use std::collections::BTreeMap;

pub use crate::diffhl_types::{DiffConfig, DiffKind, DiffMarker, DiffSummary};

/// Tracks diff state for a buffer.
#[derive(Debug, Default)]
pub struct DiffState {
    /// Markers by line number.
    markers: BTreeMap<usize, DiffMarker>,
    /// Whether diff is being computed.
    computing: bool,
    /// Whether diff is stale.
    stale: bool,
}

impl DiffState {
    /// Creates a new diff state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the diff markers.
    pub fn set_markers(&mut self, markers: Vec<DiffMarker>) {
        self.markers.clear();
        for marker in markers {
            self.markers.insert(marker.line, marker);
        }
        self.computing = false;
        self.stale = false;
    }

    /// Gets marker for a line.
    pub fn marker(&self, line: usize) -> Option<&DiffMarker> {
        self.markers.get(&line)
    }

    /// Gets all markers.
    pub fn markers(&self) -> impl Iterator<Item = &DiffMarker> {
        self.markers.values()
    }

    /// Gets marker count.
    pub fn len(&self) -> usize {
        self.markers.len()
    }

    /// Checks if empty.
    pub fn is_empty(&self) -> bool {
        self.markers.is_empty()
    }

    /// Marks diff as stale.
    pub fn mark_stale(&mut self) {
        self.stale = true;
    }

    /// Checks if stale.
    pub fn is_stale(&self) -> bool {
        self.stale
    }

    /// Marks as computing.
    pub fn set_computing(&mut self, computing: bool) {
        self.computing = computing;
    }

    /// Checks if computing.
    pub fn is_computing(&self) -> bool {
        self.computing
    }

    /// Clears all markers.
    pub fn clear(&mut self) {
        self.markers.clear();
        self.stale = false;
    }

    /// Counts by kind.
    pub fn count_by_kind(&self, kind: DiffKind) -> usize {
        self.markers.values().filter(|m| m.kind == kind).count()
    }
}

impl DiffSummary {
    /// Creates summary from state.
    pub fn from_state(state: &DiffState) -> Self {
        Self {
            added: state.count_by_kind(DiffKind::Added),
            changed: state.count_by_kind(DiffKind::Changed),
            deleted: state.count_by_kind(DiffKind::Deleted)
                + state.count_by_kind(DiffKind::DeletedTop),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_kind() {
        assert_eq!(DiffKind::Added, DiffKind::Added);
        assert_ne!(DiffKind::Added, DiffKind::Deleted);
    }

    #[test]
    fn test_diff_marker() {
        let marker = DiffMarker::new(10, DiffKind::Changed).with_count(3);
        assert_eq!(marker.line, 10);
        assert_eq!(marker.kind, DiffKind::Changed);
        assert_eq!(marker.count, 3);
    }

    #[test]
    fn test_diff_config_default() {
        let config = DiffConfig::default();
        assert!(config.enabled);
        assert!(config.show_count);
        assert_eq!(config.debounce_ms, 200);
    }

    #[test]
    fn test_diff_state() {
        let mut state = DiffState::new();
        state.set_markers(vec![
            DiffMarker::new(5, DiffKind::Added),
            DiffMarker::new(10, DiffKind::Changed),
        ]);
        assert_eq!(state.len(), 2);
        assert!(state.marker(5).is_some());
    }

    #[test]
    fn test_diff_state_stale() {
        let mut state = DiffState::new();
        assert!(!state.is_stale());
        state.mark_stale();
        assert!(state.is_stale());
    }

    #[test]
    fn test_diff_summary() {
        let mut state = DiffState::new();
        state.set_markers(vec![
            DiffMarker::new(1, DiffKind::Added),
            DiffMarker::new(2, DiffKind::Added),
            DiffMarker::new(5, DiffKind::Changed),
            DiffMarker::new(10, DiffKind::Deleted),
        ]);
        let summary = DiffSummary::from_state(&state);
        assert_eq!(summary.added, 2);
        assert_eq!(summary.changed, 1);
        assert_eq!(summary.deleted, 1);
        assert_eq!(summary.total(), 4);
    }
}
