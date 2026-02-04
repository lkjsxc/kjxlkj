//! Individual buffer state.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Cursor};
use kjxlkj_core_undo::UndoHistory;

use crate::marks::MarkStore;

/// State for a single buffer.
#[derive(Debug)]
pub struct BufferState {
    /// Buffer identifier.
    pub id: BufferId,
    /// File path (if any).
    pub path: Option<String>,
    /// The text content.
    pub text: TextBuffer,
    /// Cursor position.
    pub cursor: Cursor,
    /// Undo history.
    pub undo: UndoHistory,
    /// Local marks.
    pub marks: MarkStore,
    /// Whether buffer is modified.
    pub modified: bool,
}

impl BufferState {
    /// Create a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            path: None,
            text: TextBuffer::new(),
            cursor: Cursor::origin(),
            undo: UndoHistory::new(),
            marks: MarkStore::new(),
            modified: false,
        }
    }

    /// Create a buffer from text.
    pub fn from_text(id: BufferId, text: &str) -> Self {
        Self {
            id,
            path: None,
            text: TextBuffer::from_str(text),
            cursor: Cursor::origin(),
            undo: UndoHistory::new(),
            marks: MarkStore::new(),
            modified: false,
        }
    }

    /// Create a buffer from a file path.
    pub fn from_file(id: BufferId, path: String, content: &str) -> Self {
        Self {
            id,
            path: Some(path),
            text: TextBuffer::from_str(content),
            cursor: Cursor::origin(),
            undo: UndoHistory::new(),
            marks: MarkStore::new(),
            modified: false,
        }
    }

    /// Clamp cursor to valid position for normal mode.
    pub fn clamp_cursor(&mut self) {
        self.cursor = self.text.clamp_cursor(self.cursor);
    }

    /// Clamp cursor for insert mode.
    pub fn clamp_cursor_insert(&mut self) {
        self.cursor = self.text.clamp_cursor_insert(self.cursor);
    }

    /// Get display name for status line.
    pub fn display_name(&self) -> &str {
        self.path.as_deref().unwrap_or("[No Name]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = BufferState::new(BufferId::new(1));
        assert!(buf.text.is_empty());
        assert!(!buf.modified);
    }

    #[test]
    fn test_from_text() {
        let buf = BufferState::from_text(BufferId::new(1), "hello\nworld");
        assert_eq!(buf.text.len_lines(), 2);
    }
}
