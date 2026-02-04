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
