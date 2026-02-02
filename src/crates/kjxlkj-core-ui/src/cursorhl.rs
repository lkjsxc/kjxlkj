//! Cursor line and column tracking.
//!
//! State for cursorline and cursorcolumn options.

/// Cursor line highlight mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorLineMode {
    /// No highlight.
    #[default]
    None,
    /// Highlight the line number only.
    Number,
    /// Highlight the entire line.
    Line,
    /// Highlight both number and line.
    Both,
}

/// Cursor column highlight mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorColumnMode {
    /// No highlight.
    #[default]
    None,
    /// Highlight the column.
    Column,
}

/// Cursor highlight state.
#[derive(Debug, Clone, Default)]
pub struct CursorHighlight {
    /// Line highlight mode.
    pub line_mode: CursorLineMode,
    /// Column highlight mode.
    pub column_mode: CursorColumnMode,
    /// Current cursor line (0-indexed).
    pub line: usize,
    /// Current cursor column (0-indexed).
    pub column: usize,
}

impl CursorHighlight {
    /// Creates new cursor highlight state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets cursor line highlighting.
    pub fn set_cursorline(&mut self, enabled: bool) {
        self.line_mode = if enabled {
            CursorLineMode::Line
        } else {
            CursorLineMode::None
        };
    }

    /// Sets cursor column highlighting.
    pub fn set_cursorcolumn(&mut self, enabled: bool) {
        self.column_mode = if enabled {
            CursorColumnMode::Column
        } else {
            CursorColumnMode::None
        };
    }

    /// Updates cursor position.
    pub fn set_position(&mut self, line: usize, column: usize) {
        self.line = line;
        self.column = column;
    }

    /// Returns whether a line should be highlighted.
    pub fn is_line_highlighted(&self, line: usize) -> bool {
        matches!(
            self.line_mode,
            CursorLineMode::Line | CursorLineMode::Both
        ) && line == self.line
    }

    /// Returns whether a column should be highlighted.
    pub fn is_column_highlighted(&self, column: usize) -> bool {
        self.column_mode == CursorColumnMode::Column && column == self.column
    }

    /// Returns whether line number should be highlighted.
    pub fn is_number_highlighted(&self, line: usize) -> bool {
        matches!(
            self.line_mode,
            CursorLineMode::Number | CursorLineMode::Both
        ) && line == self.line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_line_mode_default() {
        assert_eq!(CursorLineMode::default(), CursorLineMode::None);
    }

    #[test]
    fn test_cursor_highlight_new() {
        let ch = CursorHighlight::new();
        assert_eq!(ch.line_mode, CursorLineMode::None);
    }

    #[test]
    fn test_cursor_highlight_set_cursorline() {
        let mut ch = CursorHighlight::new();
        ch.set_cursorline(true);
        assert_eq!(ch.line_mode, CursorLineMode::Line);
    }

    #[test]
    fn test_cursor_highlight_is_line_highlighted() {
        let mut ch = CursorHighlight::new();
        ch.set_cursorline(true);
        ch.set_position(5, 0);
        assert!(ch.is_line_highlighted(5));
        assert!(!ch.is_line_highlighted(4));
    }

    #[test]
    fn test_cursor_highlight_is_column_highlighted() {
        let mut ch = CursorHighlight::new();
        ch.set_cursorcolumn(true);
        ch.set_position(0, 10);
        assert!(ch.is_column_highlighted(10));
    }

    #[test]
    fn test_cursor_highlight_position() {
        let mut ch = CursorHighlight::new();
        ch.set_position(100, 50);
        assert_eq!(ch.line, 100);
        assert_eq!(ch.column, 50);
    }
}
