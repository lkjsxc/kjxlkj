//! Editor state implementation.

use crate::Registers;
use kjxlkj_core_edit::{apply_motion, Motion};
use kjxlkj_core_mode::{ModeHandler, ModeResult, NormalMode};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    BufferId, Cursor, EditorEvent, Intent, KeyEvent, Mode, MotionIntent, Position, Register,
    ScrollIntent, Selection, SelectionKind,
};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};
use kjxlkj_core_undo::{Edit, Transaction, UndoHistory};
use std::collections::HashMap;

/// Complete editor state.
pub struct EditorState {
    /// Current buffer.
    buffer: TextBuffer,
    /// Next buffer ID.
    next_buffer_id: u64,
    /// Cursor position.
    cursor: Cursor,
    /// Current mode.
    mode: Mode,
    /// Current selection (visual mode).
    selection: Option<Selection>,
    /// Viewport.
    viewport: Viewport,
    /// Register storage.
    registers: Registers,
    /// Undo history.
    undo_history: UndoHistory,
    /// Marks.
    marks: HashMap<char, Position>,
    /// Status message.
    status_message: Option<(String, bool)>,
    /// Terminal dimensions.
    width: u16,
    height: u16,
    /// Whether editor should quit.
    should_quit: bool,
    /// Current transaction (for grouping edits).
    current_transaction: Option<Transaction>,
    /// Last change for dot repeat.
    last_change: Option<Vec<KeyEvent>>,
    /// Recording change.
    recording_change: Vec<KeyEvent>,
    /// Macro recording.
    recording_macro: Option<char>,
    /// Recorded macros.
    macros: HashMap<char, Vec<KeyEvent>>,
    /// Command line content.
    command_line: Option<String>,
    /// Normal mode handler (for stateful multi-key sequences like dd, yy).
    normal_mode: NormalMode,
}

impl EditorState {
    /// Create a new editor state.
    pub fn new() -> Self {
        let buffer = TextBuffer::new(BufferId::new(1));
        Self {
            buffer,
            next_buffer_id: 2,
            cursor: Cursor::new(0, 0),
            mode: Mode::Normal,
            selection: None,
            viewport: Viewport::new(0, 24, 0, 80),
            registers: Registers::new(),
            undo_history: UndoHistory::new(),
            marks: HashMap::new(),
            status_message: None,
            width: 80,
            height: 24,
            should_quit: false,
            current_transaction: None,
            last_change: None,
            recording_change: Vec::new(),
            recording_macro: None,
            macros: HashMap::new(),
            command_line: None,
            normal_mode: NormalMode::new(),
        }
    }

    /// Load content into the buffer.
    pub fn load_content(&mut self, content: &str) {
        self.buffer.replace_all(content);
        self.buffer.mark_saved(); // Loading is not a modification
        self.cursor = Cursor::new(0, 0);
        self.undo_history.clear();
    }

    /// Load a file.
    pub fn load_file(&mut self, path: std::path::PathBuf, content: &str) {
        self.buffer = TextBuffer::from_file(
            BufferId::new(self.next_buffer_id),
            path,
            content,
        );
        self.next_buffer_id += 1;
        self.cursor = Cursor::new(0, 0);
        self.undo_history.clear();
    }

    /// Get a reference to the buffer.
    pub fn buffer(&self) -> &TextBuffer {
        &self.buffer
    }

    /// Get a mutable reference to the buffer.
    pub fn buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.buffer
    }

    /// Get the buffer content.
    pub fn content(&self) -> String {
        self.buffer.to_string()
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Get the cursor position.
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Check if the editor should quit.
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Resize the editor.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        // Reserve 2 lines for status and command
        self.viewport.height = (height as usize).saturating_sub(2);
        self.viewport.width = width as usize;
        
        // Re-clamp viewport to ensure cursor remains visible after resize
        self.ensure_cursor_visible();
    }

    /// Process an editor event.
    pub fn handle_event(&mut self, event: EditorEvent) {
        match event {
            EditorEvent::Key(key) => self.handle_key(key),
            EditorEvent::Resize { width, height } => self.resize(width, height),
            EditorEvent::Quit => self.should_quit = true,
            _ => {}
        }
    }

    /// Process a key event.
    pub fn handle_key(&mut self, key: KeyEvent) {
        // Record for macro
        if self.recording_macro.is_some() {
            self.recording_change.push(key.clone());
        }

        // Clear status message on key press
        self.status_message = None;

        // Handle based on mode
        let intents = self.get_mode_intents(&key);

        // Apply intents
        for intent in intents {
            self.apply_intent(intent);
        }

        // Ensure cursor is valid
        self.clamp_cursor();

        // Scroll viewport to follow cursor
        self.viewport
            .scroll_to_line(self.cursor.line(), self.buffer.line_count());
    }

    fn get_mode_intents(&mut self, key: &KeyEvent) -> Vec<Intent> {
        match self.mode {
            Mode::Normal => {
                // Use the stateful NormalMode handler for multi-key sequences
                match self.normal_mode.handle_key(key) {
                    ModeResult::Consumed(intents) => intents,
                    ModeResult::Pending => vec![], // Wait for more keys
                    ModeResult::Ignored => self.parse_normal_key(key), // Fallback
                }
            }
            Mode::Insert => self.parse_insert_key(key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => self.parse_visual_key(key),
            Mode::Command => self.parse_command_key(key),
            Mode::Replace => self.parse_replace_key(key),
        }
    }

    fn parse_normal_key(&mut self, key: &KeyEvent) -> Vec<Intent> {
        use kjxlkj_core_types::KeyCode;

        let mut intents = Vec::new();

        if key.modifiers.ctrl {
            match &key.code {
                KeyCode::Char('r') => intents.push(Intent::Redo),
                KeyCode::Char('d') => intents.push(Intent::Scroll(ScrollIntent::HalfPageDown)),
                KeyCode::Char('u') => intents.push(Intent::Scroll(ScrollIntent::HalfPageUp)),
                KeyCode::Char('f') => intents.push(Intent::Scroll(ScrollIntent::PageDown)),
                KeyCode::Char('b') => intents.push(Intent::Scroll(ScrollIntent::PageUp)),
                KeyCode::Char('e') => intents.push(Intent::Scroll(ScrollIntent::LineDown)),
                KeyCode::Char('y') => intents.push(Intent::Scroll(ScrollIntent::LineUp)),
                KeyCode::Char('a') => intents.push(Intent::Increment(1)),
                KeyCode::Char('x') => intents.push(Intent::Increment(-1)),
                KeyCode::Char('v') => intents.push(Intent::SwitchMode(Mode::VisualBlock)),
                _ => {}
            }
            return intents;
        }

        match &key.code {
            KeyCode::Char('h') | KeyCode::Left => intents.push(Intent::Motion(MotionIntent::Left)),
            KeyCode::Char('l') | KeyCode::Right => {
                intents.push(Intent::Motion(MotionIntent::Right))
            }
            KeyCode::Char('j') | KeyCode::Down => intents.push(Intent::Motion(MotionIntent::Down)),
            KeyCode::Char('k') | KeyCode::Up => intents.push(Intent::Motion(MotionIntent::Up)),
            KeyCode::Char('0') => intents.push(Intent::Motion(MotionIntent::LineStart)),
            KeyCode::Char('^') => intents.push(Intent::Motion(MotionIntent::FirstNonBlank)),
            KeyCode::Char('$') => intents.push(Intent::Motion(MotionIntent::LineEnd)),
            KeyCode::Char('w') => intents.push(Intent::Motion(MotionIntent::WordStart)),
            KeyCode::Char('b') => intents.push(Intent::Motion(MotionIntent::WordStartBack)),
            KeyCode::Char('e') => intents.push(Intent::Motion(MotionIntent::WordEnd)),
            KeyCode::Char('G') => intents.push(Intent::Motion(MotionIntent::FileEnd)),
            KeyCode::Char('i') => intents.push(Intent::SwitchMode(Mode::Insert)),
            KeyCode::Char('a') => {
                intents.push(Intent::Motion(MotionIntent::Right));
                intents.push(Intent::SwitchMode(Mode::Insert));
            }
            KeyCode::Char('I') => {
                intents.push(Intent::Motion(MotionIntent::FirstNonBlank));
                intents.push(Intent::SwitchMode(Mode::Insert));
            }
            KeyCode::Char('A') => {
                intents.push(Intent::Motion(MotionIntent::LineEnd));
                intents.push(Intent::SwitchMode(Mode::Insert));
            }
            KeyCode::Char('o') => intents.push(Intent::OpenLine { below: true }),
            KeyCode::Char('O') => intents.push(Intent::OpenLine { below: false }),
            KeyCode::Char('v') => intents.push(Intent::SwitchMode(Mode::Visual)),
            KeyCode::Char('V') => intents.push(Intent::SwitchMode(Mode::VisualLine)),
            KeyCode::Char('R') => intents.push(Intent::SwitchMode(Mode::Replace)),
            KeyCode::Char(':') => intents.push(Intent::SwitchMode(Mode::Command)),
            KeyCode::Char('x') => intents.push(Intent::Delete { linewise: false, count: 1, motion: None }),
            KeyCode::Char('X') => {
                intents.push(Intent::Motion(MotionIntent::Left));
                intents.push(Intent::Delete { linewise: false, count: 1, motion: None });
            }
            KeyCode::Char('p') => intents.push(Intent::Paste {
                before: false,
                cursor_at_end: false,
            }),
            KeyCode::Char('P') => intents.push(Intent::Paste {
                before: true,
                cursor_at_end: false,
            }),
            KeyCode::Char('u') => intents.push(Intent::Undo),
            KeyCode::Char('J') => intents.push(Intent::JoinLines { add_space: true }),
            KeyCode::Char('~') => intents.push(Intent::ToggleCase),
            _ => {}
        }

        intents
    }

    fn parse_insert_key(&mut self, key: &KeyEvent) -> Vec<Intent> {
        use kjxlkj_core_types::KeyCode;

        let mut intents = Vec::new();

        if key.modifiers.ctrl {
            match &key.code {
                KeyCode::Char('h') => intents.push(Intent::InsertText("\x08".to_string())),
                KeyCode::Char('w') => {} // Delete word
                KeyCode::Char('u') => {} // Delete to line start
                KeyCode::Char('j') | KeyCode::Char('m') => {
                    // Ctrl-j and Ctrl-m are newline (same as Enter)
                    intents.push(Intent::InsertText("\n".to_string()))
                }
                KeyCode::Char('r') => {
                    // TODO: Insert from register (already handled in mode handler)
                }
                _ => {}
            }
            return intents;
        }

        match &key.code {
            KeyCode::Escape => intents.push(Intent::SwitchMode(Mode::Normal)),
            KeyCode::Enter => intents.push(Intent::InsertText("\n".to_string())),
            KeyCode::Backspace => intents.push(Intent::InsertText("\x08".to_string())),
            KeyCode::Tab => intents.push(Intent::InsertText("    ".to_string())),
            KeyCode::Char(c) => intents.push(Intent::InsertText(c.to_string())),
            KeyCode::Left => intents.push(Intent::Motion(MotionIntent::Left)),
            KeyCode::Right => intents.push(Intent::Motion(MotionIntent::Right)),
            KeyCode::Up => intents.push(Intent::Motion(MotionIntent::Up)),
            KeyCode::Down => intents.push(Intent::Motion(MotionIntent::Down)),
            _ => {}
        }

        intents
    }

    fn parse_visual_key(&mut self, key: &KeyEvent) -> Vec<Intent> {
        use kjxlkj_core_types::KeyCode;

        let mut intents = Vec::new();

        match &key.code {
            KeyCode::Escape => intents.push(Intent::SwitchMode(Mode::Normal)),
            KeyCode::Char('d') | KeyCode::Char('x') => {
                let linewise = self.mode == Mode::VisualLine;
                intents.push(Intent::Delete { linewise, count: 1, motion: None });
                intents.push(Intent::SwitchMode(Mode::Normal));
            }
            KeyCode::Char('y') => {
                let linewise = self.mode == Mode::VisualLine;
                intents.push(Intent::Yank { linewise, count: 1, motion: None });
                intents.push(Intent::SwitchMode(Mode::Normal));
            }
            KeyCode::Char('c') => {
                let linewise = self.mode == Mode::VisualLine;
                intents.push(Intent::Change { linewise, count: 1, motion: None });
            }
            KeyCode::Char('h') | KeyCode::Left => intents.push(Intent::Motion(MotionIntent::Left)),
            KeyCode::Char('l') | KeyCode::Right => {
                intents.push(Intent::Motion(MotionIntent::Right))
            }
            KeyCode::Char('j') | KeyCode::Down => intents.push(Intent::Motion(MotionIntent::Down)),
            KeyCode::Char('k') | KeyCode::Up => intents.push(Intent::Motion(MotionIntent::Up)),
            KeyCode::Char('o') => {
                // Swap selection ends
                if let Some(ref mut sel) = self.selection {
                    sel.swap();
                    self.cursor.position = sel.cursor;
                }
            }
            _ => {}
        }

        intents
    }

    fn parse_command_key(&mut self, key: &KeyEvent) -> Vec<Intent> {
        use kjxlkj_core_types::KeyCode;

        let mut intents = Vec::new();

        match &key.code {
            KeyCode::Escape => {
                self.command_line = None;
                intents.push(Intent::SwitchMode(Mode::Normal));
            }
            KeyCode::Enter => {
                if let Some(cmd) = self.command_line.take() {
                    intents.push(Intent::ExecuteCommand(cmd));
                }
                intents.push(Intent::SwitchMode(Mode::Normal));
            }
            KeyCode::Backspace => {
                if let Some(ref mut cmd) = self.command_line {
                    cmd.pop();
                }
            }
            KeyCode::Char(c) => {
                if self.command_line.is_none() {
                    self.command_line = Some(String::new());
                }
                if let Some(ref mut cmd) = self.command_line {
                    cmd.push(*c);
                }
            }
            _ => {}
        }

        intents
    }

    fn parse_replace_key(&mut self, key: &KeyEvent) -> Vec<Intent> {
        use kjxlkj_core_types::KeyCode;

        let mut intents = Vec::new();

        match &key.code {
            KeyCode::Escape => intents.push(Intent::SwitchMode(Mode::Normal)),
            KeyCode::Char(c) => {
                intents.push(Intent::ReplaceChar(*c));
                intents.push(Intent::Motion(MotionIntent::Right));
            }
            KeyCode::Backspace => intents.push(Intent::Motion(MotionIntent::Left)),
            _ => {}
        }

        intents
    }

    fn apply_intent(&mut self, intent: Intent) {
        match intent {
            Intent::InsertText(text) => self.insert_text(&text),
            Intent::Delete { linewise, count, motion } => self.delete_with_motion(linewise, count, motion),
            Intent::Yank { linewise, count, motion } => self.yank_with_motion(linewise, count, motion),
            Intent::Change { linewise, count, motion } => {
                self.delete_with_motion(linewise, count, motion);
                self.mode = Mode::Insert;
            }
            Intent::MoveCursor(pos) => {
                self.cursor.position = pos;
                self.cursor.clear_preferred_col();
            }
            Intent::Motion(motion) => self.apply_motion(motion),
            Intent::SwitchMode(new_mode) => self.switch_mode(new_mode),
            Intent::ExecuteCommand(cmd) => self.execute_command(&cmd),
            Intent::OpenLine { below } => self.open_line(below),
            Intent::Undo => self.undo(),
            Intent::Redo => self.redo(),
            Intent::JoinLines { add_space } => self.join_lines(add_space),
            Intent::Indent => self.indent(true),
            Intent::Outdent => self.indent(false),
            Intent::ToggleCase => self.toggle_case(),
            Intent::Uppercase => {} // TODO
            Intent::Lowercase => {} // TODO
            Intent::Increment(n) => {} // TODO
            Intent::Repeat => {}       // TODO
            Intent::MacroToggle(c) => self.toggle_macro(c),
            Intent::MacroPlay(c) => {} // TODO
            Intent::SetMark(c) => {
                self.marks.insert(c, self.cursor.position);
            }
            Intent::JumpToMark { mark, first_non_blank } => {
                if let Some(pos) = self.marks.get(&mark).copied() {
                    self.cursor.position = pos;
                }
            }
            Intent::SearchForward(_) => {}  // TODO
            Intent::SearchBackward(_) => {} // TODO
            Intent::NextMatch => {}         // TODO
            Intent::PrevMatch => {}         // TODO
            Intent::Scroll(scroll) => self.scroll(scroll),
            Intent::Paste { before, cursor_at_end } => self.paste(before, cursor_at_end),
            Intent::SelectRegister(c) => {
                if let Some(name) = kjxlkj_core_types::RegisterName::from_char(c) {
                    self.registers.select(name);
                }
            }
            Intent::InsertFromRegister(c) => {
                if let Some(name) = kjxlkj_core_types::RegisterName::from_char(c) {
                    if let Some(content) = self.registers.get(name) {
                        let text = content.content.clone();
                        self.insert_text(&text);
                    }
                }
            }
            Intent::ReplaceChar(c) => self.replace_char(c),
            Intent::Substitute => {
                self.delete(false, 1);
                self.mode = Mode::Insert;
            }
            Intent::Nop => {}
        }
    }

    fn insert_text(&mut self, text: &str) {
        if text == "\x08" {
            // Backspace
            if self.cursor.col() > 0 {
                let line_start = self.buffer.line_to_char(self.cursor.line());
                let idx = line_start + self.cursor.col();
                self.buffer.remove(idx - 1, idx);
                self.cursor.position.col -= 1;
            } else if self.cursor.line() > 0 {
                // Join with previous line
                let prev_line = self.cursor.line() - 1;
                let prev_len = self.buffer.line_grapheme_len(prev_line);
                let line_start = self.buffer.line_to_char(self.cursor.line());
                self.buffer.remove(line_start - 1, line_start);
                self.cursor.position.line = prev_line;
                self.cursor.position.col = prev_len;
            }
        } else {
            let line_start = self.buffer.line_to_char(self.cursor.line());
            let idx = line_start + self.cursor.col();
            self.buffer.insert(idx, text);

            // Update cursor
            if text.contains('\n') {
                let lines: Vec<&str> = text.split('\n').collect();
                self.cursor.position.line += lines.len() - 1;
                self.cursor.position.col = lines.last().map(|s| s.len()).unwrap_or(0);
            } else {
                self.cursor.position.col += text.chars().count();
            }
        }
    }

    fn delete(&mut self, linewise: bool, count: usize) {
        let cursor_before = self.cursor.position;
        
        let (start, end) = if let Some(sel) = self.selection.take() {
            (sel.start(), sel.end())
        } else if linewise && count > 1 {
            // Delete multiple lines starting from cursor
            let pos = self.cursor.position;
            let end_line = (pos.line + count - 1).min(self.buffer.line_count().saturating_sub(1));
            (pos, Position::new(end_line, 0))
        } else if count > 1 {
            // Delete multiple characters at cursor
            let pos = self.cursor.position;
            // End position is count-1 chars to the right (start is included)
            (pos, Position::new(pos.line, pos.col + count - 1))
        } else {
            // Delete single character at cursor
            let pos = self.cursor.position;
            (pos, pos)
        };

        let deleted_text = if linewise {
            let mut text = String::new();
            let lines_to_delete = (end.line - start.line + 1).min(self.buffer.line_count());
            for _ in 0..lines_to_delete {
                if let Some(slice) = self.buffer.line(start.line) {
                    text.push_str(slice.as_str().unwrap_or(""));
                    // Add newline between lines for proper linewise paste
                    if !text.ends_with('\n') {
                        text.push('\n');
                    }
                }
                self.buffer.remove_line(start.line);
            }
            self.cursor.position = Position::new(
                start.line.min(self.buffer.line_count().saturating_sub(1)),
                0,
            );
            text
        } else {
            let start_idx = self.buffer.line_to_char(start.line) + start.col;
            let end_idx = self.buffer.line_to_char(end.line) + end.col + 1;
            let end_idx = end_idx.min(self.buffer.char_count());
            let text = self.buffer.rope().slice(start_idx..end_idx).to_string();
            self.buffer.remove(start_idx, end_idx);
            self.cursor.position = start;
            text
        };

        // Record undo
        if !deleted_text.is_empty() {
            let mut tx = Transaction::new();
            tx.set_cursor_before(cursor_before);
            tx.push(kjxlkj_core_undo::Edit::delete(start, deleted_text.clone()));
            tx.set_cursor_after(self.cursor.position);
            self.undo_history.push(tx);
        }

        // Store in register
        self.registers
            .set_selected(Register::new(deleted_text, linewise));
    }

    fn yank(&mut self, linewise: bool, count: usize) {
        let (start, end) = if let Some(sel) = &self.selection {
            (sel.start(), sel.end())
        } else if linewise && count > 1 {
            let pos = self.cursor.position;
            let end_line = (pos.line + count - 1).min(self.buffer.line_count().saturating_sub(1));
            (pos, Position::new(end_line, 0))
        } else {
            let pos = self.cursor.position;
            (pos, pos)
        };

        let text = if linewise {
            let mut yanked = String::new();
            for line in start.line..=end.line {
                if let Some(slice) = self.buffer.line(line) {
                    yanked.push_str(slice.as_str().unwrap_or(""));
                }
            }
            yanked
        } else {
            let start_idx = self.buffer.line_to_char(start.line) + start.col;
            let end_idx = self.buffer.line_to_char(end.line) + end.col + 1;
            let end_idx = end_idx.min(self.buffer.char_count());
            self.buffer.rope().slice(start_idx..end_idx).to_string()
        };

        self.registers.set_selected(Register::new(text, linewise));
        self.selection = None;
        self.cursor.position = start;
    }

    fn delete_with_motion(&mut self, linewise: bool, count: usize, motion: Option<MotionIntent>) {
        if let Some(m) = motion {
            // Calculate motion target first
            let start_pos = self.cursor.position;
            let end_pos = apply_motion(&Motion::new(m, count), &self.cursor, &self.buffer, self.viewport.height);
            
            // Determine actual range (start might be after end for backwards motions)
            let (start, end) = if start_pos < end_pos {
                (start_pos, end_pos)
            } else {
                (end_pos, start_pos)
            };
            
            // Delete the range (with special handling for exclusive motions)
            let start_idx = self.buffer.line_to_char(start.line) + start.col;
            let end_idx = self.buffer.line_to_char(end.line) + end.col;
            let end_idx = end_idx.min(self.buffer.char_count());
            
            if start_idx < end_idx {
                let text = self.buffer.rope().slice(start_idx..end_idx).to_string();
                self.buffer.remove(start_idx, end_idx);
                self.cursor.position = start;
                
                // Record undo
                let mut tx = Transaction::new();
                tx.set_cursor_before(start_pos);
                tx.push(kjxlkj_core_undo::Edit::delete(start, text.clone()));
                tx.set_cursor_after(self.cursor.position);
                self.undo_history.push(tx);
                
                self.registers.set_selected(Register::new(text, false));
            }
        } else {
            // No motion, use the original delete
            self.delete(linewise, count);
        }
    }

    fn yank_with_motion(&mut self, linewise: bool, count: usize, motion: Option<MotionIntent>) {
        if let Some(m) = motion {
            // Calculate motion target first
            let start_pos = self.cursor.position;
            let end_pos = apply_motion(&Motion::new(m, count), &self.cursor, &self.buffer, self.viewport.height);
            
            // Determine actual range
            let (start, end) = if start_pos < end_pos {
                (start_pos, end_pos)
            } else {
                (end_pos, start_pos)
            };
            
            let start_idx = self.buffer.line_to_char(start.line) + start.col;
            let end_idx = self.buffer.line_to_char(end.line) + end.col;
            let end_idx = end_idx.min(self.buffer.char_count());
            
            if start_idx <= end_idx {
                let text = self.buffer.rope().slice(start_idx..end_idx).to_string();
                self.registers.set_selected(Register::new(text, false));
            }
            
            self.cursor.position = start;
        } else {
            // No motion, use the original yank
            self.yank(linewise, count);
        }
    }

    fn apply_motion(&mut self, motion: MotionIntent) {
        let new_pos =
            apply_motion(&Motion::new(motion.clone(), 1), &self.cursor, &self.buffer, self.viewport.height);
        self.cursor.position = new_pos;

        // Update preferred column for vertical motions
        match motion {
            MotionIntent::Up | MotionIntent::Down => {
                if self.cursor.preferred_col.is_none() {
                    self.cursor.preferred_col = Some(self.cursor.col());
                }
            }
            _ => {
                self.cursor.clear_preferred_col();
            }
        }

        // Update selection if in visual mode
        if self.mode.is_visual() {
            if let Some(ref mut sel) = self.selection {
                sel.cursor = self.cursor.position;
            }
        }
    }

    fn switch_mode(&mut self, new_mode: Mode) {
        let old_mode = self.mode;
        self.mode = new_mode;

        match new_mode {
            Mode::Visual => {
                self.selection = Some(Selection::char_wise(
                    self.cursor.position,
                    self.cursor.position,
                ));
            }
            Mode::VisualLine => {
                self.selection = Some(Selection::line_wise(
                    self.cursor.position,
                    self.cursor.position,
                ));
            }
            Mode::VisualBlock => {
                self.selection = Some(Selection::block_wise(
                    self.cursor.position,
                    self.cursor.position,
                ));
            }
            Mode::Normal => {
                self.selection = None;
                // Clamp cursor to line length in normal mode
                let line_len = self.buffer.line_grapheme_len(self.cursor.line());
                if line_len > 0 && self.cursor.col() >= line_len {
                    self.cursor.position.col = line_len - 1;
                }
            }
            Mode::Command => {
                self.command_line = Some(String::new());
            }
            _ => {}
        }
    }

    fn execute_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();

        // Handle commands with arguments
        if cmd.starts_with("e ") || cmd.starts_with("edit ") {
            let file = cmd.splitn(2, ' ').nth(1).unwrap_or("").trim();
            self.status_message = Some((format!("Would open: {}", file), false));
            return;
        }

        if cmd.starts_with("w ") || cmd.starts_with("write ") {
            let file = cmd.splitn(2, ' ').nth(1).unwrap_or("").trim();
            self.status_message = Some((format!("Would write to: {}", file), false));
            return;
        }

        if cmd.starts_with("set ") {
            let args = cmd.splitn(2, ' ').nth(1).unwrap_or("").trim();
            self.handle_set_command(args);
            return;
        }

        if cmd.starts_with("! ") {
            let shell_cmd = cmd.splitn(2, ' ').nth(1).unwrap_or("").trim();
            self.status_message = Some((format!("Would run: {}", shell_cmd), false));
            return;
        }

        // Simple commands without arguments
        match cmd {
            "q" => {
                if self.buffer.is_modified() {
                    self.status_message = Some(("No write since last change (add ! to override)".to_string(), true));
                } else {
                    self.should_quit = true;
                }
            }
            "q!" => {
                self.should_quit = true;
            }
            "qa" => {
                if self.buffer.is_modified() {
                    self.status_message = Some(("No write since last change (add ! to override)".to_string(), true));
                } else {
                    self.should_quit = true;
                }
            }
            "qa!" => {
                self.should_quit = true;
            }
            "w" | "write" => {
                self.status_message = Some(("Written".to_string(), false));
            }
            "wa" | "wall" => {
                self.status_message = Some(("All buffers written".to_string(), false));
            }
            "wq" | "x" | "exit" => {
                self.should_quit = true;
            }
            "e" | "edit" => {
                self.status_message = Some(("Usage: :e {filename}".to_string(), true));
            }
            "e!" | "edit!" => {
                // Reload without saving
                self.buffer.mark_saved();
                self.status_message = Some(("Reloaded".to_string(), false));
            }
            "ls" | "buffers" => {
                self.status_message = Some((format!("{} buffer(s)", 1), false));
            }
            "bn" | "bnext" => {
                self.status_message = Some(("No next buffer".to_string(), false));
            }
            "bp" | "bprev" | "bprevious" => {
                self.status_message = Some(("No previous buffer".to_string(), false));
            }
            "bd" | "bdelete" => {
                if self.buffer.is_modified() {
                    self.status_message = Some(("No write since last change (add ! to override)".to_string(), true));
                } else {
                    self.status_message = Some(("Buffer deleted".to_string(), false));
                }
            }
            "bd!" | "bdelete!" => {
                self.status_message = Some(("Buffer deleted".to_string(), false));
            }
            "new" => {
                self.status_message = Some(("Would create horizontal split".to_string(), false));
            }
            "vnew" => {
                self.status_message = Some(("Would create vertical split".to_string(), false));
            }
            "sp" | "split" => {
                self.status_message = Some(("Would create horizontal split".to_string(), false));
            }
            "vsp" | "vsplit" => {
                self.status_message = Some(("Would create vertical split".to_string(), false));
            }
            "only" => {
                self.status_message = Some(("Only one window".to_string(), false));
            }
            "close" => {
                self.status_message = Some(("Cannot close last window".to_string(), true));
            }
            "" => {
                // Empty command - do nothing
            }
            _ => {
                // Check for number (goto line)
                if let Ok(line_num) = cmd.parse::<usize>() {
                    let max_line = self.buffer.line_count().saturating_sub(1);
                    let target = line_num.saturating_sub(1).min(max_line);
                    self.cursor.position.line = target;
                    self.cursor.position.col = 0;
                    self.ensure_cursor_visible();
                } else {
                    self.status_message = Some((format!("Unknown command: {}", cmd), true));
                }
            }
        }
    }

    fn handle_set_command(&mut self, args: &str) {
        // Simple set command handling
        let msg = match args {
            "number" | "nu" => "Line numbers enabled".to_string(),
            "nonumber" | "nonu" => "Line numbers disabled".to_string(),
            "wrap" => "Line wrap enabled".to_string(),
            "nowrap" => "Line wrap disabled".to_string(),
            "list" => "Show invisible chars enabled".to_string(),
            "nolist" => "Show invisible chars disabled".to_string(),
            "hlsearch" | "hls" => "Search highlighting enabled".to_string(),
            "nohlsearch" | "nohls" => "Search highlighting disabled".to_string(),
            "ignorecase" | "ic" => "Ignore case enabled".to_string(),
            "noignorecase" | "noic" => "Ignore case disabled".to_string(),
            "all" => "Would show all options".to_string(),
            _ => format!("Unknown option: {}", args),
        };
        self.status_message = Some((msg, false));
    }

    fn open_line(&mut self, below: bool) {
        let line = if below {
            self.cursor.line()
        } else {
            self.cursor.line().saturating_sub(1)
        };

        let insert_line = if below { line + 1 } else { line };
        let line_start = self.buffer.line_to_char(insert_line.min(self.buffer.line_count()));
        self.buffer.insert(line_start, "\n");

        self.cursor.position = Position::new(insert_line, 0);
        self.mode = Mode::Insert;
    }

    fn undo(&mut self) {
        if let Some(tx) = self.undo_history.undo() {
            // Apply inverse edits
            for edit in tx.edits() {
                match edit {
                    kjxlkj_core_undo::Edit::Insert { position, text } => {
                        let idx = self.buffer.line_to_char(position.line) + position.col;
                        self.buffer.insert(idx, text);
                    }
                    kjxlkj_core_undo::Edit::Delete { position, text } => {
                        let idx = self.buffer.line_to_char(position.line) + position.col;
                        self.buffer.remove(idx, idx + text.len());
                    }
                }
            }
            if let Some(pos) = tx.cursor_after() {
                self.cursor.position = pos;
            }
            self.status_message = Some(("Undo".to_string(), false));
        }
    }

    fn redo(&mut self) {
        if let Some(tx) = self.undo_history.redo() {
            for edit in tx.edits() {
                match edit {
                    kjxlkj_core_undo::Edit::Insert { position, text } => {
                        let idx = self.buffer.line_to_char(position.line) + position.col;
                        self.buffer.insert(idx, text);
                    }
                    kjxlkj_core_undo::Edit::Delete { position, text } => {
                        let idx = self.buffer.line_to_char(position.line) + position.col;
                        self.buffer.remove(idx, idx + text.len());
                    }
                }
            }
            if let Some(pos) = tx.cursor_after() {
                self.cursor.position = pos;
            }
            self.status_message = Some(("Redo".to_string(), false));
        }
    }

    fn join_lines(&mut self, add_space: bool) {
        let line = self.cursor.line();
        if line + 1 < self.buffer.line_count() {
            let line_len = self.buffer.line_grapheme_len(line);
            let next_line_start = self.buffer.line_to_char(line + 1);

            // Remove newline at end of current line
            let current_line_end = next_line_start - 1;
            self.buffer.remove(current_line_end, next_line_start);

            // Optionally add space
            if add_space {
                self.buffer.insert(current_line_end, " ");
                self.cursor.position.col = line_len;
            } else {
                self.cursor.position.col = line_len.saturating_sub(1);
            }
        }
    }

    fn indent(&mut self, indent: bool) {
        let indent_str = "    ";
        let line_start = self.buffer.line_to_char(self.cursor.line());

        if indent {
            self.buffer.insert(line_start, indent_str);
        } else {
            if let Some(slice) = self.buffer.line(self.cursor.line()) {
                let s = slice.as_str().unwrap_or("");
                let spaces: usize = s.chars().take(4).take_while(|c| *c == ' ').count();
                if spaces > 0 {
                    self.buffer.remove(line_start, line_start + spaces);
                }
            }
        }
    }

    fn toggle_case(&mut self) {
        let line_start = self.buffer.line_to_char(self.cursor.line());
        let idx = line_start + self.cursor.col();

        if idx < self.buffer.char_count() {
            let c = self.buffer.rope().char(idx);
            let toggled: String = if c.is_uppercase() {
                c.to_lowercase().collect()
            } else {
                c.to_uppercase().collect()
            };
            self.buffer.remove(idx, idx + 1);
            self.buffer.insert(idx, &toggled);
            self.cursor.position.col += 1;
        }
    }

    fn scroll(&mut self, scroll: ScrollIntent) {
        let half_page = self.viewport.height / 2;
        let line_count = self.buffer.line_count();

        match scroll {
            ScrollIntent::HalfPageDown => {
                self.viewport.scroll_down(half_page, line_count);
                self.cursor.position.line =
                    (self.cursor.line() + half_page).min(line_count.saturating_sub(1));
            }
            ScrollIntent::HalfPageUp => {
                self.viewport.scroll_up(half_page);
                self.cursor.position.line = self.cursor.line().saturating_sub(half_page);
            }
            ScrollIntent::PageDown => {
                self.viewport.scroll_down(self.viewport.height, line_count);
                self.cursor.position.line = (self.cursor.line() + self.viewport.height)
                    .min(line_count.saturating_sub(1));
            }
            ScrollIntent::PageUp => {
                self.viewport.scroll_up(self.viewport.height);
                self.cursor.position.line =
                    self.cursor.line().saturating_sub(self.viewport.height);
            }
            ScrollIntent::LineDown => {
                self.viewport.scroll_down(1, line_count);
            }
            ScrollIntent::LineUp => {
                self.viewport.scroll_up(1);
            }
            ScrollIntent::CenterCursor => {
                self.viewport.center_on_line(self.cursor.line(), line_count);
            }
            ScrollIntent::CursorToTop => {
                self.viewport.cursor_to_top(self.cursor.line());
            }
            ScrollIntent::CursorToBottom => {
                self.viewport.cursor_to_bottom(self.cursor.line());
            }
        }
    }

    fn paste(&mut self, before: bool, cursor_at_end: bool) {
        if let Some(reg) = self.registers.get_selected().cloned() {
            if reg.linewise {
                let line = if before {
                    self.cursor.line()
                } else {
                    self.cursor.line() + 1
                };
                let idx = self.buffer.line_to_char(line.min(self.buffer.line_count()));
                let content = if reg.content.ends_with('\n') {
                    reg.content.clone()
                } else {
                    format!("{}\n", reg.content)
                };
                self.buffer.insert(idx, &content);
                self.cursor.position = Position::new(line, 0);
            } else {
                let line_start = self.buffer.line_to_char(self.cursor.line());
                let idx = if before {
                    line_start + self.cursor.col()
                } else {
                    line_start + self.cursor.col() + 1
                };
                self.buffer.insert(idx.min(self.buffer.char_count()), &reg.content);
                if !before {
                    self.cursor.position.col += 1;
                }
                if cursor_at_end {
                    self.cursor.position.col += reg.content.len();
                }
            }
        }
    }

    fn replace_char(&mut self, c: char) {
        let line_start = self.buffer.line_to_char(self.cursor.line());
        let idx = line_start + self.cursor.col();
        if idx < self.buffer.char_count() {
            self.buffer.remove(idx, idx + 1);
            self.buffer.insert(idx, &c.to_string());
        }
    }

    fn toggle_macro(&mut self, c: char) {
        if let Some(recording) = self.recording_macro.take() {
            // Stop recording
            let recorded = std::mem::take(&mut self.recording_change);
            // Remove the q command itself
            let recorded: Vec<_> = recorded
                .into_iter()
                .take_while(|k| {
                    !matches!(k.code, kjxlkj_core_types::KeyCode::Char('q'))
                })
                .collect();
            self.macros.insert(recording, recorded);
            self.status_message = Some((format!("Recorded @{}", recording), false));
        } else {
            // Start recording
            self.recording_macro = Some(c);
            self.recording_change.clear();
            self.status_message = Some((format!("Recording @{}", c), false));
        }
    }

    fn clamp_cursor(&mut self) {
        let line_count = self.buffer.line_count();
        if self.cursor.line() >= line_count {
            self.cursor.position.line = line_count.saturating_sub(1);
        }

        let line_len = self.buffer.line_grapheme_len(self.cursor.line());
        let max_col = if self.mode == Mode::Insert {
            line_len
        } else {
            line_len.saturating_sub(1).max(0)
        };

        if self.cursor.col() > max_col {
            self.cursor.position.col = max_col;
        }
    }

    /// Ensure cursor is visible within the viewport.
    fn ensure_cursor_visible(&mut self) {
        let cursor_line = self.cursor.line();
        
        // If cursor is above viewport, scroll up
        if cursor_line < self.viewport.top_line {
            self.viewport.top_line = cursor_line;
        }
        
        // If cursor is below viewport, scroll down
        let bottom = self.viewport.top_line + self.viewport.height.saturating_sub(1);
        if cursor_line > bottom {
            self.viewport.top_line = cursor_line.saturating_sub(self.viewport.height.saturating_sub(1));
        }
    }

    /// Create a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let mut lines = Vec::new();
        for line_idx in self.viewport.top_line..self.viewport.bottom_line() {
            if let Some(line) = self.buffer.line(line_idx) {
                let s = line.as_str().unwrap_or("");
                let s = s.trim_end_matches('\n').trim_end_matches('\r');
                lines.push(s.to_string());
            } else {
                lines.push(String::new());
            }
        }

        let buffer_snapshot = BufferSnapshot::new(
            self.buffer.id(),
            self.buffer.name().clone(),
            self.buffer.version(),
            self.buffer.line_count(),
            lines,
            self.viewport,
            self.buffer.is_modified(),
        );

        let status = StatusLine::new(
            self.mode,
            self.buffer.name().as_str().to_string(),
            self.buffer.is_modified(),
            &self.cursor,
            self.buffer.line_count(),
        );

        let status = if let Some((msg, is_error)) = &self.status_message {
            status.with_message(msg.clone(), *is_error)
        } else {
            status
        };

        EditorSnapshot::new(
            buffer_snapshot,
            self.cursor.clone(),
            self.mode,
            self.selection.clone(),
            status,
            self.command_line.clone(),
            self.registers.search_pattern().map(String::from),
            self.width,
            self.height,
        )
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
    use kjxlkj_core_types::KeyCode;

    #[test]
    fn test_editor_state_new() {
        let state = EditorState::new();
        assert_eq!(state.mode(), Mode::Normal);
        assert_eq!(state.cursor().line(), 0);
        assert_eq!(state.cursor().col(), 0);
    }

    #[test]
    fn test_editor_state_insert() {
        let mut state = EditorState::new();
        state.handle_key(KeyEvent::char('i'));
        assert_eq!(state.mode(), Mode::Insert);

        state.handle_key(KeyEvent::char('h'));
        state.handle_key(KeyEvent::char('i'));
        assert_eq!(state.content(), "hi");
    }

    #[test]
    fn test_editor_state_motion() {
        let mut state = EditorState::new();
        state.load_content("hello\nworld");

        state.handle_key(KeyEvent::char('j'));
        assert_eq!(state.cursor().line(), 1);

        state.handle_key(KeyEvent::char('k'));
        assert_eq!(state.cursor().line(), 0);
    }

    #[test]
    fn test_editor_state_quit() {
        let mut state = EditorState::new();
        state.handle_key(KeyEvent::char(':'));
        state.handle_key(KeyEvent::char('q'));
        state.handle_key(KeyEvent::plain(KeyCode::Enter));
        assert!(state.should_quit());
    }

    #[test]
    fn test_editor_state_load_content() {
        let mut state = EditorState::new();
        state.load_content("hello world");
        assert_eq!(state.content(), "hello world");
    }

    #[test]
    fn test_editor_state_resize() {
        let mut state = EditorState::new();
        state.resize(100, 50);
        // Just ensure no panic
        assert!(!state.should_quit());
    }

    #[test]
    fn test_editor_state_visual_mode() {
        let mut state = EditorState::new();
        state.load_content("hello world");
        state.handle_key(KeyEvent::char('v'));
        assert_eq!(state.mode(), Mode::Visual);
    }

    #[test]
    fn test_editor_state_command_mode() {
        let mut state = EditorState::new();
        state.handle_key(KeyEvent::char(':'));
        assert_eq!(state.mode(), Mode::Command);
    }

    #[test]
    fn test_editor_state_escape_from_insert() {
        let mut state = EditorState::new();
        state.handle_key(KeyEvent::char('i'));
        assert_eq!(state.mode(), Mode::Insert);
        state.handle_key(KeyEvent::plain(KeyCode::Escape));
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn test_editor_state_cursor_movement_h_l() {
        let mut state = EditorState::new();
        state.load_content("hello");
        state.handle_key(KeyEvent::char('l'));
        assert_eq!(state.cursor().col(), 1);
        state.handle_key(KeyEvent::char('h'));
        assert_eq!(state.cursor().col(), 0);
    }

    #[test]
    fn test_editor_state_word_motion() {
        let mut state = EditorState::new();
        state.load_content("hello world");
        state.handle_key(KeyEvent::char('w'));
        assert_eq!(state.cursor().col(), 6);
    }

    #[test]
    fn test_editor_state_line_start_end() {
        let mut state = EditorState::new();
        state.load_content("hello world");
        state.handle_key(KeyEvent::char('$'));
        assert!(state.cursor().col() > 0);
        state.handle_key(KeyEvent::char('0'));
        assert_eq!(state.cursor().col(), 0);
    }

    #[test]
    fn test_editor_state_delete_char() {
        let mut state = EditorState::new();
        state.load_content("hello");
        state.handle_key(KeyEvent::char('x'));
        assert_eq!(state.content(), "ello");
    }

    #[test]
    fn test_editor_state_undo() {
        let mut state = EditorState::new();
        state.load_content("hello");
        state.handle_key(KeyEvent::char('x'));
        state.handle_key(KeyEvent::char('u'));
        assert_eq!(state.content(), "hello");
    }

    #[test]
    fn test_editor_state_buffer() {
        let state = EditorState::new();
        let _buffer = state.buffer();
        // Just ensure we can access buffer
    }

    #[test]
    fn test_editor_state_snapshot() {
        let state = EditorState::new();
        let _snapshot = state.snapshot();
        // Just ensure snapshot works
    }

    #[test]
    fn test_editor_state_replace_mode() {
        let mut state = EditorState::new();
        state.load_content("hello");
        state.handle_key(KeyEvent::char('R'));
        assert_eq!(state.mode(), Mode::Replace);
    }

    #[test]
    fn test_editor_state_append() {
        let mut state = EditorState::new();
        state.load_content("hello");
        state.handle_key(KeyEvent::char('a'));
        assert_eq!(state.mode(), Mode::Insert);
        state.handle_key(KeyEvent::char('!'));
        assert!(state.content().contains('!'));
    }
}
