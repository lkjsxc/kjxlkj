//! Core editor state.

use kjxlkj_core_edit::{Buffer, CursorOps};
use kjxlkj_core_mode::{CommandLineState, KeyInput, ModeHandler};
use kjxlkj_core_types::{BufferId, EditorAction, EditorEvent, Mode, Motion, Operator};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, SnapshotSeq, StatusLine, Viewport};

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
    /// Monotonically increasing snapshot sequence.
    snapshot_seq: SnapshotSeq,
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
            snapshot_seq: SnapshotSeq::new(0),
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

    /// Returns the current snapshot sequence number.
    pub fn snapshot_seq(&self) -> SnapshotSeq {
        self.snapshot_seq
    }

    /// Sets the terminal size and updates viewport.
    pub fn set_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
        let editor_height = height.saturating_sub(2) as usize;
        self.viewport.width = width as usize;
        self.viewport.height = editor_height;
    }

    /// Handles a key input and returns events.
    /// Increments snapshot sequence for each input processed.
    pub fn handle_key(&mut self, key: KeyInput) -> Vec<EditorEvent> {
        self.snapshot_seq = self.snapshot_seq.next();
        let old_mode = self.mode();
        let action = self.mode_handler.handle_key(key);
        let mut events = self.apply_action(action);

        // Check for mode change (happens in mode_handler, not apply_action)
        if self.mode() != old_mode {
            events.push(EditorEvent::ModeChanged(self.mode()));
        }

        events
    }

    /// Applies an action and returns events.
    pub fn apply_action(&mut self, action: EditorAction) -> Vec<EditorEvent> {
        let mut events = Vec::new();

        match action {
            EditorAction::CursorLeft => self.buffer.move_left(),
            EditorAction::CursorRight => self.buffer.move_right(),
            EditorAction::CursorUp => self.buffer.move_up(),
            EditorAction::CursorDown => self.buffer.move_down(),
            EditorAction::LineStart => self.buffer.move_line_start(),
            EditorAction::LineEnd => self.buffer.move_line_end(),
            EditorAction::FirstNonBlank => self.buffer.move_first_non_blank(),
            EditorAction::WordForward => self.buffer.move_word_forward(),
            EditorAction::WORDForward => self.buffer.move_word_forward(), // TODO: WORD semantics
            EditorAction::WordBackward => self.buffer.move_word_backward(),
            EditorAction::WORDBackward => self.buffer.move_word_backward(), // TODO: WORD semantics
            EditorAction::WordEnd => self.buffer.move_word_end(),
            EditorAction::WORDEnd => self.buffer.move_word_end(), // TODO: WORD semantics
            EditorAction::FileStart => self.buffer.move_file_start(),
            EditorAction::FileEnd => self.buffer.move_file_end(),
            EditorAction::InsertChar(ch) => self.buffer.insert_char(ch),
            EditorAction::InsertNewline => self.buffer.insert_newline(),
            EditorAction::DeleteCharBefore => self.buffer.delete_char_before(),
            EditorAction::DeleteCharAt => self.buffer.delete_char_at(),
            EditorAction::DeleteLine => self.buffer.delete_line(),
            EditorAction::YankLine => self.buffer.yank_line(),
            EditorAction::PasteAfter => self.buffer.paste_after(),
            EditorAction::OperatorMotion { operator, motion, count } => {
                self.apply_operator_motion(operator, motion, count);
            }
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

    /// Apply an operator over a motion range.
    fn apply_operator_motion(&mut self, operator: Operator, motion: Motion, count: Option<u32>) {
        let count = count.unwrap_or(1) as usize;
        
        // Get start position
        let start = self.buffer.cursor().position;
        
        // Execute motion to get end position
        for _ in 0..count {
            match motion {
                Motion::Left => self.buffer.move_left(),
                Motion::Right => self.buffer.move_right(),
                Motion::Up => self.buffer.move_up(),
                Motion::Down => self.buffer.move_down(),
                Motion::LineStart => self.buffer.move_line_start(),
                Motion::LineEnd => self.buffer.move_line_end(),
                Motion::FirstNonBlank => self.buffer.move_first_non_blank(),
                Motion::WordForward => self.buffer.move_word_forward(),
                Motion::WordBackward => self.buffer.move_word_backward(),
                Motion::WordEnd => self.buffer.move_word_end(),
                Motion::FileStart => self.buffer.move_file_start(),
                Motion::FileEnd => self.buffer.move_file_end(),
                Motion::CurrentLine => {
                    // For linewise operations, delete/yank whole lines
                    for _ in 0..count {
                        match operator {
                            Operator::Delete => self.buffer.delete_line(),
                            Operator::Yank => self.buffer.yank_line(),
                            Operator::Change => {
                                self.buffer.delete_line();
                                // Stay on the new line
                            }
                            Operator::Indent => {
                                self.buffer.indent_line();
                            }
                            Operator::Outdent => {
                                self.buffer.outdent_line();
                            }
                        }
                    }
                    return;
                }
            }
        }

        let end = self.buffer.cursor().position;
        
        // Determine range (handle motion going backwards)
        let (range_start, mut range_end) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };

        // For inclusive motions (like $, e, etc.), we need to include the end character
        // Ranges are exclusive, so we add 1 to the end column
        let is_inclusive = matches!(motion, Motion::LineEnd | Motion::WordEnd);
        if is_inclusive {
            range_end.col += 1;
        }

        // Apply operator over range
        match operator {
            Operator::Delete => {
                self.buffer.delete_range(range_start, range_end);
            }
            Operator::Yank => {
                self.buffer.yank_range(range_start, range_end);
            }
            Operator::Change => {
                self.buffer.delete_range(range_start, range_end);
            }
            Operator::Indent | Operator::Outdent => {
                // For indent/outdent, need linewise behavior
                // TODO: implement range indentation
            }
        }
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
            seq: self.snapshot_seq,
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

    fn esc() -> KeyInput {
        KeyInput {
            code: KeyCode::Escape,
            modifiers: Modifiers::default(),
        }
    }

    fn enter() -> KeyInput {
        KeyInput {
            code: KeyCode::Enter,
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
        state.handle_key(esc());
        state.handle_key(key(':'));
        state.handle_key(key('q'));
        state.handle_key(enter());
        assert!(!state.is_quit_requested());
    }

    // === Runtime Ordering Invariant Tests ===

    #[test]
    fn snapshot_seq_is_monotonic() {
        let mut state = EditorState::new();
        let mut prev_seq = state.snapshot_seq();

        // Each key input should produce a newer sequence
        for _ in 0..10 {
            state.handle_key(key('j'));
            let new_seq = state.snapshot_seq();
            assert!(new_seq > prev_seq, "Snapshot sequence must be monotonically increasing");
            prev_seq = new_seq;
        }
    }

    #[test]
    fn snapshot_reflects_latest_state() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        state.handle_key(key('a'));
        state.handle_key(key('b'));
        state.handle_key(key('c'));

        let snapshot = state.snapshot();

        // Snapshot must contain all typed characters
        assert_eq!(snapshot.buffer.lines.join(""), "abc");
        // Snapshot seq must match internal state
        assert_eq!(snapshot.seq, state.snapshot_seq());
    }

    #[test]
    fn snapshot_is_immutable_clone() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        state.handle_key(key('x'));

        // Take snapshot
        let snapshot1 = state.snapshot();
        let seq1 = snapshot1.seq;

        // Modify state
        state.handle_key(key('y'));

        // Original snapshot unchanged
        assert_eq!(snapshot1.seq, seq1);
        assert_eq!(snapshot1.buffer.lines.join(""), "x");

        // New snapshot reflects changes
        let snapshot2 = state.snapshot();
        assert!(snapshot2.seq > seq1);
        assert_eq!(snapshot2.buffer.lines.join(""), "xy");
    }

    #[test]
    fn rapid_typing_maintains_order() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));

        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut prev_seq = state.snapshot_seq();

        for ch in &chars {
            state.handle_key(key(*ch));
            let seq = state.snapshot_seq();
            assert!(seq > prev_seq);
            prev_seq = seq;
        }

        // All characters in order
        let content = state.buffer().content();
        assert_eq!(content, "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn event_to_snapshot_determinism() {
        // Same inputs should produce same outputs
        let run = || {
            let mut state = EditorState::new();
            state.handle_key(key('i'));
            state.handle_key(key('h'));
            state.handle_key(key('e'));
            state.handle_key(key('l'));
            state.handle_key(key('l'));
            state.handle_key(key('o'));
            state.handle_key(esc());
            state.snapshot()
        };

        let snap1 = run();
        let snap2 = run();

        assert_eq!(snap1.buffer.lines, snap2.buffer.lines);
        assert_eq!(snap1.buffer.cursor.position, snap2.buffer.cursor.position);
        assert_eq!(snap1.status.mode, snap2.status.mode);
    }

    #[test]
    fn mode_change_produces_event() {
        let mut state = EditorState::new();
        let events = state.handle_key(key('i'));

        assert!(events.iter().any(|e| matches!(e, EditorEvent::ModeChanged(Mode::Insert))));
    }

    #[test]
    fn resize_preserves_state() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        state.handle_key(key('t'));
        state.handle_key(key('e'));
        state.handle_key(key('s'));
        state.handle_key(key('t'));

        let content_before = state.buffer().content();

        // Simulate resize storm
        for w in 20..120 {
            state.set_terminal_size(w, 24);
        }
        for h in 10..50 {
            state.set_terminal_size(80, h);
        }

        // Content must be unchanged
        assert_eq!(state.buffer().content(), content_before);
    }

    #[test]
    fn delete_word_forward() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start of buffer
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // dw deletes "hello "
        state.handle_key(key('d'));
        state.handle_key(key('w'));
        
        assert_eq!(state.buffer().content(), "world");
    }

    #[test]
    fn yank_word_and_paste() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // yw yanks "hello "
        state.handle_key(key('y'));
        state.handle_key(key('w'));
        
        // Content unchanged after yank
        assert_eq!(state.buffer().content(), "hello world");
        
        // Go to end and paste
        state.handle_key(key('$'));
        state.handle_key(key('p'));
        
        assert!(state.buffer().content().contains("hello"));
    }

    #[test]
    fn delete_to_line_end() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // Move to 'w'
        state.handle_key(key('w'));
        
        // d$ deletes to end of line
        state.handle_key(key('d'));
        state.handle_key(key('$'));
        
        assert_eq!(state.buffer().content(), "hello ");
    }

    #[test]
    fn delete_line_with_dd() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "line1".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(enter());
        for ch in "line2".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to first line
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        
        // dd deletes the line
        state.handle_key(key('d'));
        state.handle_key(key('d'));
        
        assert_eq!(state.buffer().content(), "line2");
    }

    #[test]
    fn operator_pending_escape_cancels() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Start delete operator
        state.handle_key(key('d'));
        
        // Escape should cancel
        state.handle_key(esc());
        
        // Content unchanged
        assert_eq!(state.buffer().content(), "hello");
        
        // Should be back in normal mode
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn indent_line_with_double_greater() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "code".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        
        // >> indents line
        state.handle_key(key('>'));
        state.handle_key(key('>'));
        
        assert!(state.buffer().content().starts_with("    "));
    }

    #[test]
    fn outdent_line_with_double_less() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "    code".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        
        // << outdents line
        state.handle_key(key('<'));
        state.handle_key(key('<'));
        
        assert_eq!(state.buffer().content(), "code");
    }
}
