//! Main editor state.

use std::collections::HashMap;
use std::path::PathBuf;

use kjxlkj_core_edit::{clamp_cursor, clamp_cursor_for_mode, execute_motion, EditOp};
use kjxlkj_core_mode::{parse_command, process_key, ModeState};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    BufferId, Cursor, EditorError, EditorResult, Intent, Key, Mode, RegisterContent, RegisterName, WindowId,
    WindowState,
};
use kjxlkj_core_undo::UndoHistory;
use kjxlkj_core_ui::{
    build_status, EditorSnapshot, LineSnapshot, MessageLevel, StatusMessage, WindowSnapshot,
};

use crate::RegisterStore;

/// Complete editor state.
pub struct EditorState {
    /// All buffers.
    buffers: HashMap<BufferId, TextBuffer>,
    /// All windows.
    windows: HashMap<WindowId, WindowState>,
    /// Active window ID.
    active_window: WindowId,
    /// Mode state.
    mode_state: ModeState,
    /// Undo histories per buffer.
    undo_histories: HashMap<BufferId, UndoHistory>,
    /// Register store.
    registers: RegisterStore,
    /// Current status message.
    message: Option<StatusMessage>,
    /// Whether to quit.
    should_quit: bool,
    /// Terminal size.
    terminal_size: (u16, u16),
}

impl EditorState {
    /// Create a new editor state with an empty buffer.
    pub fn new() -> Self {
        let buffer = TextBuffer::new();
        let buffer_id = buffer.id();

        let window_id = WindowId::new();
        let window = WindowState::new(window_id, buffer_id);

        let mut buffers = HashMap::new();
        buffers.insert(buffer_id, buffer);

        let mut windows = HashMap::new();
        windows.insert(window_id, window);

        let mut undo_histories = HashMap::new();
        undo_histories.insert(buffer_id, UndoHistory::new());

        Self {
            buffers,
            windows,
            active_window: window_id,
            mode_state: ModeState::new(),
            undo_histories,
            registers: RegisterStore::new(),
            message: None,
            should_quit: false,
            terminal_size: (80, 24),
        }
    }

    /// Create editor state with initial file.
    pub fn with_file(path: PathBuf, content: &str) -> Self {
        let buffer = TextBuffer::from_file(path, content);
        let buffer_id = buffer.id();

        let window_id = WindowId::new();
        let window = WindowState::new(window_id, buffer_id);

        let mut buffers = HashMap::new();
        buffers.insert(buffer_id, buffer);

        let mut windows = HashMap::new();
        windows.insert(window_id, window);

        let mut undo_histories = HashMap::new();
        undo_histories.insert(buffer_id, UndoHistory::new());

        Self {
            buffers,
            windows,
            active_window: window_id,
            mode_state: ModeState::new(),
            undo_histories,
            registers: RegisterStore::new(),
            message: None,
            should_quit: false,
            terminal_size: (80, 24),
        }
    }

    /// Set terminal size.
    pub fn set_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
        // Update viewport sizes
        for window in self.windows.values_mut() {
            window.viewport.width = width;
            window.viewport.height = height.saturating_sub(2); // Reserve for status
        }
    }

    /// Check if editor should quit.
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get current mode.
    pub fn mode(&self) -> Mode {
        self.mode_state.mode
    }

    /// Get command line content.
    pub fn command_line(&self) -> &str {
        &self.mode_state.command_line
    }

    /// Process a key event.
    pub fn handle_key(&mut self, key: Key) -> EditorResult<()> {
        let intents = process_key(&mut self.mode_state, key);

        for intent in intents {
            self.apply_intent(intent)?;
        }

        Ok(())
    }

    /// Apply an intent to the editor state.
    pub fn apply_intent(&mut self, intent: Intent) -> EditorResult<()> {
        let window = self.windows.get_mut(&self.active_window)
            .ok_or(EditorError::WindowNotFound(self.active_window))?;
        let buffer = self.buffers.get_mut(&window.buffer_id)
            .ok_or(EditorError::BufferNotFound(window.buffer_id))?;

        match intent {
            Intent::CursorMove(dir) => {
                execute_motion(buffer, &mut window.cursor, dir, 1);
            }

            Intent::CursorGoto(pos) => {
                window.cursor.move_to(pos.line, pos.col);
                clamp_cursor(buffer, &mut window.cursor);
            }

            Intent::CursorLineStart => {
                kjxlkj_core_edit::move_to_line_start(&mut window.cursor);
            }

            Intent::CursorFirstNonBlank => {
                kjxlkj_core_edit::move_to_first_non_blank(buffer, &mut window.cursor);
            }

            Intent::CursorLineEnd => {
                kjxlkj_core_edit::move_to_line_end(buffer, &mut window.cursor);
            }

            Intent::CursorFileStart => {
                kjxlkj_core_edit::move_to_file_start(&mut window.cursor);
            }

            Intent::CursorFileEnd => {
                kjxlkj_core_edit::move_to_file_end(buffer, &mut window.cursor);
            }

            Intent::CursorGotoLine(line) => {
                kjxlkj_core_edit::move_to_line(buffer, &mut window.cursor, line);
            }

            Intent::EnterMode(mode) => {
                self.mode_state.enter_mode(mode);
                if mode.is_visual() {
                    self.mode_state.visual_anchor = Some(window.cursor.position);
                }
            }

            Intent::ExitToNormal => {
                self.mode_state.exit_to_normal();
            }

            Intent::InsertText(text) => {
                kjxlkj_core_edit::insert_text(buffer, &mut window.cursor, &text)?;
            }

            Intent::InsertNewline => {
                let line_ending = buffer.meta().line_ending.as_str();
                if self.mode_state.mode == Mode::Normal {
                    kjxlkj_core_edit::open_line_below(buffer, &mut window.cursor, line_ending)?;
                } else {
                    kjxlkj_core_edit::insert_newline(buffer, &mut window.cursor, line_ending)?;
                }
            }

            Intent::DeleteBackward => {
                kjxlkj_core_edit::delete_backward(buffer, &mut window.cursor)?;
            }

            Intent::DeleteForward => {
                kjxlkj_core_edit::delete_forward(buffer, &window.cursor)?;
            }

            Intent::DeleteChar => {
                let deleted = kjxlkj_core_edit::delete_char(buffer, &window.cursor)?;
                self.registers.set(RegisterName::Unnamed, RegisterContent::char(deleted));
            }

            Intent::DeleteLine => {
                let content = kjxlkj_core_edit::delete_line(buffer, &window.cursor)?;
                self.registers.set(RegisterName::Unnamed, content);
                clamp_cursor(buffer, &mut window.cursor);
            }

            Intent::DeleteRange(_range) => {
                // For visual mode - use the actual selection
                if let Some(range) = self.mode_state.visual_range(window.cursor.position) {
                    let deleted = buffer.delete(range)?;
                    self.registers.set(RegisterName::Unnamed, RegisterContent::char(deleted));
                    window.cursor.move_to(range.start.line, range.start.col);
                    clamp_cursor(buffer, &mut window.cursor);
                }
            }

            Intent::YankLine => {
                if let Some(content) = kjxlkj_core_edit::yank_line(buffer, &window.cursor) {
                    self.registers.set(RegisterName::Unnamed, content);
                    self.set_message("1 line yanked", MessageLevel::Info);
                }
            }

            Intent::YankRange(_range) => {
                if let Some(range) = self.mode_state.visual_range(window.cursor.position) {
                    let text = buffer.rope().slice(
                        buffer.pos_to_offset(range.start).unwrap_or(0)..
                        buffer.pos_to_offset(range.end).unwrap_or(0)
                    ).to_string();
                    self.registers.set(RegisterName::Unnamed, RegisterContent::char(text));
                    self.set_message("Yanked", MessageLevel::Info);
                }
            }

            Intent::PasteAfter => {
                if let Some(content) = self.registers.get(RegisterName::Unnamed) {
                    kjxlkj_core_edit::paste_after(buffer, &mut window.cursor, content)?;
                }
            }

            Intent::PasteBefore => {
                if let Some(content) = self.registers.get(RegisterName::Unnamed) {
                    kjxlkj_core_edit::paste_before(buffer, &mut window.cursor, content)?;
                }
            }

            Intent::Undo => {
                if let Some(history) = self.undo_histories.get_mut(&window.buffer_id) {
                    if let Some(tx) = history.undo() {
                        // Apply inverse operations
                        for op in tx.ops {
                            match op {
                                EditOp::Insert { pos, text } => {
                                    buffer.insert(pos, &text)?;
                                }
                                EditOp::Delete { range, .. } => {
                                    buffer.delete(range)?;
                                }
                            }
                        }
                        window.cursor.move_to(tx.cursor_after.line, tx.cursor_after.col);
                    }
                }
            }

            Intent::Redo => {
                if let Some(history) = self.undo_histories.get_mut(&window.buffer_id) {
                    if let Some(tx) = history.redo() {
                        for op in tx.ops {
                            match op {
                                EditOp::Insert { pos, text } => {
                                    buffer.insert(pos, &text)?;
                                }
                                EditOp::Delete { range, .. } => {
                                    buffer.delete(range)?;
                                }
                            }
                        }
                        window.cursor.move_to(tx.cursor_after.line, tx.cursor_after.col);
                    }
                }
            }

            Intent::ExecuteCommand(cmd) => {
                let parsed = parse_command(&cmd);
                // Recursively apply the parsed intent
                return self.apply_intent(parsed);
            }

            Intent::WriteBuffer { path, force: _ } => {
                let write_path = path.or_else(|| buffer.path().cloned());
                if let Some(p) = write_path {
                    // In real implementation, write to file
                    buffer.mark_clean();
                    self.set_message(&format!("Written: {}", p.display()), MessageLevel::Info);
                } else {
                    self.set_message("No file name", MessageLevel::Error);
                }
            }

            Intent::OpenFile(path) => {
                // In real implementation, read file content
                let content = ""; // Would be read from filesystem
                let new_buffer = TextBuffer::from_file(path, content);
                let new_id = new_buffer.id();
                self.buffers.insert(new_id, new_buffer);
                self.undo_histories.insert(new_id, UndoHistory::new());
                window.buffer_id = new_id;
                window.cursor = Cursor::default();
            }

            Intent::CloseBuffer { force } => {
                if !force && buffer.is_modified() {
                    self.set_message("No write since last change", MessageLevel::Error);
                } else {
                    self.should_quit = true;
                }
            }

            Intent::Quit { force } => {
                if !force && buffer.is_modified() {
                    self.set_message("No write since last change (add ! to override)", MessageLevel::Error);
                } else {
                    self.should_quit = true;
                }
            }

            Intent::RunExternalCommand(cmd) => {
                // Would execute via terminal service
                self.set_message(&format!(":{}", cmd), MessageLevel::Info);
            }

            Intent::ClearSelection => {
                self.mode_state.visual_anchor = None;
            }

            Intent::ExtendSelection(dir) => {
                execute_motion(buffer, &mut window.cursor, dir, 1);
            }

            Intent::Scroll(_dir) => {
                // Handle scrolling
            }

            Intent::CenterCursor => {
                let height = window.viewport.height as u32;
                let cursor_line = window.cursor.line();
                window.viewport.top_line = cursor_line.saturating_sub(height / 2);
            }

            Intent::SetRegister(_name) => {
                // Set pending register
            }

            Intent::Noop => {}

            _ => {}
        }

        // Ensure cursor is valid (but allow past-end in insert mode)
        let window = self.windows.get_mut(&self.active_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();
        clamp_cursor_for_mode(buffer, &mut window.cursor, self.mode_state.mode);

        // Update viewport to follow cursor
        let scrolloff = 3;
        window.viewport.ensure_line_visible(window.cursor.line(), scrolloff);

        Ok(())
    }

    /// Set a status message.
    pub fn set_message(&mut self, text: &str, level: MessageLevel) {
        self.message = Some(StatusMessage {
            text: text.to_string(),
            level,
        });
    }

    /// Clear the status message.
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Create a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let window = self.windows.get(&self.active_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();

        let meta = buffer.meta();
        let viewport = &window.viewport;

        // Build visible lines
        let mut lines = Vec::new();
        let start_line = viewport.top_line as usize;
        let end_line = (viewport.top_line + viewport.height as u32) as usize;

        for line_idx in start_line..end_line.min(buffer.line_count()) {
            let text = buffer.line(line_idx).unwrap_or_default();
            lines.push(LineSnapshot {
                number: (line_idx + 1) as u32,
                text,
                is_cursor_line: line_idx == window.cursor.line() as usize,
            });
        }

        let status = build_status(
            self.mode_state.mode,
            &meta,
            &window.cursor,
            buffer.line_count(),
        );

        let selection = self.mode_state.visual_range(window.cursor.position);

        EditorSnapshot {
            active_window: WindowSnapshot {
                id: window.id,
                buffer_meta: meta,
                lines,
                cursor: window.cursor,
                cursor_style: self.mode_state.mode.cursor_style(),
                viewport: window.viewport,
                selection,
            },
            mode: self.mode_state.mode,
            status,
            command_line: if self.mode_state.mode == Mode::Command {
                Some(self.mode_state.command_line.clone())
            } else {
                None
            },
            message: self.message.clone(),
        }
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
