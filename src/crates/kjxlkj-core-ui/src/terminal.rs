//! Terminal emulation basics.
//!
//! Provides basic terminal state for embedded terminals.

pub use crate::terminal_buffer::{TermBuffer, TermCell};

/// Terminal size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermSize {
    /// Width in columns.
    pub cols: usize,
    /// Height in rows.
    pub rows: usize,
}

impl TermSize {
    /// Creates a new terminal size.
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { cols, rows }
    }
}

impl Default for TermSize {
    fn default() -> Self {
        Self { cols: 80, rows: 24 }
    }
}

/// Terminal cursor state.
#[derive(Debug, Clone, Copy, Default)]
pub struct TermCursor {
    /// Row (0-based).
    pub row: usize,
    /// Column (0-based).
    pub col: usize,
    /// Whether cursor is visible.
    pub visible: bool,
}

impl TermCursor {
    /// Creates a new terminal cursor.
    pub fn new() -> Self {
        Self {
            visible: true,
            ..Default::default()
        }
    }

    /// Moves to position.
    pub fn move_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
}

/// Terminal state.
#[derive(Debug, Clone)]
pub struct TermState {
    /// Buffer.
    pub buffer: TermBuffer,
    /// Cursor.
    pub cursor: TermCursor,
    /// Title.
    pub title: String,
    /// Whether terminal is active.
    pub active: bool,
}

impl TermState {
    /// Creates a new terminal state.
    pub fn new(size: TermSize) -> Self {
        Self {
            buffer: TermBuffer::new(size),
            cursor: TermCursor::new(),
            title: String::new(),
            active: false,
        }
    }

    /// Sets the title.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_size() {
        let size = TermSize::new(120, 40);
        assert_eq!(size.cols, 120);
        assert_eq!(size.rows, 40);
    }

    #[test]
    fn test_term_cursor() {
        let mut cursor = TermCursor::new();
        cursor.move_to(10, 20);
        assert_eq!(cursor.row, 10);
        assert_eq!(cursor.col, 20);
    }

    #[test]
    fn test_term_state() {
        let mut state = TermState::new(TermSize::default());
        state.set_title("bash");
        assert_eq!(state.title, "bash");
    }
}
