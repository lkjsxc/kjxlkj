//! Code folding support.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

/// Fold storage for a buffer.
#[derive(Debug, Clone, Default)]
pub struct Folds {
    /// Folds by start line.
    folds: BTreeMap<usize, Fold>,
}

impl Folds {
    /// Creates empty fold storage.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a fold.
    pub fn add(&mut self, range: FoldRange, level: usize) {
        self.folds.insert(range.start, Fold::new(range, level));
    }

    /// Removes a fold at a line.
    pub fn remove(&mut self, line: usize) {
        self.folds.remove(&line);
    }

    /// Gets a fold at a line.
    pub fn get(&self, line: usize) -> Option<&Fold> {
        self.folds.get(&line)
    }

    /// Gets a mutable fold at a line.
    pub fn get_mut(&mut self, line: usize) -> Option<&mut Fold> {
        self.folds.get_mut(&line)
    }

    /// Finds the fold containing a line.
    pub fn find_containing(&self, line: usize) -> Option<&Fold> {
        for fold in self.folds.values() {
            if fold.range.contains(line) {
                return Some(fold);
            }
        }
        None
    }

    /// Returns if a line is hidden by a fold.
    pub fn is_hidden(&self, line: usize) -> bool {
        for fold in self.folds.values() {
            if fold.is_closed() && line > fold.range.start && line <= fold.range.end {
                return true;
            }
        }
        false
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

    /// Toggles a fold at a line.
    pub fn toggle(&mut self, line: usize) {
        if let Some(fold) = self.folds.get_mut(&line) {
            fold.toggle();
        } else if let Some(containing) = self.find_fold_start(line) {
            if let Some(fold) = self.folds.get_mut(&containing) {
                fold.toggle();
            }
        }
    }

    /// Finds the start line of the fold containing a line.
    fn find_fold_start(&self, line: usize) -> Option<usize> {
        for (start, fold) in &self.folds {
            if fold.range.contains(line) {
                return Some(*start);
            }
        }
        None
    }

    /// Returns all folds.
    pub fn iter(&self) -> impl Iterator<Item = &Fold> {
        self.folds.values()
    }

    /// Returns fold count.
    pub fn count(&self) -> usize {
        self.folds.len()
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
    fn test_folds_add_get() {
        let mut folds = Folds::new();
        folds.add(FoldRange::new(10, 20), 0);
        assert!(folds.get(10).is_some());
        assert!(folds.get(15).is_none());
    }

    #[test]
    fn test_folds_is_hidden() {
        let mut folds = Folds::new();
        folds.add(FoldRange::new(5, 10), 0);
        
        assert!(!folds.is_hidden(7)); // Open fold
        
        folds.get_mut(5).unwrap().close();
        assert!(folds.is_hidden(7)); // Closed fold
        assert!(!folds.is_hidden(5)); // Start line visible
    }

    #[test]
    fn test_folds_find_containing() {
        let mut folds = Folds::new();
        folds.add(FoldRange::new(10, 20), 0);
        
        assert!(folds.find_containing(15).is_some());
        assert!(folds.find_containing(25).is_none());
    }

    #[test]
    fn test_open_close_all() {
        let mut folds = Folds::new();
        folds.add(FoldRange::new(0, 5), 0);
        folds.add(FoldRange::new(10, 15), 0);
        
        folds.close_all();
        assert!(folds.get(0).unwrap().is_closed());
        assert!(folds.get(10).unwrap().is_closed());
        
        folds.open_all();
        assert!(!folds.get(0).unwrap().is_closed());
    }
}
