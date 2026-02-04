//! Buffer state management.

use kjxlkj_core_text::RopeText;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, CharOffset, Cursor, LineCol};
use kjxlkj_core_undo::UndoHistory;
use std::path::PathBuf;

/// State of a single buffer.
#[derive(Debug)]
pub struct BufferState {
    /// Unique identifier.
    pub id: BufferId,
    /// Display name.
    pub name: BufferName,
    /// Optional file path.
    pub path: Option<PathBuf>,
    /// Text content.
    pub text: RopeText,
    /// Current version.
    pub version: BufferVersion,
    /// Cursor position.
    pub cursor: Cursor,
    /// Modified flag.
    pub modified: bool,
    /// Undo history.
    pub undo: UndoHistory,
}

impl BufferState {
    /// Create a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            name: BufferName::unnamed(),
            path: None,
            text: RopeText::new(),
            version: BufferVersion::default(),
            cursor: Cursor::origin(),
            modified: false,
            undo: UndoHistory::new(),
        }
    }

    /// Create a buffer from text content.
    pub fn from_text(id: BufferId, name: BufferName, content: &str) -> Self {
        Self {
            id,
            name,
            path: None,
            text: RopeText::from_str(content),
            version: BufferVersion::default(),
            cursor: Cursor::origin(),
            modified: false,
            undo: UndoHistory::new(),
        }
    }

    /// Create a buffer from a file path.
    pub fn from_path(id: BufferId, path: PathBuf, content: &str) -> Self {
        let name = BufferName::new(
            path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("[Unknown]"),
        );
        Self {
            id,
            name,
            path: Some(path),
            text: RopeText::from_str(content),
            version: BufferVersion::default(),
            cursor: Cursor::origin(),
            modified: false,
            undo: UndoHistory::new(),
        }
    }

    /// Get the total number of lines.
    pub fn line_count(&self) -> usize {
        self.text.len_lines()
    }

    /// Get a line's content.
    pub fn line(&self, idx: usize) -> Option<String> {
        self.text.line(idx)
    }

    /// Insert text at cursor.
    pub fn insert_at_cursor(&mut self, text: &str) {
        if let Some(offset) = self.text.linecol_to_char(self.cursor.position) {
            self.text.insert(offset, text);
            self.version = self.version.next();
            self.modified = true;
            // Move cursor forward by inserted length
            let new_offset = CharOffset::new(offset.as_usize() + text.chars().count());
            self.cursor.position = self.text.char_to_linecol(new_offset);
        }
    }

    /// Delete character at cursor.
    pub fn delete_at_cursor(&mut self) {
        if let Some(offset) = self.text.linecol_to_char(self.cursor.position) {
            let end = CharOffset::new(offset.as_usize() + 1);
            if end.as_usize() <= self.text.len_chars() {
                self.text.delete(offset, end);
                self.version = self.version.next();
                self.modified = true;
            }
        }
    }

    /// Delete character before cursor.
    pub fn delete_before_cursor(&mut self) {
        if let Some(offset) = self.text.linecol_to_char(self.cursor.position) {
            if offset.as_usize() > 0 {
                let start = CharOffset::new(offset.as_usize() - 1);
                self.text.delete(start, offset);
                self.version = self.version.next();
                self.modified = true;
                self.cursor.position = self.text.char_to_linecol(start);
            }
        }
    }

    /// Move cursor, clamping to valid positions.
    pub fn move_cursor(&mut self, new_pos: LineCol) {
        self.cursor.position = self.text.clamp_position(new_pos);
    }

    /// Move cursor by lines.
    pub fn move_cursor_lines(&mut self, delta: isize) {
        let new_line = if delta >= 0 {
            self.cursor.position.line.saturating_add(delta as usize)
        } else {
            self.cursor.position.line.saturating_sub((-delta) as usize)
        };
        let max_line = self.line_count().saturating_sub(1);
        let clamped_line = new_line.min(max_line);

        // Use desired column (sticky column)
        let line_len = self.text.line_len_chars(clamped_line).unwrap_or(0);
        let col = self.cursor.desired_col.min(line_len.saturating_sub(1).max(0));

        self.cursor.position = LineCol::new(clamped_line, col);
    }

    /// Move cursor by columns.
    pub fn move_cursor_cols(&mut self, delta: isize) {
        let line_len = self
            .text
            .line_len_chars(self.cursor.position.line)
            .unwrap_or(0);

        let new_col = if delta >= 0 {
            self.cursor.position.col.saturating_add(delta as usize)
        } else {
            self.cursor.position.col.saturating_sub((-delta) as usize)
        };

        let max_col = line_len.saturating_sub(1).max(0);
        let clamped_col = new_col.min(max_col);

        self.cursor.position.col = clamped_col;
        self.cursor.desired_col = clamped_col;
    }

    /// Clamp cursor to valid buffer positions.
    pub fn clamp_cursor(&mut self) {
        self.cursor.position = self.text.clamp_position(self.cursor.position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_is_empty() {
        let buf = BufferState::new(BufferId::new(1));
        assert!(!buf.modified);
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn buffer_from_text() {
        let buf = BufferState::from_text(
            BufferId::new(1),
            BufferName::new("test"),
            "line1\nline2",
        );
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0), Some("line1".to_string()));
    }

    #[test]
    fn insert_at_cursor() {
        let mut buf = BufferState::new(BufferId::new(1));
        buf.insert_at_cursor("hello");
        assert!(buf.modified);
        assert!(buf.text.to_string().contains("hello"));
    }

    #[test]
    fn move_cursor_lines() {
        let mut buf = BufferState::from_text(
            BufferId::new(1),
            BufferName::new("test"),
            "a\nb\nc",
        );
        buf.move_cursor_lines(2);
        assert_eq!(buf.cursor.position.line, 2);
        buf.move_cursor_lines(-1);
        assert_eq!(buf.cursor.position.line, 1);
    }
}
