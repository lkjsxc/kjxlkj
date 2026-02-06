//! Cursor visibility and rendering state for snapshot-based rendering.

use kjxlkj_core_types::{Mode, CursorShape, Position};

/// Complete cursor state for snapshot-based rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorState {
    pub position: Position,
    pub shape: CursorShape,
    pub visible: bool,
    pub blink: bool,
}

impl CursorState {
    /// Create cursor state from mode and position.
    pub fn from_mode(mode: Mode, position: Position) -> Self {
        Self {
            position,
            shape: mode.cursor_shape(),
            visible: true,
            blink: matches!(mode, Mode::Insert | Mode::Command),
        }
    }

    /// Hidden cursor (e.g., during rendering pause).
    pub fn hidden() -> Self {
        Self { position: Position::new(0, 0), shape: CursorShape::Block, visible: false, blink: false }
    }
}

/// Cursor rendering hint for the TUI layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorHint {
    /// Show cursor at the given screen position.
    Show { row: u16, col: u16, shape: CursorShape },
    /// Hide cursor (e.g., in a popup or during redraw).
    Hide,
}

impl CursorHint {
    /// Create a show hint from viewport-relative cursor position.
    pub fn from_viewport(cursor_line: usize, top_line: usize, cursor_col: usize, left_col: usize, shape: CursorShape) -> Self {
        let row = cursor_line.saturating_sub(top_line) as u16;
        let col = cursor_col.saturating_sub(left_col) as u16;
        Self::Show { row, col, shape }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_state_from_normal() {
        let cs = CursorState::from_mode(Mode::Normal, Position::new(5, 10));
        assert_eq!(cs.shape, CursorShape::Block);
        assert!(cs.visible);
        assert!(!cs.blink);
    }

    #[test]
    fn cursor_state_from_insert() {
        let cs = CursorState::from_mode(Mode::Insert, Position::new(0, 0));
        assert_eq!(cs.shape, CursorShape::Bar);
        assert!(cs.blink);
    }

    #[test]
    fn cursor_state_from_replace() {
        let cs = CursorState::from_mode(Mode::Replace, Position::new(1, 3));
        assert_eq!(cs.shape, CursorShape::Underline);
        assert!(!cs.blink);
    }

    #[test]
    fn cursor_hidden() {
        let cs = CursorState::hidden();
        assert!(!cs.visible);
    }

    #[test]
    fn cursor_hint_from_viewport() {
        let hint = CursorHint::from_viewport(110, 100, 45, 0, CursorShape::Block);
        assert_eq!(hint, CursorHint::Show { row: 10, col: 45, shape: CursorShape::Block });
    }

    #[test]
    fn cursor_hint_with_horizontal_scroll() {
        let hint = CursorHint::from_viewport(50, 50, 100, 80, CursorShape::Bar);
        assert_eq!(hint, CursorHint::Show { row: 0, col: 20, shape: CursorShape::Bar });
    }
}
