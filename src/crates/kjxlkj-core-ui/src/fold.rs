//! Fold state management.
//!
//! Code folding support.

use std::collections::BTreeMap;

/// Fold method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FoldMethod {
    /// Manual folding.
    #[default]
    Manual,
    /// Indent-based folding.
    Indent,
    /// Expression-based folding.
    Expr,
    /// Syntax-based folding.
    Syntax,
    /// Marker-based folding.
    Marker,
    /// Diff mode folding.
    Diff,
}

/// A fold region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fold {
    /// Starting line (0-indexed).
    pub start: usize,
    /// Ending line (inclusive, 0-indexed).
    pub end: usize,
    /// Fold level.
    pub level: usize,
    /// Whether closed.
    pub closed: bool,
}

impl Fold {
    /// Creates a new fold.
    pub fn new(start: usize, end: usize, level: usize) -> Self {
        Self {
            start,
            end,
            level,
            closed: true,
        }
    }

    /// Returns the number of lines in this fold.
    pub fn line_count(&self) -> usize {
        self.end.saturating_sub(self.start) + 1
    }

    /// Returns whether a line is in this fold.
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }

    /// Opens the fold.
    pub fn open(&mut self) {
        self.closed = false;
    }

    /// Closes the fold.
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Toggles the fold.
    pub fn toggle(&mut self) {
        self.closed = !self.closed;
    }
}

/// Fold state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct FoldState {
    /// Folds by start line.
    folds: BTreeMap<usize, Fold>,
    /// Fold method.
    pub method: FoldMethod,
    /// Fold level.
    pub fold_level: usize,
}

impl FoldState {
    /// Creates a new fold state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a fold.
    pub fn add(&mut self, fold: Fold) {
        self.folds.insert(fold.start, fold);
    }

    /// Removes a fold at line.
    pub fn remove(&mut self, line: usize) -> Option<Fold> {
        self.folds.remove(&line)
    }

    /// Gets the fold at a line.
    pub fn get(&self, line: usize) -> Option<&Fold> {
        self.folds.get(&line)
    }

    /// Gets mutable fold at a line.
    pub fn get_mut(&mut self, line: usize) -> Option<&mut Fold> {
        self.folds.get_mut(&line)
    }

    /// Finds fold containing a line.
    pub fn find_containing(&self, line: usize) -> Option<&Fold> {
        self.folds.values().find(|f| f.contains(line))
    }

    /// Returns whether a line is hidden (inside closed fold).
    pub fn is_hidden(&self, line: usize) -> bool {
        self.folds
            .values()
            .any(|f| f.closed && line > f.start && line <= f.end)
    }

    /// Opens all folds.
    pub fn open_all(&mut self) {
        for fold in self.folds.values_mut() {
            fold.open();
        }
    }

    /// Closes all folds.
    pub fn close_all(&mut self) {
        for fold in self.folds.values_mut() {
            fold.close();
        }
    }

    /// Returns all folds.
    pub fn all(&self) -> Vec<&Fold> {
        self.folds.values().collect()
    }

    /// Clears all folds.
    pub fn clear(&mut self) {
        self.folds.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_new() {
        let fold = Fold::new(10, 20, 1);
        assert_eq!(fold.line_count(), 11);
    }

    #[test]
    fn test_fold_contains() {
        let fold = Fold::new(10, 20, 1);
        assert!(fold.contains(15));
        assert!(!fold.contains(25));
    }

    #[test]
    fn test_fold_toggle() {
        let mut fold = Fold::new(10, 20, 1);
        assert!(fold.closed);
        fold.toggle();
        assert!(!fold.closed);
    }

    #[test]
    fn test_fold_state_add() {
        let mut state = FoldState::new();
        state.add(Fold::new(10, 20, 1));
        assert!(state.get(10).is_some());
    }

    #[test]
    fn test_fold_state_is_hidden() {
        let mut state = FoldState::new();
        state.add(Fold::new(10, 20, 1));
        assert!(state.is_hidden(15));
        assert!(!state.is_hidden(10)); // First line not hidden
    }

    #[test]
    fn test_fold_state_open_all() {
        let mut state = FoldState::new();
        state.add(Fold::new(10, 20, 1));
        state.open_all();
        assert!(!state.is_hidden(15));
    }
}
