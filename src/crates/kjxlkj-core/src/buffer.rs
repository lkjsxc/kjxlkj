//! Editor buffer combining text, cursor, and state.

mod edit;
mod io;

use kjxlkj_core_text::TextRope;
use kjxlkj_core_types::{BufferId, Cursor, Position};

/// An editor buffer with text and cursor state.
pub struct Buffer {
    /// Buffer ID.
    id: BufferId,
    /// The text content.
    text: TextRope,
    /// Path on disk (if any).
    path: Option<std::path::PathBuf>,
    /// Primary cursor.
    cursor: Cursor,
    /// Whether modified since last save.
    modified: bool,
}

impl Buffer {
    /// Creates a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            text: TextRope::new(),
            path: None,
            cursor: Cursor::new(Position::origin()),
            modified: false,
        }
    }

    /// Creates a buffer from text.
    pub fn from_text(id: BufferId, text: &str) -> Self {
        Self {
            id,
            text: TextRope::from_text(text),
            path: None,
            cursor: Cursor::new(Position::origin()),
            modified: false,
        }
    }

    /// Returns the buffer ID.
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Returns the file path.
    pub fn path(&self) -> Option<&std::path::Path> {
        self.path.as_deref()
    }

    /// Sets the file path.
    pub fn set_path(&mut self, path: std::path::PathBuf) {
        self.path = Some(path);
    }

    /// Returns whether modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Marks as saved.
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Returns total line count.
    pub fn line_count(&self) -> usize {
        self.text.line_count()
    }

    /// Returns a line's content.
    pub fn line(&self, line: usize) -> Option<String> {
        self.text.line(line)
    }

    /// Returns the length of a line (in chars).
    pub fn line_len(&self, line: usize) -> usize {
        self.text.line(line).map(|s| s.chars().count()).unwrap_or(0)
    }

    /// Returns the cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Returns cursor line.
    pub fn cursor_line(&self) -> usize {
        self.cursor.position().line.as_usize()
    }

    /// Returns cursor column.
    pub fn cursor_col(&self) -> usize {
        self.cursor.position().col.as_usize()
    }

    /// Moves cursor to position.
    pub fn move_cursor(&mut self, line: usize, col: usize) {
        let line = line.min(self.line_count().saturating_sub(1));
        let col = col.min(self.line_len(line).saturating_sub(1));
        self.cursor = Cursor::new(Position::new(line, col));
    }

    /// Returns the character at cursor position.
    pub fn current_char(&self) -> Option<char> {
        let line = self.cursor_line();
        let col = self.cursor_col();
        let char_idx = self.text.line_to_char(line) + col;
        self.text.char_at(char_idx)
    }

    /// Returns the full text content.
    pub fn text(&self) -> String {
        self.text.contents()
    }
}
