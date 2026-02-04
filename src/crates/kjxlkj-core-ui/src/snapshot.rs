//! Editor snapshot types for rendering.

use crate::Viewport;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, Mode, Selection};

/// A snapshot of a buffer for rendering.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    /// Buffer ID.
    pub id: BufferId,
    /// Buffer name.
    pub name: BufferName,
    /// Buffer version.
    pub version: BufferVersion,
    /// Total line count.
    pub line_count: usize,
    /// Visible lines (content for each line in viewport).
    pub lines: Vec<String>,
    /// The viewport.
    pub viewport: Viewport,
    /// Whether the buffer is modified.
    pub modified: bool,
}

impl BufferSnapshot {
    /// Create a new buffer snapshot.
    pub fn new(
        id: BufferId,
        name: BufferName,
        version: BufferVersion,
        line_count: usize,
        lines: Vec<String>,
        viewport: Viewport,
        modified: bool,
    ) -> Self {
        Self {
            id,
            name,
            version,
            line_count,
            lines,
            viewport,
            modified,
        }
    }
}

/// Status line information.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatusLine {
    /// Mode display string.
    pub mode: String,
    /// File name or buffer name.
    pub file_name: String,
    /// Modified indicator.
    pub modified: bool,
    /// Cursor position (1-indexed for display).
    pub line: usize,
    /// Column position (1-indexed for display).
    pub col: usize,
    /// Total line count.
    pub total_lines: usize,
    /// Status message (if any).
    pub message: Option<(String, bool)>,
}

impl StatusLine {
    /// Create a new status line.
    pub fn new(
        mode: Mode,
        file_name: String,
        modified: bool,
        cursor: &Cursor,
        total_lines: usize,
    ) -> Self {
        Self {
            mode: mode.as_str().to_uppercase(),
            file_name,
            modified,
            line: cursor.line(),
            col: cursor.col(),
            total_lines,
            message: None,
        }
    }

    /// Set a status message.
    pub fn with_message(mut self, message: String, is_error: bool) -> Self {
        self.message = Some((message, is_error));
        self
    }
}

/// A complete editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// The active buffer snapshot.
    pub buffer: BufferSnapshot,
    /// Current cursor position.
    pub cursor: Cursor,
    /// Current mode.
    pub mode: Mode,
    /// Current selection (if any).
    pub selection: Option<Selection>,
    /// Status line.
    pub status: StatusLine,
    /// Command line content (if in command mode).
    pub command_line: Option<String>,
    /// Search pattern (if searching).
    pub search_pattern: Option<String>,
    /// Terminal dimensions.
    pub width: u16,
    pub height: u16,
}

impl EditorSnapshot {
    /// Create a new editor snapshot.
    pub fn new(
        buffer: BufferSnapshot,
        cursor: Cursor,
        mode: Mode,
        selection: Option<Selection>,
        status: StatusLine,
        command_line: Option<String>,
        search_pattern: Option<String>,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            buffer,
            cursor,
            mode,
            selection,
            status,
            command_line,
            search_pattern,
            width,
            height,
        }
    }

    /// Check if a line is within the current selection.
    pub fn is_line_in_selection(&self, line_idx: usize) -> bool {
        if let Some(ref sel) = self.selection {
            let start = sel.start().line;
            let end = sel.end().line;
            line_idx >= start && line_idx <= end
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_line() {
        let status = StatusLine::new(
            Mode::Normal,
            "test.rs".to_string(),
            false,
            &Cursor::new(0, 0),
            100,
        );
        assert_eq!(status.mode, "NORMAL");
        assert_eq!(status.line, 0);
        assert_eq!(status.col, 0);
    }

    #[test]
    fn test_status_line_with_message() {
        let status = StatusLine::new(
            Mode::Normal,
            "test.rs".to_string(),
            false,
            &Cursor::new(0, 0),
            100,
        )
        .with_message("File saved".to_string(), false);
        assert!(status.message.is_some());
        assert!(!status.message.unwrap().1);
    }

    #[test]
    fn test_status_line_modified() {
        let status = StatusLine::new(
            Mode::Insert,
            "test.rs".to_string(),
            true,
            &Cursor::new(5, 10),
            200,
        );
        assert!(status.modified);
        assert_eq!(status.mode, "INSERT");
        assert_eq!(status.line, 5);
        assert_eq!(status.col, 10);
        assert_eq!(status.total_lines, 200);
    }

    #[test]
    fn test_status_line_error_message() {
        let status = StatusLine::default()
            .with_message("Error occurred".to_string(), true);
        assert!(status.message.is_some());
        let (msg, is_error) = status.message.unwrap();
        assert_eq!(msg, "Error occurred");
        assert!(is_error);
    }

    #[test]
    fn test_status_line_default() {
        let status = StatusLine::default();
        assert!(status.mode.is_empty());
        assert!(status.file_name.is_empty());
        assert!(!status.modified);
    }

    #[test]
    fn test_buffer_snapshot_new() {
        let snapshot = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test"),
            BufferVersion::new(1),
            10,
            vec!["line1".to_string(), "line2".to_string()],
            Viewport::new(0, 24, 0, 80),
            false,
        );
        assert_eq!(snapshot.id, BufferId::new(1));
        assert_eq!(snapshot.lines.len(), 2);
    }

    #[test]
    fn test_buffer_snapshot_modified() {
        let snapshot = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test"),
            BufferVersion::new(1),
            10,
            vec![],
            Viewport::new(0, 24, 0, 80),
            true,
        );
        assert!(snapshot.modified);
    }

    #[test]
    fn test_buffer_snapshot_clone() {
        let snapshot = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test"),
            BufferVersion::new(1),
            10,
            vec!["hello".to_string()],
            Viewport::new(0, 24, 0, 80),
            false,
        );
        let cloned = snapshot.clone();
        assert_eq!(cloned.id, snapshot.id);
        assert_eq!(cloned.lines, snapshot.lines);
    }

    #[test]
    fn test_status_line_visual_mode() {
        let status = StatusLine::new(
            Mode::Visual,
            "test.rs".to_string(),
            false,
            &Cursor::new(0, 0),
            100,
        );
        assert_eq!(status.mode, "VISUAL");
    }

    #[test]
    fn test_status_line_command_mode() {
        let status = StatusLine::new(
            Mode::Command,
            "test.rs".to_string(),
            false,
            &Cursor::new(0, 0),
            100,
        );
        assert_eq!(status.mode, "COMMAND");
    }
}

