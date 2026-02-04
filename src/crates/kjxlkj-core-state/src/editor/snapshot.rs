//! Snapshot creation for editor state.

use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine};

use super::EditorState;

impl EditorState {
    /// Create a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let visible_lines = self.viewport.visible_lines();
        let first_line = self.viewport.first_line;

        let mut lines = Vec::with_capacity(visible_lines);
        for i in 0..visible_lines {
            let line_idx = first_line + i;
            if let Some(line) = self.buffer.line(line_idx) {
                lines.push(line);
            } else {
                lines.push(String::from("~"));
            }
        }

        let buffer = BufferSnapshot {
            id: self.buffer.id(),
            version: self.buffer.version(),
            lines,
            first_line,
            total_lines: self.buffer.line_count(),
            name: self.buffer.path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "[No Name]".to_string()),
            modified: self.buffer.is_modified(),
        };

        let total = self.buffer.line_count().max(1);
        let percent = if total <= 1 { 100 } else { ((self.cursor.line() + 1) * 100) / total };

        let status = StatusLine {
            mode: self.mode().display_name().to_string(),
            filename: buffer.name.clone(),
            modified: buffer.modified,
            position: format!("{}:{}", self.cursor.line() + 1, self.cursor.col() + 1),
            percentage: format!("{}%", percent),
            message: self.status_message.clone(),
            command_line: if self.mode() == Mode::Command {
                Some(self.mode_state.command_line.clone())
            } else {
                None
            },
        };

        EditorSnapshot {
            buffer,
            cursor: self.cursor,
            selection: self.selection,
            mode: self.mode(),
            viewport: self.viewport,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_creation() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        let _ = snapshot;
    }

    #[test]
    fn snapshot_has_buffer() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        let _ = snapshot.buffer;
    }

    #[test]
    fn snapshot_has_cursor() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        let _ = snapshot.cursor;
    }

    #[test]
    fn snapshot_has_mode() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert_eq!(snapshot.mode, Mode::Normal);
    }

    #[test]
    fn snapshot_has_viewport() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        let _ = snapshot.viewport;
    }

    #[test]
    fn snapshot_has_status() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        let _ = &snapshot.status;
    }

    #[test]
    fn snapshot_status_mode() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(!snapshot.status.mode.is_empty());
    }

    #[test]
    fn snapshot_status_position() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(!snapshot.status.position.is_empty());
    }

    #[test]
    fn snapshot_buffer_lines() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(!snapshot.buffer.lines.is_empty() || snapshot.buffer.total_lines == 0);
    }

    #[test]
    fn snapshot_buffer_id() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        let _ = snapshot.buffer.id;
    }

    #[test]
    fn snapshot_buffer_version() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        let _ = snapshot.buffer.version;
    }

    #[test]
    fn snapshot_selection_none() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(snapshot.selection.is_none());
    }

    #[test]
    fn snapshot_status_percentage() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(snapshot.status.percentage.contains('%'));
    }

    #[test]
    fn snapshot_no_name_default() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(snapshot.buffer.name.contains("No Name"));
    }

    #[test]
    fn snapshot_command_line_none() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(snapshot.status.command_line.is_none());
    }

    #[test]
    fn snapshot_modified_false() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(!snapshot.buffer.modified);
    }

    #[test]
    fn snapshot_status_message_none() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(snapshot.status.message.is_none());
    }

    #[test]
    fn snapshot_first_line() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert_eq!(snapshot.buffer.first_line, 0);
    }

    #[test]
    fn snapshot_total_lines() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(snapshot.buffer.total_lines >= 1);
    }
}
