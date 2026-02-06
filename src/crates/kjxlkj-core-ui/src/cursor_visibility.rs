//! Cursor visibility across mode transitions and redraws.
//!
//! Ensures cursor is always visible and matches the current mode shape/style.

/// Cursor display shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Line,
    Underline,
}

/// Cursor blink state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkState {
    Steady,
    Blinking,
}

/// Per-mode cursor configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeCursorConfig {
    pub shape: CursorShape,
    pub blink: BlinkState,
}

/// Default cursor configurations by mode name.
pub fn cursor_for_mode(mode: &str) -> ModeCursorConfig {
    match mode.to_lowercase().as_str() {
        "normal" => ModeCursorConfig { shape: CursorShape::Block, blink: BlinkState::Steady },
        "insert" => ModeCursorConfig { shape: CursorShape::Line, blink: BlinkState::Blinking },
        "visual" | "visual_line" | "visual_block" => {
            ModeCursorConfig { shape: CursorShape::Block, blink: BlinkState::Steady }
        }
        "replace" => ModeCursorConfig { shape: CursorShape::Underline, blink: BlinkState::Steady },
        "command" => ModeCursorConfig { shape: CursorShape::Line, blink: BlinkState::Blinking },
        _ => ModeCursorConfig { shape: CursorShape::Block, blink: BlinkState::Steady },
    }
}

/// Visibility check result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibilityCheck {
    pub visible: bool,
    pub reason: Option<String>,
}

impl VisibilityCheck {
    pub fn ok() -> Self { Self { visible: true, reason: None } }
    pub fn fail(reason: &str) -> Self { Self { visible: false, reason: Some(reason.into()) } }
}

/// Check that the cursor is within viewport bounds.
pub fn check_cursor_in_viewport(
    cursor_line: usize,
    cursor_col: usize,
    top_line: usize,
    left_col: usize,
    viewport_rows: usize,
    viewport_cols: usize,
) -> VisibilityCheck {
    if cursor_line < top_line {
        return VisibilityCheck::fail("cursor above viewport");
    }
    if cursor_line >= top_line + viewport_rows {
        return VisibilityCheck::fail("cursor below viewport");
    }
    if cursor_col < left_col {
        return VisibilityCheck::fail("cursor left of viewport");
    }
    if cursor_col >= left_col + viewport_cols {
        return VisibilityCheck::fail("cursor right of viewport");
    }
    VisibilityCheck::ok()
}

/// Check cursor visibility across a mode transition.
pub fn check_transition_visibility(
    from_mode: &str,
    to_mode: &str,
    cursor_line: usize,
    cursor_col: usize,
    top_line: usize,
    left_col: usize,
    viewport_rows: usize,
    viewport_cols: usize,
) -> VisibilityCheck {
    let _from_cfg = cursor_for_mode(from_mode);
    let _to_cfg = cursor_for_mode(to_mode);
    // After transition, cursor must still be visible
    check_cursor_in_viewport(cursor_line, cursor_col, top_line, left_col, viewport_rows, viewport_cols)
}

/// Emit ANSI escape for cursor shape change (for crossterm integration).
pub fn cursor_shape_escape(shape: CursorShape, blink: BlinkState) -> &'static str {
    match (shape, blink) {
        (CursorShape::Block, BlinkState::Blinking) => "\x1b[1 q",
        (CursorShape::Block, BlinkState::Steady) => "\x1b[2 q",
        (CursorShape::Underline, BlinkState::Blinking) => "\x1b[3 q",
        (CursorShape::Underline, BlinkState::Steady) => "\x1b[4 q",
        (CursorShape::Line, BlinkState::Blinking) => "\x1b[5 q",
        (CursorShape::Line, BlinkState::Steady) => "\x1b[6 q",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_mode_block() {
        let cfg = cursor_for_mode("normal");
        assert_eq!(cfg.shape, CursorShape::Block);
        assert_eq!(cfg.blink, BlinkState::Steady);
    }

    #[test]
    fn insert_mode_line() {
        let cfg = cursor_for_mode("insert");
        assert_eq!(cfg.shape, CursorShape::Line);
        assert_eq!(cfg.blink, BlinkState::Blinking);
    }

    #[test]
    fn replace_mode_underline() {
        let cfg = cursor_for_mode("replace");
        assert_eq!(cfg.shape, CursorShape::Underline);
    }

    #[test]
    fn cursor_visible_in_viewport() {
        let check = check_cursor_in_viewport(10, 5, 5, 0, 20, 80);
        assert!(check.visible);
    }

    #[test]
    fn cursor_above_viewport() {
        let check = check_cursor_in_viewport(3, 5, 5, 0, 20, 80);
        assert!(!check.visible);
        assert!(check.reason.unwrap().contains("above"));
    }

    #[test]
    fn cursor_below_viewport() {
        let check = check_cursor_in_viewport(30, 5, 5, 0, 20, 80);
        assert!(!check.visible);
        assert!(check.reason.unwrap().contains("below"));
    }

    #[test]
    fn transition_visibility() {
        let check = check_transition_visibility(
            "normal", "insert", 10, 5, 5, 0, 20, 80,
        );
        assert!(check.visible);
    }

    #[test]
    fn cursor_escape_sequences() {
        assert_eq!(cursor_shape_escape(CursorShape::Block, BlinkState::Steady), "\x1b[2 q");
        assert_eq!(cursor_shape_escape(CursorShape::Line, BlinkState::Blinking), "\x1b[5 q");
    }
}
