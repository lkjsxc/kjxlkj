//! Main editor state.

use kjxlkj_core_mode::{CommandState, ModeState};
use kjxlkj_core_types::{BufferId, Cursor, Mode, Selection};
use kjxlkj_core_ui::{EditorSnapshot, SnapshotLine, StatusLine, Viewport};

use crate::buffer_state::BufferState;
use crate::registers::RegisterStore;

/// The main editor state.
#[derive(Debug)]
pub struct EditorState {
    /// Active buffer.
    pub buffer: BufferState,
    /// Mode state machine.
    pub mode: ModeState,
    /// Command line state.
    pub command: CommandState,
    /// Register store.
    pub registers: RegisterStore,
    /// Current selection (for visual mode).
    pub selection: Option<Selection>,
    /// Viewport.
    pub viewport: Viewport,
    /// Status message.
    pub status_message: Option<String>,
    /// Whether status is an error.
    pub status_error: bool,
    /// Search pattern.
    pub search_pattern: Option<String>,
    /// Search direction (true = forward).
    pub search_forward: bool,
    /// Jump list.
    pub jump_list: Vec<Cursor>,
    /// Jump list index.
    pub jump_index: usize,
    /// Change list.
    pub change_list: Vec<Cursor>,
    /// Change list index.
    pub change_index: usize,
    /// Last change for repeat.
    pub last_change: Option<String>,
    /// Whether to quit.
    pub should_quit: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    /// Create a new editor state.
    pub fn new() -> Self {
        Self {
            buffer: BufferState::new(BufferId::new(0)),
            mode: ModeState::new(),
            command: CommandState::new(),
            registers: RegisterStore::new(),
            selection: None,
            viewport: Viewport::new(0, 24, 80),
            status_message: None,
            status_error: false,
            search_pattern: None,
            search_forward: true,
            jump_list: Vec::new(),
            jump_index: 0,
            change_list: Vec::new(),
            change_index: 0,
            last_change: None,
            should_quit: false,
        }
    }

    /// Create an editor with initial text.
    pub fn with_text(text: &str) -> Self {
        let mut state = Self::new();
        state.buffer = BufferState::from_text(BufferId::new(0), text);
        state
    }

    /// Get current cursor.
    pub fn cursor(&self) -> Cursor {
        self.buffer.cursor
    }

    /// Get current mode.
    pub fn current_mode(&self) -> Mode {
        self.mode.mode
    }

    /// Set the viewport size.
    pub fn set_viewport_size(&mut self, height: usize, width: usize) {
        self.viewport.height = height;
        self.viewport.width = width;
    }

    /// Ensure cursor is visible.
    pub fn scroll_to_cursor(&mut self) {
        self.viewport.scroll_to_line(self.buffer.cursor.line);
    }

    /// Set a status message.
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
        self.status_error = false;
    }

    /// Set an error message.
    pub fn set_error(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
        self.status_error = true;
    }

    /// Clear the status message.
    pub fn clear_message(&mut self) {
        self.status_message = None;
        self.status_error = false;
    }

    /// Add current position to jump list.
    pub fn add_jump(&mut self) {
        self.jump_list.truncate(self.jump_index);
        self.jump_list.push(self.buffer.cursor);
        self.jump_index = self.jump_list.len();
    }

    /// Add current position to change list.
    pub fn add_change(&mut self) {
        self.change_list.push(self.buffer.cursor);
        self.change_index = self.change_list.len();
    }

    /// Create a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let mut lines = Vec::with_capacity(self.viewport.height);
        let total_lines = self.buffer.text.len_lines();

        for i in 0..self.viewport.height {
            let line_idx = self.viewport.top_line + i;
            if line_idx >= total_lines {
                break;
            }
            if let Some(content) = self.buffer.text.line_content(line_idx) {
                lines.push(SnapshotLine { line_idx, content });
            }
        }

        let mut status = StatusLine::new();
        status.mode = self.mode.mode;
        status.file_name = self.buffer.path.clone();
        status.modified = self.buffer.modified;
        status.line = self.buffer.cursor.line + 1;
        status.col = self.buffer.cursor.col + 1;
        status.total_lines = total_lines;
        if let Some(ref msg) = self.status_message {
            if self.status_error {
                status.set_error(msg.clone());
            } else {
                status.set_message(msg.clone());
            }
        }

        let command_line = if self.mode.mode == Mode::Command {
            Some(format!(":{}", self.command.command()))
        } else {
            None
        };

        EditorSnapshot {
            lines,
            cursor: self.buffer.cursor,
            mode: self.mode.mode,
            selection: self.selection,
            viewport: self.viewport,
            status,
            command_line,
            search_pattern: self.search_pattern.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_editor() {
        let editor = EditorState::new();
        assert_eq!(editor.current_mode(), Mode::Normal);
        assert!(!editor.should_quit);
    }

    #[test]
    fn test_snapshot() {
        let editor = EditorState::with_text("hello\nworld");
        let snap = editor.snapshot();
        assert_eq!(snap.lines.len(), 2);
        assert_eq!(snap.lines[0].content, "hello");
    }
}
