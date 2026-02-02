//! Fold types.

use serde::{Deserialize, Serialize};

/// A fold range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldRange {
    /// Start line (0-indexed).
    pub start: usize,
    /// End line (0-indexed, inclusive).
    pub end: usize,
}

impl FoldRange {
    /// Creates a new fold range.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the number of lines in the fold.
    pub fn line_count(&self) -> usize {
        self.end.saturating_sub(self.start) + 1
    }

    /// Returns if a line is within the fold.
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }
}

/// Fold state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoldState {
    /// Fold is open (content visible).
    Open,
    /// Fold is closed (content hidden).
    Closed,
}

impl Default for FoldState {
    fn default() -> Self {
        Self::Open
    }
}

/// A fold entry.
#[derive(Debug, Clone)]
pub struct Fold {
    /// Range of the fold.
    pub range: FoldRange,
    /// Current state.
    pub state: FoldState,
    /// Fold level (0 = top level).
    pub level: usize,
}

impl Fold {
    /// Creates a new fold.
    pub fn new(range: FoldRange, level: usize) -> Self {
        Self {
            range,
            state: FoldState::Open,
            level,
        }
    }

    /// Returns if the fold is closed.
    pub fn is_closed(&self) -> bool {
        self.state == FoldState::Closed
    }

    /// Opens the fold.
    pub fn open(&mut self) {
        self.state = FoldState::Open;
    }

    /// Closes the fold.
    pub fn close(&mut self) {
        self.state = FoldState::Closed;
    }

    /// Toggles the fold.
    pub fn toggle(&mut self) {
        self.state = match self.state {
            FoldState::Open => FoldState::Closed,
            FoldState::Closed => FoldState::Open,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_range() {
        let range = FoldRange::new(5, 10);
        assert_eq!(range.line_count(), 6);
        assert!(range.contains(7));
        assert!(!range.contains(11));
    }

    #[test]
    fn test_fold_toggle() {
        let mut fold = Fold::new(FoldRange::new(0, 5), 0);
        assert!(!fold.is_closed());
        fold.toggle();
        assert!(fold.is_closed());
        fold.toggle();
        assert!(!fold.is_closed());
    }

    #[test]
    fn test_fold_state_default() {
        assert_eq!(FoldState::default(), FoldState::Open);
    }

    #[test]
    fn test_fold_open_close() {
        let mut fold = Fold::new(FoldRange::new(0, 10), 1);
        fold.close();
        assert!(fold.is_closed());
        fold.open();
        assert!(!fold.is_closed());
    }
}
