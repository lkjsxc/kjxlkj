//! Editor snapshots for rendering.

use kjxlkj_core_types::{BufferId, BufferVersion, Cursor, Mode};

use crate::Viewport;

/// A snapshot of a buffer for rendering.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub version: BufferVersion,
    pub name: String,
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub viewport: Viewport,
    pub modified: bool,
}

/// Status line content.
#[derive(Debug, Clone, Default)]
pub struct StatusLine {
    pub mode: Mode,
    pub file_name: String,
    pub modified: bool,
    pub cursor_line: u32,
    pub cursor_col: u32,
    pub line_count: usize,
    pub message: Option<String>,
}

/// A complete editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    pub buffer: BufferSnapshot,
    pub status: StatusLine,
    pub command_line: Option<String>,
    pub terminal_size: (u16, u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_line_default() {
        let status = StatusLine::default();
        assert_eq!(status.mode, Mode::Normal);
    }
}
