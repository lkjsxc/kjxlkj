//! Visual mode state.

use kjxlkj_core_types::{Mode, Position, Range};
use serde::{Deserialize, Serialize};

/// Visual mode state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualState {
    /// Anchor position (selection start).
    pub anchor: Option<Position>,
    /// Current position (selection end).
    pub cursor: Option<Position>,
    /// Visual mode type.
    pub mode: Option<Mode>,
}

impl VisualState {
    /// Creates a new visual mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the state.
    pub fn reset(&mut self) {
        self.anchor = None;
        self.cursor = None;
        self.mode = None;
    }

    /// Starts selection at a position with a mode.
    pub fn start(&mut self, pos: Position, mode: Mode) {
        self.anchor = Some(pos);
        self.cursor = Some(pos);
        self.mode = Some(mode);
    }

    /// Updates the cursor position.
    pub fn update_cursor(&mut self, pos: Position) {
        self.cursor = Some(pos);
    }

    /// Returns the selection range (normalized).
    pub fn range(&self) -> Option<Range> {
        match (self.anchor, self.cursor) {
            (Some(anchor), Some(cursor)) => {
                Some(Range::new(anchor, cursor).normalized())
            }
            _ => None,
        }
    }

    /// Returns true if position is within selection.
    pub fn contains(&self, pos: Position) -> bool {
        self.range().map(|r| r.contains(pos)).unwrap_or(false)
    }

    /// Returns selected lines for visual line mode.
    pub fn selected_lines(&self) -> Option<(usize, usize)> {
        match (self.anchor, self.cursor) {
            (Some(anchor), Some(cursor)) => {
                let start = anchor.line.min(cursor.line);
                let end = anchor.line.max(cursor.line);
                Some((start, end))
            }
            _ => None,
        }
    }

    /// Returns true if in visual line mode.
    pub fn is_linewise(&self) -> bool {
        matches!(self.mode, Some(Mode::VisualLine))
    }

    /// Returns true if in visual block mode.
    pub fn is_blockwise(&self) -> bool {
        matches!(self.mode, Some(Mode::VisualBlock))
    }

    /// Returns block corners (start_line, end_line, left_col, right_col).
    pub fn block_corners(&self) -> Option<(usize, usize, usize, usize)> {
        match (self.anchor, self.cursor) {
            (Some(anchor), Some(cursor)) => {
                let start_line = anchor.line.min(cursor.line);
                let end_line = anchor.line.max(cursor.line);
                let left_col = anchor.col.min(cursor.col);
                let right_col = anchor.col.max(cursor.col);
                Some((start_line, end_line, left_col, right_col))
            }
            _ => None,
        }
    }

    /// Returns selected column range for block mode.
    pub fn block_columns(&self) -> Option<(usize, usize)> {
        self.block_corners().map(|(_, _, left, right)| (left, right))
    }

    /// Swaps anchor and cursor positions.
    pub fn swap_ends(&mut self) {
        std::mem::swap(&mut self.anchor, &mut self.cursor);
    }

    /// Returns the number of selected lines.
    pub fn line_count(&self) -> usize {
        self.selected_lines()
            .map(|(start, end)| end - start + 1)
            .unwrap_or(0)
    }

    /// Returns whether selection is reversed (cursor before anchor).
    pub fn is_reversed(&self) -> bool {
        match (self.anchor, self.cursor) {
            (Some(anchor), Some(cursor)) => cursor < anchor,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_start() {
        let mut state = VisualState::new();
        state.start(Position::new(1, 5), Mode::Visual);
        assert_eq!(state.anchor, Some(Position::new(1, 5)));
        assert_eq!(state.cursor, Some(Position::new(1, 5)));
    }

    #[test]
    fn test_visual_range() {
        let mut state = VisualState::new();
        state.start(Position::new(1, 0), Mode::Visual);
        state.update_cursor(Position::new(3, 5));
        let range = state.range().unwrap();
        assert_eq!(range.start, Position::new(1, 0));
        assert_eq!(range.end, Position::new(3, 5));
    }

    #[test]
    fn test_selected_lines() {
        let mut state = VisualState::new();
        state.start(Position::new(5, 0), Mode::VisualLine);
        state.update_cursor(Position::new(2, 0));
        let (start, end) = state.selected_lines().unwrap();
        assert_eq!(start, 2);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_is_linewise() {
        let mut state = VisualState::new();
        state.start(Position::new(0, 0), Mode::VisualLine);
        assert!(state.is_linewise());
        assert!(!state.is_blockwise());
    }

    #[test]
    fn test_block_corners() {
        let mut state = VisualState::new();
        state.start(Position::new(5, 10), Mode::VisualBlock);
        state.update_cursor(Position::new(2, 5));
        let (sl, el, lc, rc) = state.block_corners().unwrap();
        assert_eq!(sl, 2);
        assert_eq!(el, 5);
        assert_eq!(lc, 5);
        assert_eq!(rc, 10);
    }

    #[test]
    fn test_block_columns() {
        let mut state = VisualState::new();
        state.start(Position::new(0, 3), Mode::VisualBlock);
        state.update_cursor(Position::new(0, 8));
        let (left, right) = state.block_columns().unwrap();
        assert_eq!(left, 3);
        assert_eq!(right, 8);
    }

    #[test]
    fn test_line_count() {
        let mut state = VisualState::new();
        state.start(Position::new(3, 0), Mode::VisualLine);
        state.update_cursor(Position::new(7, 0));
        assert_eq!(state.line_count(), 5);
    }

    #[test]
    fn test_is_reversed() {
        let mut state = VisualState::new();
        state.start(Position::new(5, 0), Mode::Visual);
        state.update_cursor(Position::new(2, 0));
        assert!(state.is_reversed());
    }
}
