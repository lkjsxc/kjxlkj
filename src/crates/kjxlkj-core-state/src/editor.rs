//! Main editor state.

use crate::RegisterStore;
use kjxlkj_core_mode::ModeState;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, Position, Register, RegisterName};
use kjxlkj_core_ui::{Snapshot, Viewport};
use kjxlkj_core_undo::UndoHistory;
use std::path::PathBuf;

/// Main editor state.
#[derive(Debug)]
pub struct EditorState {
    /// Text buffer.
    pub buffer: TextBuffer,
    /// Cursor state.
    pub cursor: Cursor,
    /// Mode state.
    pub mode: ModeState,
    /// Undo history.
    pub history: UndoHistory,
    /// Register store.
    pub registers: RegisterStore,
    /// Viewport.
    pub viewport: Viewport,
    /// File path.
    pub file_path: Option<PathBuf>,
    /// Modified flag.
    pub modified: bool,
    /// Status message.
    pub status: String,
    /// Command line input.
    pub cmdline: String,
    /// Last search pattern.
    pub search_pattern: String,
    /// Search direction (true = forward).
    pub search_forward: bool,
    /// Pending count for operations.
    pub pending_count: Option<usize>,
    /// Pending register selection.
    pub pending_register: Option<char>,
    /// Last recorded macro register.
    pub macro_register: Option<char>,
    /// Recording macro flag.
    pub recording_macro: bool,
    /// Macro storage.
    pub macros: std::collections::HashMap<char, Vec<char>>,
    /// Current macro recording buffer.
    pub macro_buffer: Vec<char>,
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
            buffer: TextBuffer::new(),
            cursor: Cursor::new(Position::zero()),
            mode: ModeState::new(),
            history: UndoHistory::new(),
            registers: RegisterStore::new(),
            viewport: Viewport::new(80, 24),
            file_path: None,
            modified: false,
            status: String::new(),
            cmdline: String::new(),
            search_pattern: String::new(),
            search_forward: true,
            pending_count: None,
            pending_register: None,
            macro_register: None,
            recording_macro: false,
            macros: std::collections::HashMap::new(),
            macro_buffer: Vec::new(),
        }
    }

    /// Open a file.
    pub fn open_file(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        self.buffer = TextBuffer::from_file(path)?;
        self.file_path = Some(path.to_path_buf());
        self.cursor = Cursor::new(Position::zero());
        self.history = UndoHistory::new();
        self.modified = false;
        self.mode.to_normal();
        self.ensure_cursor_valid();
        Ok(())
    }

    /// Save the current buffer.
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.file_path {
            self.buffer.save_to_file(path)?;
            self.modified = false;
            self.status = format!("Wrote {}", path.display());
        }
        Ok(())
    }

    /// Save to a specific path.
    pub fn save_as(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        self.buffer.save_to_file(path)?;
        self.file_path = Some(path.to_path_buf());
        self.modified = false;
        self.status = format!("Wrote {}", path.display());
        Ok(())
    }

    /// Ensure cursor is within valid bounds.
    pub fn ensure_cursor_valid(&mut self) {
        let max_line = self.buffer.line_count().saturating_sub(1);
        if self.cursor.pos.line > max_line {
            self.cursor.pos.line = max_line;
        }
        let line_len = self.buffer.line_len(self.cursor.pos.line);
        let max_col = if self.mode.mode().is_insert() {
            line_len
        } else {
            line_len.saturating_sub(1)
        };
        if self.cursor.pos.col > max_col {
            self.cursor.pos.col = max_col;
        }
    }

    /// Create a snapshot for rendering.
    pub fn snapshot(&self) -> Snapshot {
        let lines = self.buffer.lines_range(
            self.viewport.top_line,
            self.viewport.top_line + self.viewport.height,
        );
        Snapshot {
            lines,
            top_line: self.viewport.top_line,
            cursor: self.cursor.pos,
            mode: self.mode.mode(),
            selection: self.cursor.selection_range(),
            status: self.status.clone(),
            cmdline: self.cmdline.clone(),
            total_lines: self.buffer.line_count(),
            file_path: self.file_path.as_ref().map(|p| p.display().to_string()),
            modified: self.modified,
            viewport_width: self.viewport.width,
            viewport_height: self.viewport.height,
        }
    }

    /// Scroll viewport to keep cursor visible.
    pub fn scroll_to_cursor(&mut self) {
        self.viewport.scroll_to_line(self.cursor.pos.line);
    }

    /// Get effective count (default 1).
    pub fn count(&self) -> usize {
        self.pending_count.unwrap_or(1)
    }

    /// Clear pending state.
    pub fn clear_pending(&mut self) {
        self.pending_count = None;
        self.pending_register = None;
    }

    /// Yank text to the selected or unnamed register.
    pub fn yank(&mut self, text: String, linewise: bool) {
        let name = self
            .pending_register
            .and_then(RegisterName::from_char)
            .unwrap_or(RegisterName::Unnamed);
        self.registers.set(name, Register::new(text, linewise));
    }

    /// Paste from the selected or unnamed register.
    pub fn paste(&self) -> Option<&Register> {
        let name = self
            .pending_register
            .and_then(RegisterName::from_char)
            .unwrap_or(RegisterName::Unnamed);
        self.registers.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Mode;

    #[test]
    fn test_editor_state_new() {
        let state = EditorState::new();
        assert_eq!(state.mode.mode(), Mode::Normal);
        assert!(!state.modified);
    }

    #[test]
    fn test_snapshot() {
        let mut state = EditorState::new();
        state.buffer = TextBuffer::from_text("hello\nworld");
        state.viewport = Viewport::new(80, 24);
        let snap = state.snapshot();
        assert_eq!(snap.lines.len(), 2);
        assert_eq!(snap.lines[0], "hello");
    }
}
