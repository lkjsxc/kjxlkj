//! Diff highlighting types.
//!
//! Types for representing diff markers and configuration.

/// Type of diff change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffKind {
    /// Line was added.
    Added,
    /// Line was modified.
    Changed,
    /// Line was deleted (shown as marker).
    Deleted,
    /// Top of deletion range.
    DeletedTop,
}

/// A diff marker for a line.
#[derive(Debug, Clone)]
pub struct DiffMarker {
    /// Line number (0-indexed).
    pub line: usize,
    /// Kind of change.
    pub kind: DiffKind,
    /// Number of lines in this hunk.
    pub count: usize,
}

impl DiffMarker {
    /// Creates a new diff marker.
    pub fn new(line: usize, kind: DiffKind) -> Self {
        Self {
            line,
            kind,
            count: 1,
        }
    }

    /// Sets the line count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }
}

/// Configuration for diff highlighting.
#[derive(Debug, Clone)]
pub struct DiffConfig {
    /// Whether diff highlighting is enabled.
    pub enabled: bool,
    /// Show count on delete markers.
    pub show_count: bool,
    /// Update on text change.
    pub update_on_change: bool,
    /// Debounce delay in milliseconds.
    pub debounce_ms: u32,
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_count: true,
            update_on_change: true,
            debounce_ms: 200,
        }
    }
}

/// Summary of diff changes.
#[derive(Debug, Clone, Default)]
pub struct DiffSummary {
    /// Added lines.
    pub added: usize,
    /// Changed lines.
    pub changed: usize,
    /// Deleted lines.
    pub deleted: usize,
}

impl DiffSummary {
    /// Gets total changes.
    pub fn total(&self) -> usize {
        self.added + self.changed + self.deleted
    }
}
