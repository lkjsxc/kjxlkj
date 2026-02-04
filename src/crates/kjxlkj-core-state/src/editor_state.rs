//! Complete editor state.

use kjxlkj_core_mode::ModeState;
use kjxlkj_core_types::{BufferId, BufferName, Mode};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, Viewport};

use crate::buffer_state::BufferState;

/// Complete editor state.
#[derive(Debug)]
pub struct EditorState {
    /// All open buffers.
    buffers: Vec<BufferState>,
    /// Index of active buffer.
    active_buffer: usize,
    /// Next buffer ID to assign.
    next_buffer_id: u64,
    /// Mode state machine.
    pub mode_state: ModeState,
    /// Viewport for active buffer.
    pub viewport: Viewport,
    /// Registers (named clipboards).
    registers: std::collections::HashMap<char, String>,
    /// Unnamed register (default yank/delete target).
    unnamed_register: String,
    /// Last search pattern.
    pub search_pattern: Option<String>,
    /// Command line buffer.
    pub command_buffer: String,
    /// Last message to display.
    pub message: Option<String>,
    /// Whether to quit.
    pub should_quit: bool,
}

impl EditorState {
    /// Create a new editor state with an empty buffer.
    pub fn new() -> Self {
        let initial_buffer = BufferState::new(BufferId::new(0));
        Self {
            buffers: vec![initial_buffer],
            active_buffer: 0,
            next_buffer_id: 1,
            mode_state: ModeState::new(),
            viewport: Viewport::default(),
            registers: std::collections::HashMap::new(),
            unnamed_register: String::new(),
            search_pattern: None,
            command_buffer: String::new(),
            message: None,
            should_quit: false,
        }
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode_state.mode()
    }

    /// Get the active buffer.
    pub fn active_buffer(&self) -> &BufferState {
        &self.buffers[self.active_buffer]
    }

    /// Get the active buffer mutably.
    pub fn active_buffer_mut(&mut self) -> &mut BufferState {
        &mut self.buffers[self.active_buffer]
    }

    /// Open a new buffer with content.
    pub fn open_buffer(&mut self, name: BufferName, content: &str) -> BufferId {
        let id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;
        let buffer = BufferState::from_text(id, name, content);
        self.buffers.push(buffer);
        self.active_buffer = self.buffers.len() - 1;
        id
    }

    /// Open a file into a buffer.
    pub fn open_file(&mut self, path: std::path::PathBuf, content: &str) -> BufferId {
        let id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;
        let buffer = BufferState::from_path(id, path, content);
        self.buffers.push(buffer);
        self.active_buffer = self.buffers.len() - 1;
        id
    }

    /// Get buffer by ID.
    pub fn buffer(&self, id: BufferId) -> Option<&BufferState> {
        self.buffers.iter().find(|b| b.id == id)
    }

    /// Get buffer by ID mutably.
    pub fn buffer_mut(&mut self, id: BufferId) -> Option<&mut BufferState> {
        self.buffers.iter_mut().find(|b| b.id == id)
    }

    /// Set the named register.
    pub fn set_register(&mut self, name: char, content: String) {
        if name == '"' {
            self.unnamed_register = content;
        } else {
            self.registers.insert(name, content);
        }
    }

    /// Get the named register.
    pub fn get_register(&self, name: char) -> &str {
        if name == '"' {
            &self.unnamed_register
        } else {
            self.registers.get(&name).map(|s| s.as_str()).unwrap_or("")
        }
    }

    /// Set a message to display.
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
    }

    /// Clear the message.
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Generate a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let buf = self.active_buffer();

        // Collect visible lines
        let first_line = self.viewport.scroll_top;
        let last_line = (first_line + self.viewport.visible_lines()).min(buf.line_count());
        let lines: Vec<String> = (first_line..last_line)
            .filter_map(|i| buf.line(i))
            .collect();

        let buffer_snap = BufferSnapshot {
            id: buf.id,
            name: buf.name.clone(),
            version: buf.version,
            lines,
            first_line,
            total_lines: buf.line_count(),
            modified: buf.modified,
        };

        EditorSnapshot::new(buffer_snap, self.mode(), buf.cursor, self.viewport)
    }

    /// Ensure cursor is visible in viewport.
    pub fn ensure_cursor_visible(&mut self) {
        let cursor_line = self.active_buffer().cursor.position.line;
        self.viewport.ensure_visible(
            kjxlkj_core_types::LineCol::new(cursor_line, 0),
            3, // margin
        );
    }

    /// Resize the viewport.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.viewport.width = width;
        // Reserve 1 line for status, 1 for command
        self.viewport.height = height.saturating_sub(2);
        self.ensure_cursor_visible();
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
    fn new_editor_has_one_buffer() {
        let editor = EditorState::new();
        assert_eq!(editor.buffers.len(), 1);
    }

    #[test]
    fn open_buffer_becomes_active() {
        let mut editor = EditorState::new();
        let id = editor.open_buffer(BufferName::new("test"), "content");
        assert_eq!(editor.active_buffer().id, id);
    }

    #[test]
    fn register_storage() {
        let mut editor = EditorState::new();
        editor.set_register('a', "test content".to_string());
        assert_eq!(editor.get_register('a'), "test content");
    }

    #[test]
    fn snapshot_generation() {
        let mut editor = EditorState::new();
        editor.open_buffer(BufferName::new("test"), "line1\nline2\nline3");
        let snap = editor.snapshot();
        assert_eq!(snap.buffer.total_lines, 3);
    }
}
