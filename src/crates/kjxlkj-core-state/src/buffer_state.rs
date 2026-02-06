//! Per-buffer state: text content, undo history, file association.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::BufferId;
use kjxlkj_core_undo::UndoTree;

/// Per-buffer state aggregating text, undo, and metadata.
pub struct BufferState {
    pub id: BufferId,
    pub text: TextBuffer,
    pub undo: UndoTree,
    pub file_path: Option<String>,
    pub modified: bool,
}

impl BufferState {
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            text: TextBuffer::new(),
            undo: UndoTree::new(),
            file_path: None,
            modified: false,
        }
    }

    pub fn from_text(id: BufferId, text: &str) -> Self {
        Self {
            id,
            text: TextBuffer::from_text(text),
            undo: UndoTree::new(),
            file_path: None,
            modified: false,
        }
    }

    /// Line count of the underlying text buffer.
    pub fn line_count(&self) -> usize {
        self.text.line_count()
    }

    /// Get a line's text content.
    pub fn line_text(&self, line: usize) -> Option<String> {
        if line < self.text.line_count() {
            Some(self.text.line_to_string(line))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_is_empty() {
        let buf = BufferState::new(BufferId(1));
        assert!(!buf.modified);
        assert!(buf.file_path.is_none());
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn buffer_from_text() {
        let buf = BufferState::from_text(BufferId(1), "hello\nworld");
        assert_eq!(buf.line_count(), 2);
    }
}
