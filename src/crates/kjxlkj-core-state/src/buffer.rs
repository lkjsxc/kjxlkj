//! Buffer state.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion};
use kjxlkj_core_undo::UndoHistory;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete buffer state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferState {
    /// Buffer ID.
    pub id: BufferId,
    /// Buffer name.
    pub name: BufferName,
    /// File path (if file-backed).
    pub path: Option<PathBuf>,
    /// Text content.
    #[serde(skip)]
    pub text: TextBuffer,
    /// Undo history.
    #[serde(skip)]
    pub undo: UndoHistory,
    /// Modified flag.
    pub modified: bool,
    /// Read-only flag.
    pub readonly: bool,
    /// File type.
    pub filetype: String,
}

impl BufferState {
    /// Creates a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            name: BufferName::default(),
            path: None,
            text: TextBuffer::new(),
            undo: UndoHistory::new(),
            modified: false,
            readonly: false,
            filetype: String::new(),
        }
    }

    /// Creates a buffer from content.
    pub fn from_content(id: BufferId, content: &str) -> Self {
        Self {
            id,
            name: BufferName::default(),
            path: None,
            text: TextBuffer::from_str(content),
            undo: UndoHistory::new(),
            modified: false,
            readonly: false,
            filetype: String::new(),
        }
    }

    /// Sets the file path.
    pub fn with_path(mut self, path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        self.name = BufferName::new(name);
        self.path = Some(path);
        self
    }

    /// Returns the current version.
    pub fn version(&self) -> BufferVersion {
        self.text.version()
    }

    /// Returns the line count.
    pub fn line_count(&self) -> usize {
        self.text.line_count()
    }

    /// Returns a line.
    pub fn line(&self, idx: usize) -> String {
        self.text.line(idx)
    }
}
