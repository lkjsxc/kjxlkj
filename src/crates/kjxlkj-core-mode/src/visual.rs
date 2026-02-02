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
}
