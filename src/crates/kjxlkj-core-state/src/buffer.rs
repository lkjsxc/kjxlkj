//! Buffer state.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion};
use kjxlkj_core_undo::UndoHistory;
use std::path::PathBuf;

/// State of a single buffer.
pub struct BufferState {
    pub id: BufferId,
    pub name: BufferName,
    pub path: Option<PathBuf>,
    pub text: TextBuffer,
    pub history: UndoHistory,
    pub modified: bool,
}

impl BufferState {
    /// Create a new empty buffer.
    pub fn new(id: BufferId, name: BufferName) -> Self {
        Self {
            id,
            name,
            path: None,
            text: TextBuffer::new(),
            history: UndoHistory::new(),
            modified: false,
        }
    }

    /// Create a buffer from a file.
    pub fn from_file(id: BufferId, path: PathBuf, content: &str) -> Self {
        let name = BufferName::new(
            path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("[No Name]"),
        );
        Self {
            id,
            name,
            path: Some(path),
            text: TextBuffer::from_str(content),
            history: UndoHistory::new(),
            modified: false,
        }
    }

    /// Get the buffer version.
    pub fn version(&self) -> BufferVersion {
        self.text.version()
    }

    /// Mark the buffer as modified.
    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.text.line_count()
    }

    /// Get a line by index.
    pub fn line(&self, idx: usize) -> Option<String> {
        self.text.line(idx).ok()
    }

    /// Get the length of a line.
    pub fn line_len(&self, idx: usize) -> usize {
        self.text.line_len(idx).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_state_new() {
        let buf = BufferState::new(BufferId::new(1), BufferName::unnamed());
        assert_eq!(buf.line_count(), 1);
        assert!(!buf.modified);
    }

    #[test]
    fn test_buffer_state_from_file() {
        let buf = BufferState::from_file(
            BufferId::new(1),
            PathBuf::from("/tmp/test.txt"),
            "hello\nworld",
        );
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.name.as_str(), "test.txt");
    }
}
