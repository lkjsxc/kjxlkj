//! Cursor visibility, viewport checks, and mode-based shape selection.

use kjxlkj_core_types::{Mode, Position};

use crate::snapshot::CursorShape;

/// Return the appropriate cursor shape for the given editor mode.
pub fn cursor_for_mode(mode: &Mode) -> CursorShape {
    match mode {
        Mode::Normal | Mode::Visual | Mode::VisualLine | Mode::VisualBlock => CursorShape::Block,
        Mode::Insert => CursorShape::Line,
        Mode::Replace => CursorShape::Underline,
        Mode::Command => CursorShape::Line,
        Mode::Terminal => CursorShape::Block,
        Mode::OperatorPending => CursorShape::Block,
    }
}

/// Check whether the cursor is within the visible viewport.
pub fn check_cursor_in_viewport(
    cursor: Position,
    top_line: usize,
    visible_lines: usize,
    left_col: usize,
    visible_cols: usize,
) -> bool {
    cursor.line >= top_line
        && cursor.line < top_line + visible_lines
        && cursor.col >= left_col
        && cursor.col < left_col + visible_cols
}

/// Check whether the cursor remains visible after a mode transition.
///
/// Returns `true` if the cursor is within the viewport defined by
/// `viewport_top` and `viewport_height`.
pub fn check_transition_visibility(
    cursor: Position,
    viewport_top: usize,
    viewport_height: usize,
) -> bool {
    cursor.line >= viewport_top && cursor.line < viewport_top + viewport_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_shapes_for_modes() {
        assert_eq!(cursor_for_mode(&Mode::Normal), CursorShape::Block);
        assert_eq!(cursor_for_mode(&Mode::Insert), CursorShape::Line);
        assert_eq!(cursor_for_mode(&Mode::Replace), CursorShape::Underline);
        assert_eq!(cursor_for_mode(&Mode::Visual), CursorShape::Block);
        assert_eq!(cursor_for_mode(&Mode::Command), CursorShape::Line);
    }

    #[test]
    fn in_viewport() {
        assert!(check_cursor_in_viewport(Position::new(5, 3), 0, 24, 0, 80));
        assert!(!check_cursor_in_viewport(
            Position::new(25, 0),
            0,
            24,
            0,
            80
        ));
        assert!(!check_cursor_in_viewport(
            Position::new(5, 81),
            0,
            24,
            0,
            80
        ));
    }

    #[test]
    fn transition_visible() {
        assert!(check_transition_visibility(Position::new(10, 0), 5, 20));
        assert!(!check_transition_visibility(Position::new(30, 0), 5, 20));
        assert!(!check_transition_visibility(Position::new(3, 0), 5, 20));
    }
}
