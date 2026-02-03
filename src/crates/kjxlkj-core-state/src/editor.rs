//! Core editor state.

use kjxlkj_core_edit::{Buffer, CursorOps};
use kjxlkj_core_mode::{CommandLineState, KeyInput, ModeHandler};
use kjxlkj_core_types::{BufferId, EditorAction, EditorEvent, Mode};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};

use crate::CommandParser;

/// The core editor state - single writer, owns all mutable state.
#[derive(Debug)]
pub struct EditorState {
    buffer: Buffer,
    mode_handler: ModeHandler,
    viewport: Viewport,
    terminal_size: (u16, u16),
    next_buffer_id: u64,
    status_message: Option<String>,
    quit_requested: bool,
    scroll_off: usize,
}

impl EditorState {
    /// Creates a new editor state.
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(BufferId::new(1), "[No Name]".to_string()),
            mode_handler: ModeHandler::new(),
            viewport: Viewport::new(0, 24, 0, 80),
            terminal_size: (80, 24),
            next_buffer_id: 2,
            status_message: None,
            quit_requested: false,
            scroll_off: 3,
        }
    }

    /// Creates editor state with initial content.
    pub fn with_content(content: &str) -> Self {
        let mut state = Self::new();
        state.buffer = Buffer::from_content(
            BufferId::new(1),
            "[No Name]".to_string(),
            content,
        );
        state
    }

    pub fn mode(&self) -> Mode {
        self.mode_handler.mode()
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn is_quit_requested(&self) -> bool {
        self.quit_requested
    }

    pub fn command_line(&self) -> &CommandLineState {
        self.mode_handler.command_line()
    }

    /// Sets the terminal size and updates viewport.
    pub fn set_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
        let editor_height = height.saturating_sub(2) as usize;
        self.viewport.width = width as usize;
        self.viewport.height = editor_height;
    }

    /// Handles a key input and returns events.
    pub fn handle_key(&mut self, key: KeyInput) -> Vec<EditorEvent> {
        let action = self.mode_handler.handle_key(key);
        self.apply_action(action)
    }

    /// Applies an action and returns events.
    pub fn apply_action(&mut self, action: EditorAction) -> Vec<EditorEvent> {
        let mut events = Vec::new();
        let old_mode = self.mode();

        match action {
            EditorAction::CursorLeft => self.buffer.move_left(),
            EditorAction::CursorRight => self.buffer.move_right(),
            EditorAction::CursorUp => self.buffer.move_up(),
            EditorAction::CursorDown => self.buffer.move_down(),
            EditorAction::LineStart => self.buffer.move_line_start(),
            EditorAction::LineEnd => self.buffer.move_line_end(),
            EditorAction::InsertChar(ch) => self.buffer.insert_char(ch),
            EditorAction::InsertNewline => self.buffer.insert_newline(),
            EditorAction::DeleteCharBefore => self.buffer.delete_char_before(),
            EditorAction::DeleteCharAt => self.buffer.delete_char_at(),
            EditorAction::DeleteLine => self.buffer.delete_line(),
            EditorAction::YankLine => self.buffer.yank_line(),
            EditorAction::PasteAfter => self.buffer.paste_after(),
            EditorAction::Undo => {
                if !self.buffer.undo() {
                    self.status_message = Some("Already at oldest change".to_string());
                }
            }
            EditorAction::Redo => {
                if !self.buffer.redo() {
                    self.status_message = Some("Already at newest change".to_string());
                }
            }
            EditorAction::EnterInsertMode => {}
            EditorAction::EnterInsertModeAfter => {
                self.buffer.move_right();
            }
            EditorAction::EnterInsertModeEndOfLine => {
                self.buffer.move_line_end();
                self.buffer.move_right();
            }
            EditorAction::OpenLineBelow => {
                self.buffer.move_line_end();
                self.buffer.insert_newline();
            }
            EditorAction::EnterVisualMode => {}
            EditorAction::EnterVisualLineMode => {}
            EditorAction::EnterReplaceMode => {}
            EditorAction::EnterCommandMode => {}
            EditorAction::ReturnToNormalMode => {
                self.buffer.clamp_cursor();
            }
            EditorAction::ExecuteCommand(cmd) => {
                let parsed = CommandParser::parse(&cmd);
                events.extend(self.apply_action(parsed));
            }
            EditorAction::Quit { force } => {
                if !force && self.buffer.is_modified() {
                    self.status_message = Some(
                        "No write since last change (add ! to override)".to_string(),
                    );
                } else {
                    self.quit_requested = true;
                    events.push(EditorEvent::QuitRequested);
                }
            }
            EditorAction::Write { path } => {
                let write_path = path
                    .or_else(|| self.buffer.path().map(|s| s.to_string()));
                if let Some(p) = write_path {
                    self.buffer.set_path(p.clone());
                    self.buffer.mark_saved();
                    self.status_message = Some(format!("\"{}\" written", p));
                } else {
                    self.status_message = Some("No file name".to_string());
                }
            }
            EditorAction::EditFile { path, force: _ } => {
                self.status_message = Some(format!("Edit: {}", path));
            }
            EditorAction::RunExternal(cmd) => {
                self.status_message = Some(format!(":{}", cmd));
            }
            EditorAction::Nop => {}
        }

        self.update_viewport();

        if self.mode() != old_mode {
            events.push(EditorEvent::ModeChanged(self.mode()));
        }

        events
    }

    /// Applies a service event.
    pub fn apply_service_event(
        &mut self,
        event: kjxlkj_core_types::ServiceEvent,
    ) -> Vec<EditorEvent> {
        use kjxlkj_core_types::ServiceEvent;
        match event {
            ServiceEvent::FileRead { path, content } => {
                self.buffer = Buffer::from_content(
                    BufferId::new(self.next_buffer_id),
                    path.clone(),
                    &content,
                );
                self.buffer.set_path(path);
                self.next_buffer_id += 1;
                vec![]
            }
            ServiceEvent::FileWritten { path } => {
                self.buffer.mark_saved();
                self.status_message = Some(format!("\"{}\" written", path));
                vec![]
            }
            ServiceEvent::CommandOutput { output } => {
                self.status_message = Some(output);
                vec![]
            }
            ServiceEvent::Error { message } => {
                self.status_message = Some(format!("Error: {}", message));
                vec![EditorEvent::ErrorMessage(message)]
            }
        }
    }

    fn update_viewport(&mut self) {
        let cursor_line = self.buffer.cursor().position.line as usize;
        self.viewport.follow_cursor(cursor_line, self.scroll_off);
    }

    /// Produces a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let lines: Vec<String> = (0..self.buffer.line_count())
            .filter_map(|i| self.buffer.line(i))
            .collect();

        let buffer_snapshot = BufferSnapshot {
            id: self.buffer.id(),
            version: self.buffer.version(),
            name: self.buffer.name().to_string(),
            lines,
            cursor: self.buffer.cursor(),
            viewport: self.viewport,
            modified: self.buffer.is_modified(),
        };

        let status = StatusLine {
            mode: self.mode(),
            file_name: self
                .buffer
                .path()
                .unwrap_or(self.buffer.name())
                .to_string(),
            modified: self.buffer.is_modified(),
            cursor_line: self.buffer.cursor().position.line + 1,
            cursor_col: self.buffer.cursor().position.col + 1,
            line_count: self.buffer.line_count(),
            message: self.status_message.clone(),
        };

        let command_line = if self.mode() == Mode::Command {
            Some(format!(":{}", self.command_line().content))
        } else {
            None
        };

        EditorSnapshot {
            buffer: buffer_snapshot,
            status,
            command_line,
            terminal_size: self.terminal_size,
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
    use kjxlkj_core_mode::{KeyCode, Modifiers};

    fn key(ch: char) -> KeyInput {
        KeyInput {
            code: KeyCode::Char(ch),
            modifiers: Modifiers::default(),
        }
    }

    #[test]
    fn initial_mode_is_normal() {
        let state = EditorState::new();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn insert_mode_typing() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        state.handle_key(key('h'));
        state.handle_key(key('i'));
        assert_eq!(state.buffer().content(), "hi");
    }

    #[test]
    fn quit_modified_buffer_fails() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        state.handle_key(key('x'));
        state.handle_key(KeyInput {
            code: KeyCode::Escape,
            modifiers: Modifiers::default(),
        });
        state.handle_key(key(':'));
        state.handle_key(key('q'));
        state.handle_key(KeyInput {
            code: KeyCode::Enter,
            modifiers: Modifiers::default(),
        });
        assert!(!state.is_quit_requested());
    }
}
