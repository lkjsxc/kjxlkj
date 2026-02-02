//! Conceal support.
//!
//! Handles character concealment and replacement.

use std::collections::HashMap;

use crate::conceal_types::{ConcealLevel, ConcealRegion, LineConceal};

/// Buffer conceal state.
#[derive(Debug, Clone, Default)]
pub struct ConcealState {
    /// Per-line conceal info.
    lines: HashMap<usize, LineConceal>,
    /// Global conceal level.
    level: ConcealLevel,
    /// Cursor conceal level (for cursor line).
    cursor_level: ConcealLevel,
}

impl ConcealState {
    /// Creates new conceal state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets conceal info for a line.
    pub fn line(&self, line: usize) -> Option<&LineConceal> {
        self.lines.get(&line)
    }

    /// Gets or creates conceal info for a line.
    pub fn line_mut(&mut self, line: usize) -> &mut LineConceal {
        self.lines.entry(line).or_default()
    }

    /// Adds a conceal region to a line.
    pub fn add_region(&mut self, line: usize, region: ConcealRegion) {
        self.line_mut(line).add(region);
    }

    /// Sets the global conceal level.
    pub fn set_level(&mut self, level: ConcealLevel) {
        self.level = level;
    }

    /// Returns the global conceal level.
    pub fn level(&self) -> ConcealLevel {
        self.level
    }

    /// Sets the cursor conceal level.
    pub fn set_cursor_level(&mut self, level: ConcealLevel) {
        self.cursor_level = level;
    }

    /// Returns the effective conceal level for a line.
    pub fn effective_level(&self, line: usize, cursor_line: usize) -> ConcealLevel {
        if line == cursor_line {
            self.cursor_level
        } else {
            self.level
        }
    }

    /// Clears all conceal state.
    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conceal_state_level() {
        let mut state = ConcealState::new();
        state.set_level(ConcealLevel::Replace);

        assert_eq!(state.level(), ConcealLevel::Replace);
    }

    #[test]
    fn test_conceal_state_effective_level() {
        let mut state = ConcealState::new();
        state.set_level(ConcealLevel::Hide);
        state.set_cursor_level(ConcealLevel::None);

        assert_eq!(state.effective_level(5, 5), ConcealLevel::None);
        assert_eq!(state.effective_level(3, 5), ConcealLevel::Hide);
    }

    #[test]
    fn test_conceal_state_add_region() {
        let mut state = ConcealState::new();
        state.add_region(10, ConcealRegion::new(0..5));

        assert!(state.line(10).is_some());
        assert!(state.line(10).unwrap().is_concealed(3));
    }
}
