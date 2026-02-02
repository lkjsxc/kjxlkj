//! Fold storage and management.

use crate::fold_types::{Fold, FoldRange};
use std::collections::BTreeMap;

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

    #[test]
    fn test_folds_remove() {
        let mut folds = Folds::new();
        folds.add(FoldRange::new(5, 10), 0);
        folds.remove(5);
        assert!(folds.get(5).is_none());
    }

    #[test]
    fn test_folds_clear() {
        let mut folds = Folds::new();
        folds.add(FoldRange::new(0, 5), 0);
        folds.clear();
        assert_eq!(folds.count(), 0);
    }
}
