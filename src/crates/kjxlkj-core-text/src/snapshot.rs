//! Immutable buffer snapshots for O(viewport) rendering.

use kjxlkj_core_types::{BufferId, BufferVersion, Position};

/// An immutable snapshot of a text buffer region (O(viewport)).
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    pub buffer_id: BufferId,
    pub version: BufferVersion,
    pub lines: Vec<String>,
    pub first_line: usize,
    pub total_lines: usize,
    pub cursor: Position,
    pub modified: bool,
    pub name: String,
    pub file_path: Option<String>,
}

impl BufferSnapshot {
    /// Create a snapshot from a TextBuffer for a given viewport range.
    /// This is O(viewport_height), not O(total_lines).
    pub fn from_buffer(
        buf: &crate::TextBuffer,
        top_line: usize,
        viewport_height: usize,
        cursor: Position,
    ) -> Self {
        let total = buf.line_count();
        let start = top_line.min(total.saturating_sub(1));
        let end = (start + viewport_height).min(total);
        let mut lines = Vec::with_capacity(end - start);
        for i in start..end {
            lines.push(buf.line_to_string(i));
        }
        Self {
            buffer_id: buf.id(),
            version: buf.version(),
            lines,
            first_line: start,
            total_lines: total,
            cursor,
            modified: buf.is_modified(),
            name: buf.name().to_string(),
            file_path: buf.file_path().map(|p| p.to_string_lossy().into()),
        }
    }

    /// Number of lines in this snapshot.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get a line relative to the snapshot (0 = first visible line).
    pub fn line(&self, offset: usize) -> Option<&str> {
        self.lines.get(offset).map(|s| s.as_str())
    }

    /// Get a horizontal slice of a line for no-wrap mode (viewport-bounded).
    /// Returns only the visible portion of the line from left_col within width.
    pub fn line_slice(&self, offset: usize, left_col: usize, width: usize) -> Option<String> {
        let line = self.lines.get(offset)?;
        let chars: Vec<char> = line.chars().collect();
        let start = left_col.min(chars.len());
        let end = (left_col + width).min(chars.len());
        Some(chars[start..end].iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextBuffer;

    #[test]
    fn snapshot_viewport_only() {
        let mut text = String::new();
        for i in 0..1000 {
            text.push_str(&format!("line {}\n", i));
        }
        let buf = TextBuffer::from_text(&text);
        let snap = BufferSnapshot::from_buffer(
            &buf,
            100,
            24,
            Position::new(100, 0),
        );
        assert_eq!(snap.line_count(), 24);
        assert_eq!(snap.first_line, 100);
        assert_eq!(snap.total_lines, 1001);
        assert_eq!(snap.line(0), Some("line 100"));
    }

    #[test]
    fn snapshot_end_of_file() {
        let buf = TextBuffer::from_text("a\nb\nc");
        let snap = BufferSnapshot::from_buffer(
            &buf,
            1,
            100,
            Position::new(1, 0),
        );
        assert_eq!(snap.line_count(), 2);
        assert_eq!(snap.line(0), Some("b"));
        assert_eq!(snap.line(1), Some("c"));
    }

    #[test]
    fn snapshot_preserves_metadata() {
        let buf = TextBuffer::from_text("test");
        let snap = BufferSnapshot::from_buffer(
            &buf,
            0,
            10,
            Position::new(0, 0),
        );
        assert_eq!(snap.buffer_id, buf.id());
        assert_eq!(snap.version, buf.version());
        assert!(!snap.modified);
    }
}
