use kjxlkj_core_text::Rope;
use kjxlkj_core_types::{
    BufferId, BufferName, BufferVersion, CursorPosition, Encoding, LineEnding,
};
use kjxlkj_core_undo::UndoTree;
use std::path::PathBuf;

/// A single text buffer in the editor.
#[derive(Debug)]
pub struct Buffer {
    pub id: BufferId,
    pub name: BufferName,
    pub path: Option<PathBuf>,
    pub content: Rope,
    pub modified: bool,
    pub version: BufferVersion,
    pub encoding: Encoding,
    pub line_ending: LineEnding,
    pub readonly: bool,
    pub undo_tree: UndoTree,
    /// Content at last save for dirty detection.
    pub(crate) saved_version: BufferVersion,
}

impl Buffer {
    pub fn new_scratch(id: BufferId) -> Self {
        Self {
            id,
            name: BufferName::Scratch,
            path: None,
            content: Rope::new(),
            modified: false,
            version: BufferVersion(0),
            encoding: Encoding::default(),
            line_ending: LineEnding::default(),
            readonly: false,
            undo_tree: UndoTree::new(),
            saved_version: BufferVersion(0),
        }
    }

    pub fn from_text(id: BufferId, text: &str, path: Option<PathBuf>) -> Self {
        let name = path
            .as_ref()
            .map(|p| BufferName::from_path(p))
            .unwrap_or(BufferName::Scratch);
        let line_ending = if text.contains("\r\n") {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
        Self {
            id,
            name,
            path,
            content: Rope::from_str(text),
            modified: false,
            version: BufferVersion(0),
            encoding: Encoding::Utf8,
            line_ending,
            readonly: false,
            undo_tree: UndoTree::new(),
            saved_version: BufferVersion(0),
        }
    }

    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    pub fn increment_version(&mut self) {
        self.version = self.version.next();
        self.modified = self.version != self.saved_version;
    }

    pub fn mark_saved(&mut self) {
        self.saved_version = self.version;
        self.modified = false;
    }

    /// Save undo checkpoint before modification.
    pub fn save_undo_checkpoint(&mut self, cursor: CursorPosition) {
        self.undo_tree.push(
            self.version,
            self.content.clone(),
            cursor.line,
            cursor.grapheme,
        );
    }

    pub fn snapshot(&self) -> kjxlkj_core_ui::BufferSnapshot {
        kjxlkj_core_ui::BufferSnapshot {
            id: self.id,
            version: self.version,
            content: self.content.clone(),
            line_count: self.line_count(),
            path: self.path.clone(),
            modified: self.modified,
            name: self.name.to_string(),
        }
    }
}
