//! Buffer snapshot for rendering.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use kjxlkj_core_types::BufferId;

/// Immutable buffer snapshot produced by core for rendering.
///
/// Contains all data needed to render a buffer window without
/// querying core state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSnapshot {
    /// Buffer identity.
    pub id: BufferId,
    /// Monotonic edit version.
    pub version: u64,
    /// Total line count.
    pub line_count: usize,
    /// Filesystem path (None for scratch buffers).
    pub path: Option<PathBuf>,
    /// Display name.
    pub name: String,
    /// Whether content differs from disk.
    pub modified: bool,
    /// Whether buffer is readonly.
    pub readonly: bool,
    /// Line contents for visible range.
    /// Each entry is the grapheme-decomposed line string.
    pub visible_lines: Vec<String>,
    /// First visible line index (zero-based).
    pub top_line: usize,
    /// Cursor line (zero-based, relative to buffer).
    pub cursor_line: usize,
    /// Cursor grapheme offset within the line.
    pub cursor_col: usize,
    /// File type / language for statusline.
    pub file_type: String,
    /// Line ending style.
    pub line_ending: String,
    /// Encoding.
    pub encoding: String,
}

impl BufferSnapshot {
    /// Create a minimal empty snapshot.
    pub fn empty(id: BufferId) -> Self {
        Self {
            id,
            version: 0,
            line_count: 1,
            path: None,
            name: String::from("[No Name]"),
            modified: false,
            readonly: false,
            visible_lines: vec![String::new()],
            top_line: 0,
            cursor_line: 0,
            cursor_col: 0,
            file_type: String::new(),
            line_ending: String::from("LF"),
            encoding: String::from("utf-8"),
        }
    }

    /// Cursor position formatted as "line:col" (1-based for display).
    pub fn cursor_display(&self) -> String {
        format!("{}:{}", self.cursor_line + 1, self.cursor_col + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot() {
        let snap = BufferSnapshot::empty(BufferId(1));
        assert_eq!(snap.name, "[No Name]");
        assert_eq!(snap.cursor_display(), "1:1");
    }
}
