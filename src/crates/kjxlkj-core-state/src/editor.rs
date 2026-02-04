//! Main editor state.

use kjxlkj_core_mode::ModeState;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Cursor, Mode, Position, Selection, SelectionKind};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};
use kjxlkj_core_undo::UndoHistory;

use crate::{MarkStore, RegisterStore};

/// The main editor state.
#[derive(Debug)]
pub struct EditorState {
    /// Current buffer.
    pub buffer: TextBuffer,
    /// Cursor position.
    pub cursor: Cursor,
    /// Visual selection.
    pub selection: Option<Selection>,
    /// Mode state.
    pub mode_state: ModeState,
    /// Undo history.
    pub undo: UndoHistory,
    /// Registers.
    pub registers: RegisterStore,
    /// Marks.
    pub marks: MarkStore,
    /// Viewport.
    pub viewport: Viewport,
    /// Status message.
    pub status_message: Option<String>,
    /// Whether to quit.
    pub should_quit: bool,
    /// Jump list.
    jump_list: Vec<Position>,
    /// Jump list index.
    jump_index: usize,
    /// Change list.
    change_list: Vec<Position>,
    /// Change list index.
    change_index: usize,
    /// Next buffer ID.
    next_buffer_id: u64,
}

impl EditorState {
    /// Create a new editor state.
    pub fn new() -> Self {
        Self {
            buffer: TextBuffer::new(BufferId::new(0)),
            cursor: Cursor::origin(),
            selection: None,
            mode_state: ModeState::new(),
            undo: UndoHistory::new(),
            registers: RegisterStore::new(),
            marks: MarkStore::new(),
            viewport: Viewport::default(),
            status_message: None,
            should_quit: false,
            jump_list: Vec::new(),
            jump_index: 0,
            change_list: Vec::new(),
            change_index: 0,
            next_buffer_id: 1,
        }
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode_state.mode
    }

    /// Set the mode.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode_state.set_mode(mode);
        if mode.is_visual() {
            let kind = match mode {
                Mode::Visual => SelectionKind::Char,
                Mode::VisualLine => SelectionKind::Line,
                Mode::VisualBlock => SelectionKind::Block,
                _ => SelectionKind::Char,
            };
            self.selection = Some(Selection::new(
                self.cursor.position,
                self.cursor.position,
                kind,
            ));
        } else {
            self.selection = None;
        }
    }

    /// Set status message.
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
    }

    /// Clear status message.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Add to jump list.
    pub fn push_jump(&mut self) {
        // Truncate future jumps
        self.jump_list.truncate(self.jump_index);
        self.jump_list.push(self.cursor.position);
        self.jump_index = self.jump_list.len();
    }

    /// Jump backward in jump list.
    pub fn jump_backward(&mut self) -> Option<Position> {
        if self.jump_index > 0 {
            self.jump_index -= 1;
            Some(self.jump_list[self.jump_index])
        } else {
            None
        }
    }

    /// Jump forward in jump list.
    pub fn jump_forward(&mut self) -> Option<Position> {
        if self.jump_index < self.jump_list.len() {
            let pos = self.jump_list[self.jump_index];
            self.jump_index += 1;
            Some(pos)
        } else {
            None
        }
    }

    /// Add to change list.
    pub fn push_change(&mut self, pos: Position) {
        self.change_list.push(pos);
        self.change_index = self.change_list.len();
    }

    /// Ensure cursor is visible.
    pub fn ensure_cursor_visible(&mut self) {
        self.viewport.ensure_visible(self.cursor.line(), 3);
    }

    /// Clamp cursor to valid position.
    pub fn clamp_cursor(&mut self) {
        let line_count = self.buffer.line_count();
        if line_count == 0 {
            self.cursor = Cursor::origin();
            return;
        }
        let line = self.cursor.line().min(line_count.saturating_sub(1));
        let max_col = if self.mode().is_insert() {
            self.buffer.line_len(line)
        } else {
            self.buffer.line_len(line).saturating_sub(1)
        };
        let col = self.cursor.col().min(max_col);
        self.cursor.position = Position::new(line, col);
    }

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
            name: self
                .buffer
                .path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "[No Name]".to_string()),
            modified: self.buffer.is_modified(),
        };

        let total = self.buffer.line_count().max(1);
        let percent = if total <= 1 {
            100
        } else {
            ((self.cursor.line() + 1) * 100) / total
        };

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

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_state_creation() {
        let state = EditorState::new();
        assert_eq!(state.mode(), Mode::Normal);
        assert!(!state.should_quit);
    }

    #[test]
    fn editor_state_mode_change() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        assert_eq!(state.mode(), Mode::Insert);
    }

    #[test]
    fn editor_state_visual_creates_selection() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Visual);
        assert!(state.selection.is_some());
    }

    #[test]
    fn editor_state_snapshot() {
        let state = EditorState::new();
        let snap = state.snapshot();
        assert_eq!(snap.mode, Mode::Normal);
    }
}
