//! Scroll types.

/// Scroll direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Scroll up (content moves down).
    Up,
    /// Scroll down (content moves up).
    Down,
}

/// Scroll amount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAmount {
    /// Half page (Ctrl-D, Ctrl-U).
    HalfPage,
    /// Full page (Ctrl-F, Ctrl-B).
    FullPage,
    /// Single line (Ctrl-E, Ctrl-Y).
    Line,
    /// Multiple lines.
    Lines(usize),
}

/// Cursor position relative to viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorPosition {
    /// Center cursor in viewport (zz).
    Center,
    /// Move cursor to top of viewport (zt).
    Top,
    /// Move cursor to bottom of viewport (zb).
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_direction() {
        assert_ne!(ScrollDirection::Up, ScrollDirection::Down);
    }

    #[test]
    fn test_scroll_amount() {
        assert_eq!(ScrollAmount::Lines(5), ScrollAmount::Lines(5));
    }

    #[test]
    fn test_cursor_position() {
        assert_ne!(CursorPosition::Center, CursorPosition::Top);
    }
}
