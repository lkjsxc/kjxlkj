//! Individual buffer state.

use std::path::PathBuf;

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::BufferId;
use kjxlkj_core_undo::UndoTree;

/// Line ending representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
}

impl LineEnding {
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Lf => "LF",
            LineEnding::CrLf => "CRLF",
        }
    }
}

/// State for a single buffer.
pub struct BufferState {
    /// Stable unique identifier.
    pub id: BufferId,
    /// Display name.
    pub name: String,
    /// Filesystem path (None for scratch buffers).
    pub path: Option<PathBuf>,
    /// Text content.
    pub content: BufferContent,
    /// Whether content differs from last-saved state.
    pub modified: bool,
    /// Monotonic edit version.
    pub version: u64,
    /// Line ending style.
    pub line_ending: LineEnding,
    /// Whether buffer is readonly.
    pub readonly: bool,
    /// Undo tree.
    pub undo_tree: UndoTree,
    /// File type / language.
    pub file_type: String,
    /// Original content hash (for tracking modification).
    saved_hash: u64,
}

impl BufferState {
    /// Create a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            name: String::from("[No Name]"),
            path: None,
            content: BufferContent::empty(),
            modified: false,
            version: 0,
            line_ending: LineEnding::Lf,
            readonly: false,
            undo_tree: UndoTree::new(),
            file_type: String::new(),
            saved_hash: 0,
        }
    }

    /// Create a buffer from file content.
    pub fn from_content(id: BufferId, path: PathBuf, text: &str) -> Self {
        let le = if text.contains("\r\n") {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());
        let content = BufferContent::from_str(text);
        let hash = simple_hash(text);
        Self {
            id,
            name,
            path: Some(path),
            content,
            modified: false,
            version: 0,
            line_ending: le,
            readonly: false,
            undo_tree: UndoTree::new(),
            file_type: String::new(),
            saved_hash: hash,
        }
    }

    /// Insert text at a byte offset position.
    pub fn insert_at(&mut self, line: usize, col: usize, text: &str) {
        self.content.insert(line, col, text);
        self.version += 1;
        self.modified = true;
    }

    /// Delete a range of text.
    pub fn delete_range(
        &mut self,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) {
        self.content
            .delete(start_line, start_col, end_line, end_col);
        self.version += 1;
        self.modified = true;
    }

    /// Total line count.
    pub fn line_count(&self) -> usize {
        self.content.line_count()
    }

    /// Mark as saved.
    pub fn mark_saved(&mut self) {
        self.modified = false;
        let snap = self.content.snapshot();
        let text = snap.to_string();
        self.saved_hash = simple_hash(&text);
    }

    /// Detect file type from path and content.
    pub fn detect_file_type(&mut self) {
        if let Some(path) = &self.path {
            let path_str = path.to_string_lossy().to_string();
            let first_line = if self.content.line_count() > 0 {
                Some(self.content.line_str(0))
            } else {
                None
            };
            let ft = crate::filetype::detect_filetype(&path_str, first_line.as_deref());
            if !ft.is_empty() {
                self.file_type = ft;
            }
        }
    }

    /// Create an empty buffer with a path.
    pub fn new_with_path(id: BufferId, path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());
        Self {
            id,
            name,
            path: Some(path),
            content: BufferContent::empty(),
            modified: false,
            version: 0,
            line_ending: LineEnding::Lf,
            readonly: false,
            undo_tree: UndoTree::new(),
            file_type: String::new(),
            saved_hash: 0,
        }
    }

    /// Undo the last change (stub — records exist in tree).
    pub fn undo(&mut self) {
        if self.undo_tree.undo().is_some() {
            self.version += 1;
        }
    }

    /// Redo a previously undone change (stub).
    pub fn redo(&mut self) {
        if self.undo_tree.redo().is_some() {
            self.version += 1;
        }
    }
}

fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u64);
    }
    hash
}
