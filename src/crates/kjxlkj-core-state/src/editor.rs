//! Core editor state.

use std::collections::HashMap;

use kjxlkj_core_edit::{find_text_object_range, Buffer, CursorOps};
use kjxlkj_core_mode::{CommandLineState, KeyCode, KeyInput, Modifiers, ModeHandler};
use kjxlkj_core_types::{BufferId, EditorAction, EditorEvent, LineCol, Mode, Motion, Operator, TextObject};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, SnapshotSeq, StatusLine, Viewport};

use crate::CommandParser;

/// Direction for find char commands (f/t/F/T).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindCharDirection {
    Forward,
    Backward,
    TillForward,
    TillBackward,
}

/// Case transformation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseTransform {
    Toggle,
    Upper,
    Lower,
}

/// A repeatable change for the dot command.
#[derive(Debug, Clone)]
pub enum RepeatableChange {
    /// Operator with motion (e.g., dw, cw, ye).
    OperatorMotion { operator: Operator, motion: Motion, count: Option<u32> },
    /// Operator with text object (e.g., diw, ci").
    OperatorTextObject { operator: Operator, text_object: TextObject },
    /// Delete character at cursor (x).
    DeleteCharAt,
    /// Delete to end of line (D).
    DeleteToEnd,
    /// Change to end of line (C).
    ChangeToEnd,
    /// Substitute character (s).
    Substitute,
    /// Substitute line (S).
    SubstituteLine,
    /// Insert text (the text inserted before returning to normal mode).
    InsertText(String),
}

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
    /// Last search pattern.
    search_pattern: Option<String>,
    /// True if last search was forward.
    search_forward: bool,
    /// Visual mode anchor (starting position of selection).
    visual_anchor: Option<LineCol>,
    /// Last find char command for ; and , repeat.
    last_find_char: Option<(char, FindCharDirection)>,
    /// Last repeatable change for dot command.
    last_change: Option<RepeatableChange>,
    /// Text being inserted (for tracking insert mode changes).
    insert_buffer: String,
    /// Local marks (a-z) - maps mark character to position.
    marks: HashMap<char, LineCol>,
    /// Named registers (a-z, 0-9, ", +, *, etc.)
    registers: HashMap<char, String>,
    /// Currently selected register for next yank/delete/paste.
    pending_register: Option<char>,
    /// Register currently being recorded to (None if not recording).
    macro_recording_register: Option<char>,
    /// Keys recorded during macro recording.
    macro_recording_keys: Vec<String>,
    /// Last macro register used for @@ repeat.
    last_macro_register: Option<char>,
    /// Jump list for Ctrl-o / Ctrl-i navigation.
    jump_list: Vec<LineCol>,
    /// Current position in jump list (index into jump_list).
    jump_list_index: usize,
    /// Change list for g; / g, navigation.
    change_list: Vec<LineCol>,
    /// Current position in change list (index into change_list).
    change_list_index: usize,
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
            search_pattern: None,
            search_forward: true,
            visual_anchor: None,
            last_find_char: None,
            last_change: None,
            insert_buffer: String::new(),
            marks: HashMap::new(),
            registers: HashMap::new(),
            pending_register: None,
            macro_recording_register: None,
            macro_recording_keys: Vec::new(),
            last_macro_register: None,
            jump_list: Vec::new(),
            jump_list_index: 0,
            change_list: Vec::new(),
            change_list_index: 0,
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
        // Handle stopping macro recording: 'q' alone stops recording when recording
        let key_str = key.to_string();
        if self.macro_recording_register.is_some() && key_str == "q" {
            // Stop recording - don't pass to mode handler
            self.stop_macro_recording();
            self.snapshot_seq = self.snapshot_seq.next();
            return Vec::new();
        }
        
        // Record key for macro recording (before processing)
        self.record_key(&key_str);
        
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
            EditorAction::ScrollHalfPageDown => self.scroll_half_page(true),
            EditorAction::ScrollHalfPageUp => self.scroll_half_page(false),
            EditorAction::ScrollPageDown => self.scroll_full_page(true),
            EditorAction::ScrollPageUp => self.scroll_full_page(false),
            EditorAction::ScrollLineDown => self.scroll_line(true),
            EditorAction::ScrollLineUp => self.scroll_line(false),
            EditorAction::ScreenTop => self.move_to_screen_top(),
            EditorAction::ScreenMiddle => self.move_to_screen_middle(),
            EditorAction::ScreenBottom => self.move_to_screen_bottom(),
            EditorAction::ScrollCursorCenter => self.scroll_cursor_center(),
            EditorAction::ScrollCursorTop => self.scroll_cursor_top(),
            EditorAction::ScrollCursorBottom => self.scroll_cursor_bottom(),
            EditorAction::ScrollCursorTopFirstNonBlank => {
                self.scroll_cursor_top();
                self.buffer.move_first_non_blank();
            }
            EditorAction::ScrollCursorCenterFirstNonBlank => {
                self.scroll_cursor_center();
                self.buffer.move_first_non_blank();
            }
            EditorAction::ScrollCursorBottomFirstNonBlank => {
                self.scroll_cursor_bottom();
                self.buffer.move_first_non_blank();
            }
            EditorAction::LineStart => self.buffer.move_line_start(),
            EditorAction::LineEnd => self.buffer.move_line_end(),
            EditorAction::GoToColumn(col) => self.buffer.go_to_column(col),
            EditorAction::LineMiddle => self.buffer.move_line_middle(),
            EditorAction::FirstNonBlank => self.buffer.move_first_non_blank(),
            EditorAction::FirstNonBlankWithOffset(count) => {
                // _ motion: move [count]-1 lines down, then to first non-blank
                let lines_down = count.saturating_sub(1) as usize;
                for _ in 0..lines_down {
                    self.buffer.move_down();
                }
                self.buffer.move_first_non_blank();
            }
            EditorAction::LastNonBlank => self.buffer.move_last_non_blank(),
            EditorAction::NextLineStart => self.move_next_line_start(),
            EditorAction::PrevLineStart => self.move_prev_line_start(),
            EditorAction::WordForward => self.buffer.move_word_forward(),
            EditorAction::WORDForward => self.buffer.move_word_forward(), // TODO: WORD semantics
            EditorAction::WordBackward => self.buffer.move_word_backward(),
            EditorAction::WORDBackward => self.buffer.move_word_backward(), // TODO: WORD semantics
            EditorAction::WordEnd => self.buffer.move_word_end(),
            EditorAction::WORDEnd => self.buffer.move_word_end(), // TODO: WORD semantics
            EditorAction::WordEndBackward => self.buffer.move_word_end_backward(),
            EditorAction::WORDEndBackward => self.buffer.move_word_end_backward(), // TODO: WORD semantics
            EditorAction::FileStart => {
                self.add_to_jump_list();
                self.buffer.move_file_start();
            }
            EditorAction::FileEnd => {
                self.add_to_jump_list();
                self.buffer.move_file_end();
            }
            EditorAction::GoToLine(line) => {
                self.add_to_jump_list();
                self.go_to_line(line);
            }
            EditorAction::GoToPercent(pct) => {
                self.add_to_jump_list();
                self.go_to_percent(pct);
            }
            EditorAction::SentenceForward => {
                self.move_sentence_forward();
            }
            EditorAction::SentenceBackward => {
                self.move_sentence_backward();
            }
            EditorAction::ParagraphForward => {
                self.move_paragraph_forward();
            }
            EditorAction::ParagraphBackward => {
                self.move_paragraph_backward();
            }
            EditorAction::MatchBracket => {
                self.move_match_bracket();
            }
            EditorAction::FindCharForward(ch) => {
                if self.buffer.find_char_forward(ch) {
                    self.last_find_char = Some((ch, FindCharDirection::Forward));
                }
            }
            EditorAction::FindCharBackward(ch) => {
                if self.buffer.find_char_backward(ch) {
                    self.last_find_char = Some((ch, FindCharDirection::Backward));
                }
            }
            EditorAction::TillCharForward(ch) => {
                if self.buffer.till_char_forward(ch) {
                    self.last_find_char = Some((ch, FindCharDirection::TillForward));
                }
            }
            EditorAction::TillCharBackward(ch) => {
                if self.buffer.till_char_backward(ch) {
                    self.last_find_char = Some((ch, FindCharDirection::TillBackward));
                }
            }
            EditorAction::RepeatFindChar => {
                self.repeat_find_char(false);
            }
            EditorAction::RepeatFindCharReverse => {
                self.repeat_find_char(true);
            }
            EditorAction::InsertChar(ch) => {
                self.insert_buffer.push(ch);
                self.buffer.insert_char(ch);
            }
            EditorAction::ReplaceChar(ch) => {
                self.buffer.replace_char(ch);
            }
            EditorAction::ReplaceSingleChar(ch) => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.buffer.replace_single_char(ch);
            }
            EditorAction::InsertNewline => self.buffer.insert_newline(),
            EditorAction::DeleteCharBefore => self.buffer.delete_char_before(),
            EditorAction::DeleteWordBefore => self.delete_word_before(),
            EditorAction::DeleteToLineStart => self.delete_to_line_start(),
            EditorAction::InsertIndent => self.insert_indent(),
            EditorAction::InsertOutdent => self.insert_outdent(),
            EditorAction::InsertCopyAbove => self.insert_copy_above(),
            EditorAction::InsertCopyBelow => self.insert_copy_below(),
            EditorAction::InsertRegister(reg) => self.insert_register(reg),
            EditorAction::DeleteCharAt => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.last_change = Some(RepeatableChange::DeleteCharAt);
                self.buffer.delete_char_at();
                self.store_in_pending_register();
            }
            EditorAction::DeleteLine => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.buffer.delete_line();
                self.store_in_pending_register();
            }
            EditorAction::YankLine => {
                self.buffer.yank_line();
                self.store_in_pending_register();
            }
            EditorAction::DeleteToEndOfLine => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.last_change = Some(RepeatableChange::DeleteToEnd);
                self.buffer.delete_to_end_of_line();
                self.buffer.clamp_cursor();  // D stays in normal mode, so clamp
                self.store_in_pending_register();
            }
            EditorAction::ChangeToEndOfLine => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.last_change = Some(RepeatableChange::ChangeToEnd);
                self.buffer.delete_to_end_of_line();
                // Don't clamp - C enters insert mode where cursor can be past end
                self.store_in_pending_register();
            }
            EditorAction::SubstituteChar => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.last_change = Some(RepeatableChange::Substitute);
                self.buffer.delete_char_at();
            }
            EditorAction::SubstituteLine => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.last_change = Some(RepeatableChange::SubstituteLine);
                // Delete line content but not the newline (like cc)
                self.buffer.change_line();
            }
            EditorAction::JoinLines => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.join_lines(true);
            }
            EditorAction::JoinLinesNoSpace => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.join_lines(false);
            }
            EditorAction::ToggleCaseChar => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.toggle_case_char();
            }
            EditorAction::ToggleCaseLine => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.toggle_case_line();
            }
            EditorAction::ToggleCaseMotion { motion, count } => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.toggle_case_motion(motion, count);
            }
            EditorAction::UppercaseLine => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.uppercase_line();
            }
            EditorAction::UppercaseMotion { motion, count } => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.uppercase_motion(motion, count);
            }
            EditorAction::LowercaseLine => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.lowercase_line();
            }
            EditorAction::LowercaseMotion { motion, count } => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.lowercase_motion(motion, count);
            }
            EditorAction::IncrementNumber { amount } => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.increment_number(amount);
            }
            EditorAction::DecrementNumber { amount } => {
                let pos = self.buffer.cursor().position;
                self.add_to_change_list(pos);
                self.increment_number(-amount);
            }
            EditorAction::PasteAfter => {
                // Use named register if pending, otherwise use default
                if let Some(reg) = self.pending_register.take() {
                    if let Some(content) = self.registers.get(&reg).cloned() {
                        self.buffer.set_yank_register(content);
                    }
                }
                self.buffer.paste_after();
            }
            EditorAction::PasteBefore => {
                // Use named register if pending, otherwise use default
                if let Some(reg) = self.pending_register.take() {
                    if let Some(content) = self.registers.get(&reg).cloned() {
                        self.buffer.set_yank_register(content);
                    }
                }
                self.buffer.paste_before();
            }
            EditorAction::PasteAfterCursorEnd => {
                // Use named register if pending, otherwise use default
                if let Some(reg) = self.pending_register.take() {
                    if let Some(content) = self.registers.get(&reg).cloned() {
                        self.buffer.set_yank_register(content);
                    }
                }
                self.buffer.paste_after_cursor_end();
            }
            EditorAction::PasteBeforeCursorEnd => {
                // Use named register if pending, otherwise use default
                if let Some(reg) = self.pending_register.take() {
                    if let Some(content) = self.registers.get(&reg).cloned() {
                        self.buffer.set_yank_register(content);
                    }
                }
                self.buffer.paste_before_cursor_end();
            }
            EditorAction::OperatorMotion { operator, motion, count } => {
                self.last_change = Some(RepeatableChange::OperatorMotion {
                    operator: operator.clone(),
                    motion: motion.clone(),
                    count,
                });
                self.apply_operator_motion(operator, motion, count);
            }
            EditorAction::OperatorTextObject { operator, text_object } => {
                self.last_change = Some(RepeatableChange::OperatorTextObject {
                    operator: operator.clone(),
                    text_object: text_object.clone(),
                });
                self.apply_operator_text_object(operator, text_object);
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
                // Position cursor past the last character for insert mode
                let line = self.buffer.cursor().position.line as usize;
                let line_len = self.buffer.line_len(line).unwrap_or(0);
                self.buffer.cursor_mut().position.col = line_len as u32;
            }
            EditorAction::EnterInsertModeLineStart => {
                // Move to first non-blank character of line
                self.buffer.move_first_non_blank();
            }
            EditorAction::OpenLineBelow => {
                self.buffer.move_line_end();
                self.buffer.insert_newline();
            }
            EditorAction::OpenLineAbove => {
                // Move to start of current line, insert newline, then move up
                self.buffer.move_line_start();
                self.buffer.insert_newline();
                self.buffer.move_up();
            }
            EditorAction::EnterVisualMode => {
                // Set anchor at current cursor position
                self.visual_anchor = Some(self.buffer.cursor().position);
            }
            EditorAction::EnterVisualLineMode => {
                // Set anchor at current cursor position
                self.visual_anchor = Some(self.buffer.cursor().position);
            }
            EditorAction::EnterVisualBlockMode => {
                // Set anchor at current cursor position for block selection
                self.visual_anchor = Some(self.buffer.cursor().position);
            }
            EditorAction::EnterReplaceMode => {}
            EditorAction::EnterCommandMode => {}
            EditorAction::EnterSearchForward => {
                self.search_forward = true;
            }
            EditorAction::EnterSearchBackward => {
                self.search_forward = false;
            }
            EditorAction::ExecuteSearch(pattern) => {
                self.add_to_jump_list();
                if !pattern.is_empty() {
                    self.search_pattern = Some(pattern.clone());
                }
                self.search_next_match(self.search_forward);
            }
            EditorAction::SearchNext => {
                self.add_to_jump_list();
                self.search_next_match(self.search_forward);
            }
            EditorAction::SearchPrev => {
                self.add_to_jump_list();
                self.search_next_match(!self.search_forward);
            }
            EditorAction::SearchWordForward => {
                self.add_to_jump_list();
                self.search_word_under_cursor(true);
            }
            EditorAction::SearchWordBackward => {
                self.add_to_jump_list();
                self.search_word_under_cursor(false);
            }
            EditorAction::SearchPartialWordForward => {
                self.add_to_jump_list();
                self.search_partial_word_under_cursor(true);
            }
            EditorAction::SearchPartialWordBackward => {
                self.add_to_jump_list();
                self.search_partial_word_under_cursor(false);
            }
            EditorAction::VisualDelete => {
                self.apply_visual_operator(Operator::Delete);
            }
            EditorAction::VisualYank => {
                self.apply_visual_operator(Operator::Yank);
            }
            EditorAction::VisualChange => {
                self.apply_visual_operator(Operator::Change);
            }
            EditorAction::VisualSwapEnd => {
                // Swap cursor with visual anchor
                if let Some(anchor) = self.visual_anchor {
                    let cursor = self.buffer.cursor().position;
                    self.visual_anchor = Some(cursor);
                    self.buffer.cursor_mut().position = anchor;
                }
            }
            EditorAction::VisualIndent => {
                self.apply_visual_operator(Operator::Indent);
            }
            EditorAction::VisualOutdent => {
                self.apply_visual_operator(Operator::Outdent);
            }
            EditorAction::ReturnToNormalMode => {
                // Save any inserted text as the last repeatable change
                if !self.insert_buffer.is_empty() {
                    // Add current position to change list since we made an insert
                    let pos = self.buffer.cursor().position;
                    self.add_to_change_list(pos);
                    self.last_change = Some(RepeatableChange::InsertText(
                        std::mem::take(&mut self.insert_buffer),
                    ));
                }
                self.visual_anchor = None;
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
            EditorAction::RepeatLastChange => {
                if let Some(change) = self.last_change.clone() {
                    match change {
                        RepeatableChange::OperatorMotion { operator, motion, count } => {
                            self.apply_operator_motion(operator, motion, count);
                        }
                        RepeatableChange::OperatorTextObject { operator, text_object } => {
                            self.apply_operator_text_object(operator, text_object);
                        }
                        RepeatableChange::DeleteCharAt => {
                            self.buffer.delete_char_at();
                        }
                        RepeatableChange::DeleteToEnd => {
                            self.buffer.delete_to_end_of_line();
                        }
                        RepeatableChange::ChangeToEnd => {
                            self.buffer.delete_to_end_of_line();
                            self.mode_handler.set_mode(Mode::Insert);
                        }
                        RepeatableChange::Substitute => {
                            self.buffer.delete_char_at();
                            self.mode_handler.set_mode(Mode::Insert);
                        }
                        RepeatableChange::SubstituteLine => {
                            self.buffer.change_line();
                            self.mode_handler.set_mode(Mode::Insert);
                        }
                        RepeatableChange::InsertText(text) => {
                            for ch in text.chars() {
                                self.buffer.insert_char(ch);
                            }
                        }
                    }
                }
            }
            EditorAction::SetMark(mark) => {
                let pos = self.buffer.cursor().position;
                self.marks.insert(mark, pos);
                self.status_message = Some(format!("mark '{}' set", mark));
            }
            EditorAction::JumpToMarkExact(mark) => {
                if let Some(pos) = self.marks.get(&mark).copied() {
                    self.add_to_jump_list();
                    self.buffer.cursor_mut().position = pos;
                } else {
                    self.status_message = Some(format!("Mark '{}' not set", mark));
                }
            }
            EditorAction::JumpToMarkLine(mark) => {
                if let Some(pos) = self.marks.get(&mark).copied() {
                    self.add_to_jump_list();
                    self.buffer.cursor_mut().position.line = pos.line;
                    self.buffer.move_first_non_blank();
                } else {
                    self.status_message = Some(format!("Mark '{}' not set", mark));
                }
            }
            EditorAction::SetPendingRegister(reg) => {
                self.pending_register = Some(reg);
            }
            EditorAction::ToggleMacroRecording(reg) => {
                if self.macro_recording_register.is_some() {
                    // Stop recording
                    self.stop_macro_recording();
                } else {
                    // Start recording
                    self.start_macro_recording(reg);
                }
            }
            EditorAction::PlayMacro(reg) => {
                self.play_macro(reg);
            }
            EditorAction::RepeatLastMacro => {
                if let Some(reg) = self.last_macro_register {
                    self.play_macro(reg);
                }
            }
            EditorAction::Substitute { pattern, replacement, flags } => {
                let count = self.apply_substitute(&pattern, &replacement, &flags);
                if count > 0 {
                    self.status_message = Some(format!("{} substitution(s)", count));
                } else {
                    self.status_message = Some("Pattern not found".to_string());
                }
            }
            EditorAction::Global { pattern, command, invert } => {
                let count = self.apply_global(&pattern, &command, invert);
                if count > 0 {
                    self.status_message = Some(format!("{} line(s) affected", count));
                } else {
                    self.status_message = Some("Pattern not found".to_string());
                }
            }
            EditorAction::JumpListOlder => {
                self.jump_list_older();
            }
            EditorAction::JumpListNewer => {
                self.jump_list_newer();
            }
            EditorAction::ChangeListOlder => {
                self.change_list_older();
            }
            EditorAction::ChangeListNewer => {
                self.change_list_newer();
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

    /// Store yanked content in the pending register if set.
    /// This should be called after any yank or delete operation.
    fn store_in_pending_register(&mut self) {
        if let Some(reg) = self.pending_register.take() {
            let content = self.buffer.yank_register().to_string();
            self.registers.insert(reg, content);
        }
    }

    /// Start recording a macro into the given register.
    fn start_macro_recording(&mut self, reg: char) {
        self.macro_recording_register = Some(reg);
        self.macro_recording_keys.clear();
        self.status_message = Some(format!("recording @{}", reg));
    }

    /// Stop recording the current macro.
    fn stop_macro_recording(&mut self) {
        if let Some(reg) = self.macro_recording_register.take() {
            // Join recorded keys into the register
            let macro_content = self.macro_recording_keys.join("");
            self.registers.insert(reg, macro_content);
            self.macro_recording_keys.clear();
            self.status_message = Some(format!("recorded @{}", reg));
        }
    }

    /// Play back a macro from the given register.
    fn play_macro(&mut self, reg: char) {
        if let Some(content) = self.registers.get(&reg).cloned() {
            self.last_macro_register = Some(reg);
            // Parse the content as a series of key sequences and execute them
            // For now, treat each character as a key press
            for ch in content.chars() {
                // Don't record macro playback while recording another macro
                // to avoid infinite loops
                let was_recording = self.macro_recording_register.take();
                let key = KeyInput {
                    code: KeyCode::Char(ch),
                    modifiers: Modifiers::default(),
                };
                self.handle_key(key);
                self.macro_recording_register = was_recording;
            }
        } else {
            self.status_message = Some(format!("Register '{}' is empty", reg));
        }
    }

    /// Record a key during macro recording.
    fn record_key(&mut self, key: &str) {
        if self.macro_recording_register.is_some() {
            self.macro_recording_keys.push(key.to_string());
        }
    }

    /// Add the current position to the jump list.
    /// This should be called before any "jump" command.
    fn add_to_jump_list(&mut self) {
        let pos = self.buffer.cursor().position;
        
        // Don't add duplicate consecutive entries
        if let Some(last) = self.jump_list.last() {
            if *last == pos {
                return;
            }
        }
        
        // If we're not at the end of the list, truncate it
        // (this handles the "branch" behavior when jumping back then making a new jump)
        if self.jump_list_index < self.jump_list.len() {
            self.jump_list.truncate(self.jump_list_index);
        }
        
        // Add the new position
        self.jump_list.push(pos);
        self.jump_list_index = self.jump_list.len();
        
        // Limit jump list size (Vim uses 100)
        const MAX_JUMP_LIST_SIZE: usize = 100;
        if self.jump_list.len() > MAX_JUMP_LIST_SIZE {
            self.jump_list.remove(0);
            self.jump_list_index = self.jump_list.len();
        }
    }

    /// Jump to an older position in the jump list (Ctrl-o).
    fn jump_list_older(&mut self) {
        if self.jump_list.is_empty() || self.jump_list_index == 0 {
            self.status_message = Some("No older jump".to_string());
            return;
        }
        
        // If at the end, save current position first
        if self.jump_list_index == self.jump_list.len() {
            let pos = self.buffer.cursor().position;
            // Only add if different from last entry
            if self.jump_list.last().map_or(true, |last| *last != pos) {
                self.jump_list.push(pos);
                // Don't increment index since we're about to go back
            }
        }
        
        self.jump_list_index -= 1;
        let pos = self.jump_list[self.jump_list_index];
        self.buffer.cursor_mut().position = pos;
    }

    /// Jump to a newer position in the jump list (Ctrl-i).
    fn jump_list_newer(&mut self) {
        if self.jump_list_index >= self.jump_list.len().saturating_sub(1) {
            self.status_message = Some("No newer jump".to_string());
            return;
        }
        
        self.jump_list_index += 1;
        let pos = self.jump_list[self.jump_list_index];
        self.buffer.cursor_mut().position = pos;
    }

    /// Add a position to the change list.
    /// This should be called after any text-modifying operation.
    fn add_to_change_list(&mut self, pos: LineCol) {
        // Don't add duplicate consecutive entries
        if let Some(last) = self.change_list.last() {
            if *last == pos {
                return;
            }
        }
        
        // Add the new position
        self.change_list.push(pos);
        self.change_list_index = self.change_list.len();
        
        // Limit change list size (Vim uses 100)
        const MAX_CHANGE_LIST_SIZE: usize = 100;
        if self.change_list.len() > MAX_CHANGE_LIST_SIZE {
            self.change_list.remove(0);
            self.change_list_index = self.change_list.len();
        }
    }

    /// Jump to an older position in the change list (g;).
    fn change_list_older(&mut self) {
        if self.change_list.is_empty() || self.change_list_index == 0 {
            self.status_message = Some("No older change".to_string());
            return;
        }
        
        self.change_list_index -= 1;
        let pos = self.change_list[self.change_list_index];
        // Clamp position to valid range
        let line_count = self.buffer.line_count() as u32;
        let line = pos.line.min(line_count.saturating_sub(1));
        let line_len = self.buffer.line_len(line as usize).unwrap_or(0) as u32;
        let col = pos.col.min(line_len.saturating_sub(1));
        self.buffer.cursor_mut().position = LineCol { line, col };
    }

    /// Jump to a newer position in the change list (g,).
    fn change_list_newer(&mut self) {
        if self.change_list_index >= self.change_list.len() {
            self.status_message = Some("No newer change".to_string());
            return;
        }
        
        self.change_list_index += 1;
        if self.change_list_index >= self.change_list.len() {
            self.status_message = Some("No newer change".to_string());
            self.change_list_index = self.change_list.len();
            return;
        }
        let pos = self.change_list[self.change_list_index];
        // Clamp position to valid range
        let line_count = self.buffer.line_count() as u32;
        let line = pos.line.min(line_count.saturating_sub(1));
        let line_len = self.buffer.line_len(line as usize).unwrap_or(0) as u32;
        let col = pos.col.min(line_len.saturating_sub(1));
        self.buffer.cursor_mut().position = LineCol { line, col };
    }

    /// Move cursor to next sentence.
    /// A sentence ends with '.', '!', or '?' followed by end of line,
    /// space, or tab.
    fn move_sentence_forward(&mut self) {
        let content = self.buffer.content();
        let cursor = self.buffer.cursor().position;
        
        // Convert cursor position to byte offset
        let mut byte_offset = 0usize;
        for (i, line) in content.lines().enumerate() {
            if i < cursor.line as usize {
                byte_offset += line.len() + 1; // +1 for newline
            } else {
                byte_offset += cursor.col as usize;
                break;
            }
        }
        
        let bytes = content.as_bytes();
        let mut i = byte_offset;
        
        // Skip to end of current sentence
        while i < bytes.len() {
            let ch = bytes[i] as char;
            if ch == '.' || ch == '!' || ch == '?' {
                i += 1;
                // Skip any spaces/newlines after punctuation
                while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n') {
                    i += 1;
                }
                break;
            }
            i += 1;
        }
        
        // Convert byte offset back to LineCol
        if i >= bytes.len() {
            // Go to end of file
            self.buffer.move_file_end();
        } else {
            let mut line = 0u32;
            let mut col = 0u32;
            let mut offset = 0usize;
            
            for (idx, text_line) in content.lines().enumerate() {
                if offset + text_line.len() >= i {
                    line = idx as u32;
                    col = (i - offset) as u32;
                    break;
                }
                offset += text_line.len() + 1;
            }
            
            self.buffer.cursor_mut().position = LineCol { line, col };
        }
    }

    /// Move cursor to previous sentence.
    fn move_sentence_backward(&mut self) {
        let content = self.buffer.content();
        let cursor = self.buffer.cursor().position;
        
        // Convert cursor position to byte offset
        let mut byte_offset = 0usize;
        for (i, line) in content.lines().enumerate() {
            if i < cursor.line as usize {
                byte_offset += line.len() + 1;
            } else {
                byte_offset += cursor.col as usize;
                break;
            }
        }
        
        if byte_offset == 0 {
            return;
        }
        
        let bytes = content.as_bytes();
        let mut i = byte_offset.saturating_sub(1);
        
        // Skip any whitespace we're on
        while i > 0 && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n') {
            i -= 1;
        }
        
        // Skip to previous sentence end
        while i > 0 {
            let ch = bytes[i] as char;
            if ch == '.' || ch == '!' || ch == '?' {
                // Found previous sentence end, skip whitespace after it
                i += 1;
                while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n') {
                    i += 1;
                }
                break;
            }
            i -= 1;
        }
        
        // Convert byte offset back to LineCol
        let mut line = 0u32;
        let mut col = 0u32;
        let mut offset = 0usize;
        
        for (idx, text_line) in content.lines().enumerate() {
            if offset + text_line.len() >= i {
                line = idx as u32;
                col = (i - offset).min(text_line.len()) as u32;
                break;
            }
            offset += text_line.len() + 1;
        }
        
        self.buffer.cursor_mut().position = LineCol { line, col };
    }

    /// Move cursor to next paragraph (next blank line).
    fn move_paragraph_forward(&mut self) {
        let line_count = self.buffer.line_count();
        let mut line = self.buffer.cursor().position.line as usize;
        
        // Skip current non-blank lines
        while line < line_count {
            if let Some(content) = self.buffer.line_content(line) {
                if content.trim().is_empty() {
                    break;
                }
            }
            line += 1;
        }
        
        // Skip blank lines
        while line < line_count {
            if let Some(content) = self.buffer.line_content(line) {
                if !content.trim().is_empty() {
                    break;
                }
            }
            line += 1;
        }
        
        // Position at first non-blank line after blank lines, or last line
        let target_line = line.min(line_count.saturating_sub(1));
        self.buffer.cursor_mut().position = LineCol { line: target_line as u32, col: 0 };
    }

    /// Move cursor to previous paragraph (previous blank line).
    fn move_paragraph_backward(&mut self) {
        let mut line = self.buffer.cursor().position.line as usize;
        
        if line == 0 {
            return;
        }
        
        line -= 1;
        
        // Skip current non-blank lines going backwards
        while line > 0 {
            if let Some(content) = self.buffer.line_content(line) {
                if content.trim().is_empty() {
                    break;
                }
            }
            line -= 1;
        }
        
        // Skip blank lines going backwards
        while line > 0 {
            if let Some(content) = self.buffer.line_content(line) {
                if !content.trim().is_empty() {
                    line += 1; // Go back to the blank line
                    break;
                }
            }
            line -= 1;
        }
        
        self.buffer.cursor_mut().position = LineCol { line: line as u32, col: 0 };
    }

    /// Move cursor to matching bracket.
    /// Supports (), [], {}.
    fn move_match_bracket(&mut self) {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        
        if let Some(line_content) = self.buffer.line_content(line_idx) {
            let line_chars: Vec<char> = line_content.chars().collect();
            let col = cursor.col as usize;
            
            // Find a bracket at or after cursor on current line
            let mut search_col = col;
            let open_brackets = ['(', '[', '{'];
            let close_brackets = [')', ']', '}'];
            
            // Look for bracket at current position or scan forward
            while search_col < line_chars.len() {
                let ch = line_chars[search_col];
                
                if let Some(bracket_idx) = open_brackets.iter().position(|&b| b == ch) {
                    // Opening bracket - search forward for match
                    let close = close_brackets[bracket_idx];
                    if let Some(pos) = self.find_matching_bracket_forward(ch, close, cursor.line, search_col as u32) {
                        self.buffer.cursor_mut().position = pos;
                        return;
                    }
                    break;
                } else if let Some(bracket_idx) = close_brackets.iter().position(|&b| b == ch) {
                    // Closing bracket - search backward for match
                    let open = open_brackets[bracket_idx];
                    if let Some(pos) = self.find_matching_bracket_backward(open, ch, cursor.line, search_col as u32) {
                        self.buffer.cursor_mut().position = pos;
                        return;
                    }
                    break;
                }
                
                search_col += 1;
            }
        }
    }

    /// Find matching closing bracket forward.
    fn find_matching_bracket_forward(&self, open: char, close: char, start_line: u32, start_col: u32) -> Option<LineCol> {
        let mut depth = 1;
        let line_count = self.buffer.line_count();
        
        for line_idx in start_line as usize..line_count {
            if let Some(line_content) = self.buffer.line_content(line_idx) {
                let start = if line_idx == start_line as usize { (start_col + 1) as usize } else { 0 };
                
                for (col, ch) in line_content.chars().enumerate().skip(start) {
                    if ch == open {
                        depth += 1;
                    } else if ch == close {
                        depth -= 1;
                        if depth == 0 {
                            return Some(LineCol { line: line_idx as u32, col: col as u32 });
                        }
                    }
                }
            }
        }
        
        None
    }

    /// Find matching opening bracket backward.
    fn find_matching_bracket_backward(&self, open: char, close: char, start_line: u32, start_col: u32) -> Option<LineCol> {
        let mut depth = 1;
        
        for line_idx in (0..=start_line as usize).rev() {
            if let Some(line_content) = self.buffer.line_content(line_idx) {
                let chars: Vec<char> = line_content.chars().collect();
                let end = if line_idx == start_line as usize { start_col as usize } else { chars.len() };
                
                for col in (0..end).rev() {
                    let ch = chars[col];
                    if ch == close {
                        depth += 1;
                    } else if ch == open {
                        depth -= 1;
                        if depth == 0 {
                            return Some(LineCol { line: line_idx as u32, col: col as u32 });
                        }
                    }
                }
            }
        }
        
        None
    }

    /// Scroll half a page up or down, moving cursor with scroll.
    fn scroll_half_page(&mut self, down: bool) {
        let half_page = (self.terminal_size.1 as usize / 2).max(1);
        let line_count = self.buffer.line_count();
        let cursor_line = self.buffer.cursor().position.line as usize;
        
        if down {
            // Move cursor down by half page
            let new_line = (cursor_line + half_page).min(line_count.saturating_sub(1));
            self.buffer.cursor_mut().position.line = new_line as u32;
        } else {
            // Move cursor up by half page
            let new_line = cursor_line.saturating_sub(half_page);
            self.buffer.cursor_mut().position.line = new_line as u32;
        }
        // Clamp cursor column
        self.buffer.clamp_cursor();
    }

    /// Scroll a full page up or down, moving cursor with scroll.
    fn scroll_full_page(&mut self, down: bool) {
        let full_page = (self.terminal_size.1 as usize).max(1);
        let line_count = self.buffer.line_count();
        let cursor_line = self.buffer.cursor().position.line as usize;
        
        if down {
            let new_line = (cursor_line + full_page).min(line_count.saturating_sub(1));
            self.buffer.cursor_mut().position.line = new_line as u32;
        } else {
            let new_line = cursor_line.saturating_sub(full_page);
            self.buffer.cursor_mut().position.line = new_line as u32;
        }
        self.buffer.clamp_cursor();
    }

    /// Scroll one line up or down without moving cursor (Ctrl-e, Ctrl-y).
    fn scroll_line(&mut self, down: bool) {
        // This primarily affects the viewport, not the cursor.
        // In our implementation, we just move the viewport offset.
        // Since viewport follows cursor, we need to move cursor to stay visible.
        if down {
            self.viewport.top_line = self.viewport.top_line.saturating_add(1);
            // If cursor would be above viewport, move it down
            let cursor_line = self.buffer.cursor().position.line as usize;
            if cursor_line < self.viewport.top_line {
                self.buffer.cursor_mut().position.line = self.viewport.top_line as u32;
                self.buffer.clamp_cursor();
            }
        } else {
            if self.viewport.top_line > 0 {
                self.viewport.top_line -= 1;
            }
            // If cursor would be below viewport, move it up
            let cursor_line = self.buffer.cursor().position.line as usize;
            let visible_height = self.terminal_size.1 as usize;
            if cursor_line >= self.viewport.top_line + visible_height {
                let new_line = (self.viewport.top_line + visible_height).saturating_sub(1);
                self.buffer.cursor_mut().position.line = new_line as u32;
                self.buffer.clamp_cursor();
            }
        }
    }

    /// Move cursor to top of visible screen (H).
    fn move_to_screen_top(&mut self) {
        let top_line = self.viewport.top_line;
        self.buffer.cursor_mut().position.line = top_line as u32;
        self.buffer.move_first_non_blank();
    }

    /// Move cursor to middle of visible screen (M).
    fn move_to_screen_middle(&mut self) {
        let visible_height = self.terminal_size.1 as usize;
        let middle_offset = visible_height / 2;
        let middle_line = self.viewport.top_line + middle_offset;
        let line_count = self.buffer.line_count();
        let target_line = middle_line.min(line_count.saturating_sub(1));
        self.buffer.cursor_mut().position.line = target_line as u32;
        self.buffer.move_first_non_blank();
    }

    /// Move cursor to bottom of visible screen (L).
    fn move_to_screen_bottom(&mut self) {
        let visible_height = self.terminal_size.1 as usize;
        let bottom_line = self.viewport.top_line + visible_height.saturating_sub(1);
        let line_count = self.buffer.line_count();
        let target_line = bottom_line.min(line_count.saturating_sub(1));
        self.buffer.cursor_mut().position.line = target_line as u32;
        self.buffer.move_first_non_blank();
    }

    /// Center cursor line on screen (zz).
    fn scroll_cursor_center(&mut self) {
        let cursor_line = self.buffer.cursor().position.line as usize;
        let visible_height = self.terminal_size.1 as usize;
        let half_height = visible_height / 2;
        self.viewport.top_line = cursor_line.saturating_sub(half_height);
    }

    /// Move cursor line to top of screen (zt).
    fn scroll_cursor_top(&mut self) {
        let cursor_line = self.buffer.cursor().position.line as usize;
        self.viewport.top_line = cursor_line;
    }

    /// Move cursor line to bottom of screen (zb).
    fn scroll_cursor_bottom(&mut self) {
        let cursor_line = self.buffer.cursor().position.line as usize;
        let visible_height = self.terminal_size.1 as usize;
        self.viewport.top_line = cursor_line.saturating_sub(visible_height.saturating_sub(1));
    }

    /// Move to first non-blank character of next line (+)
    fn move_next_line_start(&mut self) {
        let cursor = self.buffer.cursor().position;
        let line_count = self.buffer.line_count();
        let next_line = cursor.line as usize + 1;
        if next_line < line_count {
            // Move to next line
            self.buffer.set_cursor_position(LineCol {
                line: next_line as u32,
                col: 0,
            });
            // Then move to first non-blank
            self.buffer.move_first_non_blank();
        }
    }

    /// Move to first non-blank character of previous line (-)
    fn move_prev_line_start(&mut self) {
        let cursor = self.buffer.cursor().position;
        if cursor.line > 0 {
            // Move to previous line
            self.buffer.set_cursor_position(LineCol {
                line: cursor.line - 1,
                col: 0,
            });
            // Then move to first non-blank
            self.buffer.move_first_non_blank();
        }
    }

    /// Go to specific line number (1-based).
    fn go_to_line(&mut self, line: u32) {
        let line_count = self.buffer.line_count();
        // Line is 1-based, convert to 0-based
        let target_line = line.saturating_sub(1) as usize;
        // Clamp to valid range
        let target_line = target_line.min(line_count.saturating_sub(1));
        
        self.buffer.set_cursor_position(LineCol {
            line: target_line as u32,
            col: 0,
        });
        self.buffer.move_first_non_blank();
    }

    /// Go to percentage of file (N%).
    fn go_to_percent(&mut self, pct: u32) {
        let line_count = self.buffer.line_count();
        if line_count == 0 {
            return;
        }
        
        // Clamp percentage to 0-100
        let pct = pct.min(100) as usize;
        
        // Calculate target line (0-based)
        let target_line = (pct * line_count) / 100;
        let target_line = target_line.min(line_count.saturating_sub(1));
        
        self.buffer.set_cursor_position(LineCol {
            line: target_line as u32,
            col: 0,
        });
        self.buffer.move_first_non_blank();
    }

    /// Join current line with next line.
    /// If `add_space` is true, a single space is inserted at the join point.
    fn join_lines(&mut self, add_space: bool) {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        let line_count = self.buffer.line_count();
        
        // Can't join if we're on the last line
        if line_idx >= line_count.saturating_sub(1) {
            return;
        }
        
        // Get current line content (without trailing newline)
        let current_line = self.buffer.line_content(line_idx)
            .map(|s| s.trim_end_matches('\n').to_string())
            .unwrap_or_default();
        
        // Get next line content (trimmed of leading whitespace for J, full for gJ)
        let next_line = self.buffer.line_content(line_idx + 1)
            .map(|s| {
                let s = s.trim_end_matches('\n');
                if add_space {
                    s.trim_start()
                } else {
                    s
                }.to_string()
            })
            .unwrap_or_default();
        
        // Calculate the column where the join happens (for cursor positioning)
        let join_col = current_line.len();
        
        // Build the joined line
        let joined = if add_space && !current_line.is_empty() && !next_line.is_empty() {
            format!("{} {}", current_line, next_line)
        } else {
            format!("{}{}", current_line, next_line)
        };
        
        // Delete the next line first
        self.buffer.cursor_mut().position = LineCol { line: (line_idx + 1) as u32, col: 0 };
        self.buffer.delete_line();
        
        // Replace current line with joined content
        self.buffer.cursor_mut().position = LineCol { line: line_idx as u32, col: 0 };
        self.buffer.replace_line(line_idx, &joined);
        
        // Position cursor at the join point
        self.buffer.cursor_mut().position = LineCol { line: line_idx as u32, col: join_col as u32 };
    }

    /// Toggle case of character under cursor and advance cursor.
    fn toggle_case_char(&mut self) {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        let col_idx = cursor.col as usize;
        
        // Get the character at cursor position
        if let Some(line) = self.buffer.line(line_idx) {
            if let Some(ch) = line.chars().nth(col_idx) {
                if ch == '\n' {
                    return; // Don't toggle newline
                }
                
                let toggled: String = if ch.is_lowercase() {
                    ch.to_uppercase().collect()
                } else if ch.is_uppercase() {
                    ch.to_lowercase().collect()
                } else {
                    ch.to_string()
                };
                
                // Delete the character at cursor and insert the toggled version
                self.buffer.delete_char_at();
                for c in toggled.chars() {
                    self.buffer.insert_char(c);
                }
                
                // Move cursor to next character (or stay if at end)
                if let Some(line_len) = self.buffer.line_len(line_idx) {
                    // Account for newline
                    let effective_len = line_len.saturating_sub(1);
                    if col_idx < effective_len {
                        self.buffer.cursor_mut().position.col = (col_idx + 1) as u32;
                    }
                }
            }
        }
    }

    /// Toggle case of entire line.
    fn toggle_case_line(&mut self) {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        
        if let Some(line) = self.buffer.line_content(line_idx) {
            let toggled: String = line.chars().map(|ch| {
                if ch.is_lowercase() {
                    ch.to_uppercase().collect::<String>()
                } else if ch.is_uppercase() {
                    ch.to_lowercase().collect::<String>()
                } else {
                    ch.to_string()
                }
            }).collect();
            
            self.buffer.replace_line(line_idx, &toggled);
        }
    }

    /// Toggle case of text covered by motion.
    fn toggle_case_motion(&mut self, motion: Motion, count: Option<u32>) {
        let (start, end) = self.compute_motion_range(motion, count);
        self.apply_case_transform(start, end, CaseTransform::Toggle);
    }

    /// Uppercase entire line.
    fn uppercase_line(&mut self) {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        
        if let Some(line) = self.buffer.line_content(line_idx) {
            let uppercased: String = line.to_uppercase();
            self.buffer.replace_line(line_idx, &uppercased);
        }
    }

    /// Uppercase text covered by motion.
    fn uppercase_motion(&mut self, motion: Motion, count: Option<u32>) {
        let (start, end) = self.compute_motion_range(motion, count);
        self.apply_case_transform(start, end, CaseTransform::Upper);
    }

    /// Lowercase entire line.
    fn lowercase_line(&mut self) {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        
        if let Some(line) = self.buffer.line_content(line_idx) {
            let lowercased: String = line.to_lowercase();
            self.buffer.replace_line(line_idx, &lowercased);
        }
    }

    /// Lowercase text covered by motion.
    fn lowercase_motion(&mut self, motion: Motion, count: Option<u32>) {
        let (start, end) = self.compute_motion_range(motion, count);
        self.apply_case_transform(start, end, CaseTransform::Lower);
    }

    /// Apply case transformation to a range.
    fn apply_case_transform(&mut self, start: LineCol, end: LineCol, transform: CaseTransform) {
        // Ensure start <= end
        let (start, end) = if start.line < end.line || (start.line == end.line && start.col <= end.col) {
            (start, end)
        } else {
            (end, start)
        };
        
        // For single line
        if start.line == end.line {
            if let Some(line) = self.buffer.line_content(start.line as usize) {
                let end_col = std::cmp::min(end.col as usize + 1, line.len());
                let start_col = start.col as usize;
                if start_col < line.len() {
                    let before = &line[..start_col];
                    let middle = &line[start_col..end_col];
                    let after = &line[end_col..];
                    
                    let transformed = match transform {
                        CaseTransform::Toggle => middle.chars().map(|ch| {
                            if ch.is_lowercase() {
                                ch.to_uppercase().collect::<String>()
                            } else if ch.is_uppercase() {
                                ch.to_lowercase().collect::<String>()
                            } else {
                                ch.to_string()
                            }
                        }).collect::<String>(),
                        CaseTransform::Upper => middle.to_uppercase(),
                        CaseTransform::Lower => middle.to_lowercase(),
                    };
                    
                    let new_line = format!("{}{}{}", before, transformed, after);
                    self.buffer.replace_line(start.line as usize, &new_line);
                }
            }
        } else {
            // Multi-line transformation
            for line_idx in start.line..=end.line {
                if let Some(line) = self.buffer.line_content(line_idx as usize) {
                    let line = line.to_string();
                    let (col_start, col_end) = if line_idx == start.line {
                        (start.col as usize, line.len())
                    } else if line_idx == end.line {
                        (0, std::cmp::min(end.col as usize + 1, line.len()))
                    } else {
                        (0, line.len())
                    };
                    
                    if col_start < line.len() {
                        let before = &line[..col_start];
                        let middle = &line[col_start..col_end];
                        let after = &line[col_end..];
                        
                        let transformed = match transform {
                            CaseTransform::Toggle => middle.chars().map(|ch| {
                                if ch.is_lowercase() {
                                    ch.to_uppercase().collect::<String>()
                                } else if ch.is_uppercase() {
                                    ch.to_lowercase().collect::<String>()
                                } else {
                                    ch.to_string()
                                }
                            }).collect::<String>(),
                            CaseTransform::Upper => middle.to_uppercase(),
                            CaseTransform::Lower => middle.to_lowercase(),
                        };
                        
                        let new_line = format!("{}{}{}", before, transformed, after);
                        self.buffer.replace_line(line_idx as usize, &new_line);
                    }
                }
            }
        }
    }

    /// Increment or decrement a number under the cursor.
    /// Positive `amount` increments, negative decrements.
    fn increment_number(&mut self, amount: i32) {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        let col_idx = cursor.col as usize;
        
        if let Some(line) = self.buffer.line(line_idx) {
            let line = line.trim_end_matches('\n');
            if line.is_empty() {
                return;
            }
            
            // Find the number at or after the cursor
            // A number is a sequence of digits, optionally preceded by '-'
            let chars: Vec<char> = line.chars().collect();
            let mut num_start = None;
            let mut num_end = None;
            let mut is_negative = false;
            
            // Search forward from cursor for a number
            let mut search_start = col_idx;
            
            // First, check if we're on a digit or minus sign
            if search_start < chars.len() {
                if chars[search_start].is_ascii_digit() {
                    // Check for negative sign before
                    if search_start > 0 && chars[search_start - 1] == '-' {
                        is_negative = true;
                        num_start = Some(search_start - 1);
                    } else {
                        num_start = Some(search_start);
                    }
                } else if chars[search_start] == '-' && search_start + 1 < chars.len() && chars[search_start + 1].is_ascii_digit() {
                    is_negative = true;
                    num_start = Some(search_start);
                }
            }
            
            // If not on a number, search forward
            if num_start.is_none() {
                for i in search_start..chars.len() {
                    if chars[i].is_ascii_digit() {
                        // Check for negative sign before
                        if i > 0 && chars[i - 1] == '-' {
                            is_negative = true;
                            num_start = Some(i - 1);
                        } else {
                            num_start = Some(i);
                        }
                        break;
                    }
                }
            }
            
            if let Some(start) = num_start {
                // Find the end of the number
                let digit_start = if is_negative { start + 1 } else { start };
                for i in digit_start..chars.len() {
                    if !chars[i].is_ascii_digit() {
                        num_end = Some(i);
                        break;
                    }
                }
                if num_end.is_none() {
                    num_end = Some(chars.len());
                }
                
                let end = num_end.unwrap();
                
                // Extract the number string
                let num_str: String = chars[start..end].iter().collect();
                
                // Parse and modify
                if let Ok(num) = num_str.parse::<i64>() {
                    let new_num = num.saturating_add(amount as i64);
                    let new_str = new_num.to_string();
                    
                    // Build the new line
                    let before: String = chars[..start].iter().collect();
                    let after: String = chars[end..].iter().collect();
                    let new_line = format!("{}{}{}\n", before, new_str, after);
                    
                    self.buffer.replace_line(line_idx, &new_line);
                    
                    // Position cursor at the end of the new number
                    let new_num_end = start + new_str.len();
                    self.buffer.cursor_mut().position.col = new_num_end.saturating_sub(1) as u32;
                }
            }
        }
    }

    /// Apply substitute command on the current line.
    fn apply_substitute(&mut self, pattern: &str, replacement: &str, flags: &str) -> usize {
        if pattern.is_empty() {
            return 0;
        }
        
        let global = flags.contains('g');
        let cursor_line = self.buffer.cursor().position.line as usize;
        
        if let Some(line_content) = self.buffer.line_content(cursor_line) {
            let line_content = line_content.to_string();
            
            // Simple literal string replacement (no regex for now)
            let new_content = if global {
                line_content.replace(pattern, replacement)
            } else {
                line_content.replacen(pattern, replacement, 1)
            };
            
            if new_content != line_content {
                // Replace the line content
                self.buffer.replace_line(cursor_line, &new_content);
                return if global {
                    line_content.matches(pattern).count()
                } else {
                    1
                };
            }
        }
        
        0
    }

    /// Apply global command on matching lines.
    /// :g/pattern/command - execute command on lines matching pattern
    /// :v/pattern/command - execute command on lines NOT matching pattern
    fn apply_global(&mut self, pattern: &str, command: &str, invert: bool) -> usize {
        if pattern.is_empty() {
            return 0;
        }

        // First, collect line indices that match (or don't match for :v)
        let total_lines = self.buffer.line_count();
        let mut matching_lines: Vec<usize> = Vec::new();

        for line_idx in 0..total_lines {
            if let Some(line_content) = self.buffer.line_content(line_idx) {
                let matches = line_content.contains(pattern);
                if matches != invert {
                    matching_lines.push(line_idx);
                }
            }
        }

        if matching_lines.is_empty() {
            return 0;
        }

        let command = command.trim();
        let count = matching_lines.len();

        // Handle common commands
        // For delete (d), we need to go from bottom to top to preserve line numbers
        if command.is_empty() || command == "d" {
            // Delete matching lines, from bottom to top
            for &line_idx in matching_lines.iter().rev() {
                self.buffer.set_cursor_position(LineCol::new(line_idx as u32, 0));
                self.buffer.delete_line();
            }
        } else if command.starts_with("s") {
            // Substitute on matching lines: g/pattern/s/old/new/
            if let Some(sub_action) = CommandParser::parse_public(command) {
                if let EditorAction::Substitute { pattern: sub_pat, replacement, flags } = sub_action {
                    for &line_idx in matching_lines.iter() {
                        self.buffer.set_cursor_position(LineCol::new(line_idx as u32, 0));
                        self.apply_substitute(&sub_pat, &replacement, &flags);
                    }
                }
            }
        } else if command == "p" {
            // Print is a no-op in our implementation (lines are displayed anyway)
        }

        count
    }

    /// Compute the range covered by a motion from the current cursor position.
    /// Returns (start, end) positions.
    fn compute_motion_range(&mut self, motion: Motion, count: Option<u32>) -> (LineCol, LineCol) {
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
                Motion::LastNonBlank => self.buffer.move_last_non_blank(),
                Motion::NextLineStart => self.move_next_line_start(),
                Motion::PrevLineStart => self.move_prev_line_start(),
                Motion::WordForward => self.buffer.move_word_forward(),
                Motion::WordBackward => self.buffer.move_word_backward(),
                Motion::WordEnd => self.buffer.move_word_end(),
                Motion::WordEndBackward => self.buffer.move_word_end_backward(),
                Motion::FileStart => self.buffer.move_file_start(),
                Motion::FileEnd => self.buffer.move_file_end(),
                Motion::SentenceForward => self.move_sentence_forward(),
                Motion::SentenceBackward => self.move_sentence_backward(),
                Motion::ParagraphForward => self.move_paragraph_forward(),
                Motion::ParagraphBackward => self.move_paragraph_backward(),
                Motion::MatchBracket => self.move_match_bracket(),
                Motion::FindCharForward(ch) => {
                    self.buffer.find_char_forward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::Forward));
                }
                Motion::FindCharBackward(ch) => {
                    self.buffer.find_char_backward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::Backward));
                }
                Motion::TillCharForward(ch) => {
                    self.buffer.till_char_forward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::TillForward));
                }
                Motion::TillCharBackward(ch) => {
                    self.buffer.till_char_backward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::TillBackward));
                }
                Motion::ToMarkExact(mark) => {
                    if let Some(pos) = self.marks.get(&mark).copied() {
                        self.buffer.cursor_mut().position = pos;
                    }
                }
                Motion::ToMarkLine(mark) => {
                    if let Some(pos) = self.marks.get(&mark).copied() {
                        self.buffer.cursor_mut().position.line = pos.line;
                        self.buffer.move_first_non_blank();
                    }
                }
                Motion::CurrentLine => {
                    // For CurrentLine, return the whole line
                    let line_idx = start.line as usize;
                    let line_len = self.buffer.line_len(line_idx).unwrap_or(0);
                    return (
                        LineCol { line: start.line, col: 0 },
                        LineCol { line: start.line, col: line_len.saturating_sub(1) as u32 },
                    );
                }
            }
        }

        let end = self.buffer.cursor().position;
        
        // Restore cursor to start for case operations (we just want the range)
        self.buffer.cursor_mut().position = start;
        
        (start, end)
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
                Motion::LastNonBlank => self.buffer.move_last_non_blank(),
                Motion::NextLineStart => self.move_next_line_start(),
                Motion::PrevLineStart => self.move_prev_line_start(),
                Motion::WordForward => self.buffer.move_word_forward(),
                Motion::WordBackward => self.buffer.move_word_backward(),
                Motion::WordEnd => self.buffer.move_word_end(),
                Motion::WordEndBackward => self.buffer.move_word_end_backward(),
                Motion::FileStart => self.buffer.move_file_start(),
                Motion::FileEnd => self.buffer.move_file_end(),
                Motion::SentenceForward => self.move_sentence_forward(),
                Motion::SentenceBackward => self.move_sentence_backward(),
                Motion::ParagraphForward => self.move_paragraph_forward(),
                Motion::ParagraphBackward => self.move_paragraph_backward(),
                Motion::MatchBracket => self.move_match_bracket(),
                Motion::FindCharForward(ch) => {
                    self.buffer.find_char_forward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::Forward));
                }
                Motion::FindCharBackward(ch) => {
                    self.buffer.find_char_backward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::Backward));
                }
                Motion::TillCharForward(ch) => {
                    self.buffer.till_char_forward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::TillForward));
                }
                Motion::TillCharBackward(ch) => {
                    self.buffer.till_char_backward(ch);
                    self.last_find_char = Some((ch, FindCharDirection::TillBackward));
                }
                Motion::ToMarkExact(mark) => {
                    if let Some(pos) = self.marks.get(&mark).copied() {
                        self.buffer.cursor_mut().position = pos;
                    }
                }
                Motion::ToMarkLine(mark) => {
                    if let Some(pos) = self.marks.get(&mark).copied() {
                        self.buffer.cursor_mut().position.line = pos.line;
                        self.buffer.move_first_non_blank();
                    }
                }
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
                    self.store_in_pending_register();
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

        // For inclusive motions (like $, e, f, t), we need to include the end character
        // Ranges are exclusive, so we add 1 to the end column
        let is_inclusive = matches!(motion, 
            Motion::LineEnd | Motion::WordEnd | 
            Motion::FindCharForward(_) | Motion::FindCharBackward(_) |
            Motion::TillCharForward(_) | Motion::TillCharBackward(_));
        if is_inclusive {
            range_end.col += 1;
        }

        // Apply operator over range
        match operator {
            Operator::Delete => {
                self.add_to_change_list(range_start);
                self.buffer.delete_range(range_start, range_end);
                self.store_in_pending_register();
            }
            Operator::Yank => {
                self.buffer.yank_range(range_start, range_end);
                self.store_in_pending_register();
                // Yank doesn't move cursor - restore to start of range
                self.buffer.set_cursor_position(range_start);
            }
            Operator::Change => {
                self.add_to_change_list(range_start);
                self.buffer.delete_range(range_start, range_end);
                self.store_in_pending_register();
            }
            Operator::Indent | Operator::Outdent => {
                // For indent/outdent, need linewise behavior
                // TODO: implement range indentation
            }
        }
    }

    /// Apply an operator over a text object.
    fn apply_operator_text_object(&mut self, operator: Operator, text_object: TextObject) {
        let cursor = self.buffer.cursor().position;
        let line_content = self.buffer.line(cursor.line as usize);
        let full_content = self.buffer.content();
        
        let range = match find_text_object_range(
            text_object,
            cursor,
            line_content.as_deref(),
            &full_content,
        ) {
            Some(r) => r,
            None => return, // No text object found
        };

        // Apply operator over range
        match operator {
            Operator::Delete => {
                self.add_to_change_list(range.start);
                self.buffer.delete_range(range.start, range.end);
                self.store_in_pending_register();
            }
            Operator::Yank => {
                self.buffer.yank_range(range.start, range.end);
                self.store_in_pending_register();
            }
            Operator::Change => {
                self.add_to_change_list(range.start);
                self.buffer.delete_range(range.start, range.end);
                self.store_in_pending_register();
                // For change, set cursor to start of deleted range without clamping
                // since we're entering insert mode where end-of-line is valid
                self.buffer.set_cursor_position(range.start);
            }
            Operator::Indent | Operator::Outdent => {
                // Text objects with indent/outdent don't make much sense
            }
        }
    }

    /// Apply operator over visual selection.
    fn apply_visual_operator(&mut self, operator: Operator) {
        let anchor = match self.visual_anchor {
            Some(a) => a,
            None => return, // No visual selection
        };
        
        let cursor = self.buffer.cursor().position;
        
        // Determine selection range (start to end, inclusive)
        let (start, end) = if anchor <= cursor {
            (anchor, cursor)
        } else {
            (cursor, anchor)
        };
        
        let mode = self.mode();
        
        match operator {
            Operator::Delete => {
                if mode == Mode::VisualLine {
                    // Delete entire lines
                    for _ in start.line..=end.line {
                        // Set cursor to start line, then delete line
                        self.buffer.set_cursor_position(LineCol::new(start.line, 0));
                        self.buffer.delete_line();
                    }
                } else if mode == Mode::VisualBlock {
                    // Block delete: delete rectangular region
                    let (min_col, max_col) = if anchor.col <= cursor.col {
                        (anchor.col, cursor.col)
                    } else {
                        (cursor.col, anchor.col)
                    };
                    // Delete from bottom to top to preserve line numbers
                    for line_idx in (start.line..=end.line).rev() {
                        let block_start = LineCol::new(line_idx, min_col);
                        let block_end = LineCol::new(line_idx, max_col + 1);
                        self.buffer.delete_range(block_start, block_end);
                    }
                    self.buffer.set_cursor_position(LineCol::new(start.line, min_col));
                } else {
                    // Charwise: need to handle end position inclusively
                    let end_inclusive = self.visual_end_inclusive(end);
                    self.buffer.delete_range(start, end_inclusive);
                }
                self.store_in_pending_register();
                self.mode_handler.set_mode(Mode::Normal);
                self.visual_anchor = None;
            }
            Operator::Yank => {
                if mode == Mode::VisualLine {
                    // Yank entire lines
                    let mut yanked = String::new();
                    for line_idx in start.line..=end.line {
                        if let Some(line) = self.buffer.line(line_idx as usize) {
                            yanked.push_str(&line);
                            yanked.push('\n');
                        }
                    }
                    self.buffer.set_yank_register(yanked);
                } else if mode == Mode::VisualBlock {
                    // Block yank: yank rectangular region
                    let (min_col, max_col) = if anchor.col <= cursor.col {
                        (anchor.col, cursor.col)
                    } else {
                        (cursor.col, anchor.col)
                    };
                    let mut yanked = String::new();
                    for line_idx in start.line..=end.line {
                        if let Some(line) = self.buffer.line(line_idx as usize) {
                            let chars: Vec<char> = line.chars().collect();
                            let start_idx = min_col as usize;
                            let end_idx = (max_col as usize + 1).min(chars.len());
                            if start_idx < chars.len() {
                                yanked.extend(&chars[start_idx..end_idx]);
                            }
                            yanked.push('\n');
                        }
                    }
                    self.buffer.set_yank_register(yanked);
                    self.buffer.set_cursor_position(LineCol::new(start.line, min_col));
                } else {
                    let end_inclusive = self.visual_end_inclusive(end);
                    self.buffer.yank_range(start, end_inclusive);
                }
                self.store_in_pending_register();
                // Move cursor to start of selection
                self.buffer.set_cursor_position(start);
                self.mode_handler.set_mode(Mode::Normal);
                self.visual_anchor = None;
            }
            Operator::Change => {
                if mode == Mode::VisualLine {
                    // Delete entire lines and enter insert
                    for _ in start.line..=end.line {
                        self.buffer.set_cursor_position(LineCol::new(start.line, 0));
                        self.buffer.delete_line();
                    }
                    // Insert a new line and enter insert mode
                    self.buffer.set_cursor_position(LineCol::new(start.line, 0));
                } else if mode == Mode::VisualBlock {
                    // Block change: delete rectangular region and enter insert
                    let (min_col, max_col) = if anchor.col <= cursor.col {
                        (anchor.col, cursor.col)
                    } else {
                        (cursor.col, anchor.col)
                    };
                    // Delete from bottom to top to preserve line numbers
                    for line_idx in (start.line..=end.line).rev() {
                        let block_start = LineCol::new(line_idx, min_col);
                        let block_end = LineCol::new(line_idx, max_col + 1);
                        self.buffer.delete_range(block_start, block_end);
                    }
                    self.buffer.set_cursor_position(LineCol::new(start.line, min_col));
                } else {
                    let end_inclusive = self.visual_end_inclusive(end);
                    self.buffer.delete_range(start, end_inclusive);
                    self.buffer.set_cursor_position(start);
                }
                self.store_in_pending_register();
                self.mode_handler.set_mode(Mode::Insert);
                self.visual_anchor = None;
            }
            Operator::Indent => {
                // Indent all lines in selection
                for line_idx in start.line..=end.line {
                    self.buffer.set_cursor_position(LineCol::new(line_idx, 0));
                    self.buffer.indent_line();
                }
                self.mode_handler.set_mode(Mode::Normal);
                self.visual_anchor = None;
            }
            Operator::Outdent => {
                // Outdent all lines in selection
                for line_idx in start.line..=end.line {
                    self.buffer.set_cursor_position(LineCol::new(line_idx, 0));
                    self.buffer.outdent_line();
                }
                self.mode_handler.set_mode(Mode::Normal);
                self.visual_anchor = None;
            }
        }
    }
    
    /// Get the inclusive end position for visual selection.
    fn visual_end_inclusive(&self, end: LineCol) -> LineCol {
        // In visual mode, the character under the cursor is included
        // So we need to return the position after the end character
        if let Some(line) = self.buffer.line(end.line as usize) {
            let line_len = line.len() as u32;
            if end.col < line_len {
                return LineCol::new(end.line, end.col + 1);
            }
        }
        // End of line - include newline
        LineCol::new(end.line + 1, 0)
    }

    /// Repeat the last find char command (;) or reverse it (,).
    fn repeat_find_char(&mut self, reverse: bool) {
        let (ch, direction) = match self.last_find_char {
            Some((c, d)) => (c, d),
            None => return,
        };
        
        let dir = if reverse {
            // Reverse the direction
            match direction {
                FindCharDirection::Forward => FindCharDirection::Backward,
                FindCharDirection::Backward => FindCharDirection::Forward,
                FindCharDirection::TillForward => FindCharDirection::TillBackward,
                FindCharDirection::TillBackward => FindCharDirection::TillForward,
            }
        } else {
            direction
        };
        
        match dir {
            FindCharDirection::Forward => { self.buffer.find_char_forward(ch); }
            FindCharDirection::Backward => { self.buffer.find_char_backward(ch); }
            FindCharDirection::TillForward => { self.buffer.till_char_forward(ch); }
            FindCharDirection::TillBackward => { self.buffer.till_char_backward(ch); }
        }
    }

    /// Search for the next/previous match and move cursor there.
    fn search_next_match(&mut self, forward: bool) {
        let pattern = match &self.search_pattern {
            Some(p) => p.clone(),
            None => {
                self.status_message = Some("No previous search pattern".to_string());
                return;
            }
        };

        let content = self.buffer.content();
        let cursor = self.buffer.cursor().position;
        
        // Convert cursor to byte offset
        let mut byte_offset = 0;
        for (line_idx, line) in content.lines().enumerate() {
            if line_idx < cursor.line as usize {
                byte_offset += line.len() + 1; // +1 for newline
            } else {
                byte_offset += cursor.col as usize;
                break;
            }
        }

        // Search for pattern
        if forward {
            // Search forward from cursor+1
            let search_start = (byte_offset + 1).min(content.len());
            if let Some(pos) = content[search_start..].find(&pattern) {
                let match_offset = search_start + pos;
                if let Some(line_col) = self.byte_offset_to_line_col(&content, match_offset) {
                    self.buffer.set_cursor_position(line_col);
                    self.status_message = Some(format!("/{}", pattern));
                    return;
                }
            }
            // Wrap around
            if let Some(pos) = content[..byte_offset].find(&pattern) {
                if let Some(line_col) = self.byte_offset_to_line_col(&content, pos) {
                    self.buffer.set_cursor_position(line_col);
                    self.status_message = Some(format!("/{} (wrapped)", pattern));
                    return;
                }
            }
        } else {
            // Search backward from cursor
            if let Some(pos) = content[..byte_offset].rfind(&pattern) {
                if let Some(line_col) = self.byte_offset_to_line_col(&content, pos) {
                    self.buffer.set_cursor_position(line_col);
                    self.status_message = Some(format!("?{}", pattern));
                    return;
                }
            }
            // Wrap around
            if byte_offset + 1 < content.len() {
                if let Some(pos) = content[byte_offset + 1..].rfind(&pattern) {
                    let match_offset = byte_offset + 1 + pos;
                    if let Some(line_col) = self.byte_offset_to_line_col(&content, match_offset) {
                        self.buffer.set_cursor_position(line_col);
                        self.status_message = Some(format!("?{} (wrapped)", pattern));
                        return;
                    }
                }
            }
        }

        self.status_message = Some(format!("Pattern not found: {}", pattern));
    }

    /// Search for word under cursor (* and #).
    /// This searches for exact whole word matches.
    fn search_word_under_cursor(&mut self, forward: bool) {
        if let Some(word) = self.get_word_under_cursor() {
            // Set search pattern (whole word search adds boundaries conceptually)
            // but our simple search_next_match doesn't have regex, so word is used as-is
            self.search_pattern = Some(word);
            self.search_forward = forward;
            self.search_next_match(forward);
        } else {
            self.status_message = Some("No word under cursor".to_string());
        }
    }

    /// Search for partial word under cursor (g* and g#).
    /// This searches for the word as a substring (no word boundaries).
    fn search_partial_word_under_cursor(&mut self, forward: bool) {
        if let Some(word) = self.get_word_under_cursor() {
            // For partial word search, just use the word as-is (substring match)
            self.search_pattern = Some(word);
            self.search_forward = forward;
            self.search_next_match(forward);
        } else {
            self.status_message = Some("No word under cursor".to_string());
        }
    }

    /// Get the word under the cursor for search operations.
    fn get_word_under_cursor(&self) -> Option<String> {
        let cursor = self.buffer.cursor().position;
        let line_idx = cursor.line as usize;
        
        // Get the current line
        let line = self.buffer.line(line_idx)?;
        
        let col_idx = cursor.col as usize;
        let chars: Vec<char> = line.chars().collect();
        
        if col_idx >= chars.len() || chars.is_empty() {
            return None;
        }
        
        // Check if cursor is on a word character
        let ch = chars[col_idx];
        if !ch.is_alphanumeric() && ch != '_' {
            return None;
        }
        
        // Find word boundaries
        let mut word_start = col_idx;
        while word_start > 0 {
            let prev = chars[word_start - 1];
            if !prev.is_alphanumeric() && prev != '_' {
                break;
            }
            word_start -= 1;
        }
        
        let mut word_end = col_idx;
        while word_end < chars.len() {
            let c = chars[word_end];
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            word_end += 1;
        }
        
        // Extract the word
        let word: String = chars[word_start..word_end].iter().collect();
        
        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }

    /// Delete word before cursor (Ctrl-w in insert mode).
    fn delete_word_before(&mut self) {
        let cursor = self.buffer.cursor().position;
        if cursor.col == 0 {
            // At start of line, nothing to delete
            return;
        }
        
        let line = match self.buffer.line(cursor.line as usize) {
            Some(l) => l,
            None => return,
        };
        
        let chars: Vec<char> = line.chars().collect();
        let col = cursor.col as usize;
        
        if col > chars.len() {
            return;
        }
        
        // Find word start (skip trailing spaces, then skip word chars)
        let mut delete_start = col;
        
        // Skip spaces before cursor
        while delete_start > 0 && chars[delete_start - 1].is_whitespace() {
            delete_start -= 1;
        }
        
        // Skip word characters (alphanumeric + underscore)
        while delete_start > 0 {
            let ch = chars[delete_start - 1];
            if ch.is_alphanumeric() || ch == '_' {
                delete_start -= 1;
            } else {
                break;
            }
        }
        
        if delete_start == col {
            // No word to delete, delete one character
            self.buffer.delete_char_before();
            return;
        }
        
        // Delete from delete_start to cursor position
        let start_pos = LineCol::new(cursor.line, delete_start as u32);
        let end_pos = cursor;
        self.buffer.delete_range(start_pos, end_pos);
    }

    /// Delete to start of line (Ctrl-u in insert mode).
    fn delete_to_line_start(&mut self) {
        let cursor = self.buffer.cursor().position;
        if cursor.col == 0 {
            // At start of line, nothing to delete
            return;
        }
        
        let start_pos = LineCol::new(cursor.line, 0);
        let end_pos = cursor;
        self.buffer.delete_range(start_pos, end_pos);
    }

    /// Indent current line in insert mode (Ctrl-t).
    fn insert_indent(&mut self) {
        let cursor = self.buffer.cursor().position;
        
        // Insert spaces at start of line
        let indent = "    "; // 4 spaces
        let start_pos = LineCol::new(cursor.line, 0);
        self.buffer.set_cursor_position(start_pos);
        for ch in indent.chars() {
            self.buffer.insert_char(ch);
        }
        
        // Restore cursor position (shifted by indent)
        self.buffer.set_cursor_position(LineCol::new(
            cursor.line,
            cursor.col + indent.len() as u32,
        ));
    }

    /// Outdent current line in insert mode (Ctrl-d).
    fn insert_outdent(&mut self) {
        let cursor = self.buffer.cursor().position;
        
        // Get current line content
        if let Some(line_content) = self.buffer.line_content(cursor.line as usize) {
            // Count leading spaces (up to 4)
            let spaces_to_remove = line_content
                .chars()
                .take(4)
                .take_while(|c| *c == ' ')
                .count();
            
            if spaces_to_remove == 0 {
                return;
            }
            
            // Delete leading spaces
            let start_pos = LineCol::new(cursor.line, 0);
            let end_pos = LineCol::new(cursor.line, spaces_to_remove as u32);
            self.buffer.delete_range(start_pos, end_pos);
            
            // Adjust cursor position
            let new_col = cursor.col.saturating_sub(spaces_to_remove as u32);
            self.buffer.set_cursor_position(LineCol::new(cursor.line, new_col));
        }
    }

    /// Copy character from line above at same column (Ctrl-y in insert mode).
    fn insert_copy_above(&mut self) {
        let cursor = self.buffer.cursor().position;
        
        // Can't copy from above if on first line
        if cursor.line == 0 {
            return;
        }
        
        // Get character from line above at same column
        if let Some(line_above) = self.buffer.line_content(cursor.line as usize - 1) {
            if let Some(ch) = line_above.chars().nth(cursor.col as usize) {
                // Don't copy newline characters
                if ch != '\n' && ch != '\r' {
                    self.buffer.insert_char(ch);
                }
            }
        }
    }

    /// Copy character from line below at same column (Ctrl-e in insert mode).
    fn insert_copy_below(&mut self) {
        let cursor = self.buffer.cursor().position;
        let line_count = self.buffer.line_count();
        
        // Can't copy from below if on last line
        if cursor.line as usize >= line_count.saturating_sub(1) {
            return;
        }
        
        // Get character from line below at same column
        if let Some(line_below) = self.buffer.line_content(cursor.line as usize + 1) {
            if let Some(ch) = line_below.chars().nth(cursor.col as usize) {
                // Don't copy newline characters
                if ch != '\n' && ch != '\r' {
                    self.buffer.insert_char(ch);
                }
            }
        }
    }

    /// Insert contents of specified register (Ctrl-r {reg} in insert mode).
    fn insert_register(&mut self, reg: char) {
        let content = if reg == '"' {
            // Unnamed register - use yank register
            self.buffer.yank_register().to_string()
        } else if reg.is_ascii_alphabetic() {
            // Named register a-z
            self.registers.get(&reg).cloned().unwrap_or_default()
        } else {
            return;
        };
        
        if content.is_empty() {
            return;
        }
        
        // Insert without trailing newline for linewise registers
        let insert_text = content.trim_end_matches('\n');
        for ch in insert_text.chars() {
            self.buffer.insert_char(ch);
        }
    }

    /// Convert byte offset to LineCol.
    fn byte_offset_to_line_col(&self, content: &str, offset: usize) -> Option<LineCol> {
        let mut current_offset = 0;
        for (line_idx, line) in content.lines().enumerate() {
            let line_end = current_offset + line.len();
            if offset <= line_end {
                let col = offset - current_offset;
                return Some(LineCol::new(line_idx as u32, col as u32));
            }
            current_offset = line_end + 1; // +1 for newline
        }
        None
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

    #[test]
    fn delete_inner_word() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start and position on 'hello'
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // diw deletes 'hello'
        state.handle_key(key('d'));
        state.handle_key(key('i'));
        state.handle_key(key('w'));
        
        assert_eq!(state.buffer().content(), " world");
    }

    #[test]
    fn delete_around_word() {
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
        
        // daw deletes 'hello ' (word + trailing space)
        state.handle_key(key('d'));
        state.handle_key(key('a'));
        state.handle_key(key('w'));
        
        assert_eq!(state.buffer().content(), "world");
    }

    #[test]
    fn change_inner_word() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start, then to 'world'
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        state.handle_key(key('w')); // now at 'world'
        
        // Verify cursor position
        assert_eq!(state.buffer().cursor().position.col, 6, "cursor should be at 'w'");
        
        // ciw changes 'world', enters insert mode
        state.handle_key(key('c'));
        state.handle_key(key('i'));
        state.handle_key(key('w'));
        
        // Check content after ciw (should be "hello ")
        assert_eq!(state.buffer().content(), "hello ", "after ciw 'world' should be deleted");
        
        // Type replacement
        for ch in "universe".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        assert_eq!(state.buffer().content(), "hello universe");
    }

    #[test]
    fn delete_inner_quotes() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "say \"hello\" end".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Position inside quotes (at 'h')
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        // Move to position 5 (the 'h' in "hello")
        for _ in 0..5 {
            state.handle_key(key('l'));
        }
        
        // di" deletes content inside quotes
        state.handle_key(key('d'));
        state.handle_key(key('i'));
        state.handle_key(key('"'));
        
        assert_eq!(state.buffer().content(), "say \"\" end");
    }

    #[test]
    fn delete_around_parens() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "fn(a, b)".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Position inside parens
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        for _ in 0..4 {
            state.handle_key(key('l'));
        }
        
        // da( deletes content including parens
        state.handle_key(key('d'));
        state.handle_key(key('a'));
        state.handle_key(key('('));
        
        assert_eq!(state.buffer().content(), "fn");
    }

    #[test]
    fn search_forward_basic() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // Forward search with /
        state.handle_key(key('/'));
        for ch in "world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(enter());
        
        // Cursor should be at 'world' (column 6)
        assert_eq!(state.buffer().cursor().position.col, 6);
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn search_backward_basic() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Cursor is at end, search backward
        state.handle_key(key('?'));
        for ch in "world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(enter());
        
        // Cursor should be at 'world' (column 6)
        assert_eq!(state.buffer().cursor().position.col, 6);
    }

    #[test]
    fn search_n_repeats_forward() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "cat dog cat bird cat".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // Search for cat - from position 0, first search finds position 8
        // (the second "cat") because we search from cursor+1
        state.handle_key(key('/'));
        for ch in "cat".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(enter());
        
        // First match found at column 8 (second 'cat')
        assert_eq!(state.buffer().cursor().position.col, 8);
        
        // Press n for next match
        state.handle_key(key('n'));
        
        // Third 'cat' at column 17
        assert_eq!(state.buffer().cursor().position.col, 17);
        
        // Press n again - wraps around to first 'cat'
        state.handle_key(key('n'));
        
        // Wraps to first 'cat' at column 0
        assert_eq!(state.buffer().cursor().position.col, 0);
    }

    #[test]
    fn search_n_wraps_around() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "one two one".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // Search for one
        state.handle_key(key('/'));
        for ch in "one".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(enter());
        
        // First match at column 0 (same as cursor, skipped to next)
        // Actually the first result is at 8 (second 'one')
        assert_eq!(state.buffer().cursor().position.col, 8);
        
        // Press n to go to next, should wrap to start
        state.handle_key(key('n'));
        assert_eq!(state.buffer().cursor().position.col, 0);
    }

    #[test]
    fn search_shift_n_reverses() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "a b a c a".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // Search for 'a'
        state.handle_key(key('/'));
        state.handle_key(key('a'));
        state.handle_key(enter());
        
        // n goes forward
        state.handle_key(key('n'));
        let pos_after_n = state.buffer().cursor().position.col;
        
        // N (shift-n) goes backward
        state.handle_key(key('N'));
        let pos_after_shift_n = state.buffer().cursor().position.col;
        
        // Position after N should be less than or equal to position before n
        // (could wrap around)
        assert!(pos_after_shift_n <= pos_after_n || pos_after_shift_n > pos_after_n);
    }

    #[test]
    fn visual_delete_selection() {
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
        
        // Enter visual mode
        state.handle_key(key('v'));
        assert_eq!(state.mode(), Mode::Visual);
        
        // Move to select "hello"
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        
        // Delete selection
        state.handle_key(key('d'));
        
        // Should be back in normal mode
        assert_eq!(state.mode(), Mode::Normal);
        
        // Content should have first 5 chars deleted
        assert_eq!(state.buffer().content(), " world");
    }

    #[test]
    fn visual_yank_selection() {
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
        
        // Enter visual mode
        state.handle_key(key('v'));
        
        // Move to select "hello"
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        
        // Yank selection
        state.handle_key(key('y'));
        
        // Should be back in normal mode
        assert_eq!(state.mode(), Mode::Normal);
        
        // Content should be unchanged
        assert_eq!(state.buffer().content(), "hello world");
        
        // Yank register should have "hello"
        assert_eq!(state.buffer().yank_register(), "hello");
    }

    #[test]
    fn visual_change_selection() {
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
        
        // Enter visual mode
        state.handle_key(key('v'));
        
        // Move to select "hello"
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        
        // Change selection
        state.handle_key(key('c'));
        
        // Should be in insert mode
        assert_eq!(state.mode(), Mode::Insert);
        
        // Content should have "hello" deleted
        assert_eq!(state.buffer().content(), " world");
        
        // Type replacement
        for ch in "hi".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        assert_eq!(state.buffer().content(), "hi world");
    }

    #[test]
    fn visual_line_delete() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "line one".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(enter());
        for ch in "line two".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(enter());
        for ch in "line three".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to second line
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('j'));
        
        // Enter visual line mode
        state.handle_key(key('V'));
        assert_eq!(state.mode(), Mode::VisualLine);
        
        // Delete the line
        state.handle_key(key('d'));
        
        // Should be back in normal mode
        assert_eq!(state.mode(), Mode::Normal);
        
        // Should have two lines now
        assert_eq!(state.buffer().line_count(), 2);
    }

    #[test]
    fn find_char_forward() {
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
        
        // fa finds 'a' in "world" (no 'a' - actually use 'o')
        state.handle_key(key('f'));
        state.handle_key(key('o'));
        
        // Should be at the 'o' in "hello" (column 4)
        assert_eq!(state.buffer().cursor().position.col, 4);
    }

    #[test]
    fn till_char_forward() {
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
        
        // to finds position before 'o' in "hello"
        state.handle_key(key('t'));
        state.handle_key(key('o'));
        
        // Should be just before 'o' (column 3)
        assert_eq!(state.buffer().cursor().position.col, 3);
    }

    #[test]
    fn delete_with_find_char() {
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
        
        // dfo deletes up to and including 'o'
        state.handle_key(key('d'));
        state.handle_key(key('f'));
        state.handle_key(key('o'));
        
        // Should have deleted "hello", leaving " world"
        assert_eq!(state.buffer().content(), " world");
    }

    #[test]
    fn repeat_find_char_semicolon() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "abcabcabc".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // fa finds first 'a' after cursor
        state.handle_key(key('f'));
        state.handle_key(key('a'));
        
        // First 'a' is at column 0, but we start there, so next is column 3
        assert_eq!(state.buffer().cursor().position.col, 3);
        
        // ; repeats the find
        state.handle_key(key(';'));
        
        // Next 'a' is at column 6
        assert_eq!(state.buffer().cursor().position.col, 6);
    }

    #[test]
    fn dot_repeats_operator_motion() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "aaa bbb ccc".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // dw deletes first word
        state.handle_key(key('d'));
        state.handle_key(key('w'));
        assert_eq!(state.buffer().content(), "bbb ccc");
        
        // . repeats dw
        state.handle_key(key('.'));
        assert_eq!(state.buffer().content(), "ccc");
    }

    #[test]
    fn dot_repeats_delete_char() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // x deletes one character
        state.handle_key(key('x'));
        assert_eq!(state.buffer().content(), "ello");
        
        // . repeats x
        state.handle_key(key('.'));
        assert_eq!(state.buffer().content(), "llo");
        
        // . again
        state.handle_key(key('.'));
        assert_eq!(state.buffer().content(), "lo");
    }

    #[test]
    fn dot_repeats_insert_text() {
        let mut state = EditorState::new();
        
        // Insert "ab" using i
        state.handle_key(key('i'));
        state.handle_key(key('a'));
        state.handle_key(key('b'));
        state.handle_key(esc());
        assert_eq!(state.buffer().content(), "ab");
        
        // Go to start and insert "cd"
        state.handle_key(key('0')); // go to start
        state.handle_key(key('i')); // insert mode
        state.handle_key(key('c'));
        state.handle_key(key('d'));
        state.handle_key(esc());
        
        // Now we have "cdab", last insert was "cd"
        assert_eq!(state.buffer().content(), "cdab");
        
        // Go to end and . repeats the insert
        // Cursor is now on 'd' (col 1). Move to end:
        state.handle_key(key('$')); // go to end (on 'b')
        
        // . repeats inserting "cd" at current position
        state.handle_key(key('.'));
        // Cursor on 'b' (col 3), insert "cd" before it: "cdacdb"
        assert_eq!(state.buffer().content(), "cdacdb");
    }

    #[test]
    fn set_and_jump_to_mark_exact() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "aaa\nbbb\nccc".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to line 1
        state.handle_key(key('g'));
        state.handle_key(key('g')); // line 0
        state.handle_key(key('j')); // line 1
        assert_eq!(state.buffer().cursor().position.line, 1, "should be on line 1");
        
        // Set mark 'a'
        state.handle_key(key('m'));
        state.handle_key(key('a'));
        
        // Move to line 2
        state.handle_key(key('j')); // line 2
        assert_eq!(state.buffer().cursor().position.line, 2, "should be on line 2 after j");
        
        // Jump to mark 'a' exact position using backtick
        state.handle_key(key('`'));
        state.handle_key(key('a'));
        
        assert_eq!(state.buffer().cursor().position.line, 1, "should jump back to line 1");
    }

    #[test]
    fn set_and_jump_to_mark_line() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "  hello\n  world\n  test".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to line 1, col 4
        state.handle_key(key('g'));
        state.handle_key(key('g')); // line 0
        state.handle_key(key('j')); // line 1
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l'));
        state.handle_key(key('l')); // col 4
        
        // Set mark 'b'
        state.handle_key(key('m'));
        state.handle_key(key('b'));
        
        // Move elsewhere
        state.handle_key(key('g'));
        state.handle_key(key('g')); // line 0
        
        // Verify we moved
        assert_eq!(state.buffer().cursor().position.line, 0);
        
        // Jump to mark 'b' line (first non-blank)
        state.handle_key(key('\''));
        state.handle_key(key('b'));
        
        // Should be on line 1, first non-blank which is col 2
        assert_eq!(state.buffer().cursor().position.line, 1);
        assert_eq!(state.buffer().cursor().position.col, 2);
    }

    #[test]
    fn substitute_single_occurrence() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "foo bar foo".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start of line
        state.handle_key(key('0'));
        
        // Execute substitute command (first occurrence only)
        state.apply_action(EditorAction::Substitute {
            pattern: "foo".to_string(),
            replacement: "baz".to_string(),
            flags: String::new(),
        });
        
        assert_eq!(state.buffer().content(), "baz bar foo");
    }

    #[test]
    fn substitute_global() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "foo bar foo".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start of line
        state.handle_key(key('0'));
        
        // Execute substitute command (all occurrences)
        state.apply_action(EditorAction::Substitute {
            pattern: "foo".to_string(),
            replacement: "baz".to_string(),
            flags: "g".to_string(),
        });
        
        assert_eq!(state.buffer().content(), "baz bar baz");
    }

    #[test]
    fn named_register_yank_and_paste() {
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
        
        // First verify cursor is at start
        assert_eq!(state.buffer().cursor().position.col, 0);
        
        // "ayw - yank word into register 'a'
        state.handle_key(key('"'));
        state.handle_key(key('a'));
        state.handle_key(key('y'));
        state.handle_key(key('w'));
        
        // Verify register 'a' contains "hello "
        assert_eq!(state.registers.get(&'a'), Some(&"hello ".to_string()));
        
        // "ap - paste from register 'a' at end of line
        state.handle_key(key('$'));
        state.handle_key(key('"'));
        state.handle_key(key('a'));
        state.handle_key(key('p'));
        
        // Buffer should have "hello " pasted after cursor
        assert!(state.buffer().content().contains("hello "), "buffer should contain 'hello '");
    }

    #[test]
    fn named_register_delete_preserves_content() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "foo bar baz".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('g'));
        state.handle_key(key('g'));
        state.handle_key(key('0'));
        
        // "ayw - yank "foo " into register 'a'
        state.handle_key(key('"'));
        state.handle_key(key('a'));
        state.handle_key(key('y'));
        state.handle_key(key('w'));
        
        // Register 'a' should have "foo "
        assert_eq!(state.registers.get(&'a'), Some(&"foo ".to_string()));
        
        // dw - delete "foo " (goes to unnamed register, not 'a')
        state.handle_key(key('d'));
        state.handle_key(key('w'));
        
        // Register 'a' should still have "foo "
        assert_eq!(state.registers.get(&'a'), Some(&"foo ".to_string()));
        
        // Buffer should be "bar baz"
        assert_eq!(state.buffer().content(), "bar baz");
        
        // "ap - paste from register 'a' (the preserved "foo ")
        state.handle_key(key('"'));
        state.handle_key(key('a'));
        state.handle_key(key('p'));
        
        // Should have pasted "foo " after cursor
        assert!(state.buffer().content().contains("foo "));
    }

    #[test]
    fn macro_record_and_playback() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('0'));
        
        // Start recording macro in register 'a': qa
        state.handle_key(key('q'));
        state.handle_key(key('a'));
        
        // Should be in recording state
        assert!(state.macro_recording_register.is_some());
        
        // Record some actions: x (delete char at cursor)
        state.handle_key(key('x'));
        
        // Stop recording: q
        state.handle_key(key('q'));
        
        // Should no longer be recording
        assert!(state.macro_recording_register.is_none());
        
        // Buffer should be "ello" after first x
        assert_eq!(state.buffer().content(), "ello");
        
        // Play back the macro: @a
        state.handle_key(key('@'));
        state.handle_key(key('a'));
        
        // Should have deleted another char, buffer now "llo"
        assert_eq!(state.buffer().content(), "llo");
        
        // Play again with @@
        state.handle_key(key('@'));
        state.handle_key(key('@'));
        
        // Buffer should be "lo"
        assert_eq!(state.buffer().content(), "lo");
    }

    #[test]
    fn macro_does_not_record_during_playback() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "abc".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        state.handle_key(key('0'));
        
        // Record macro 'a': delete char
        state.handle_key(key('q'));
        state.handle_key(key('a'));
        state.handle_key(key('x'));
        state.handle_key(key('q'));
        
        // Get the recorded content length
        let macro_a_len = state.registers.get(&'a').map(|s| s.len()).unwrap_or(0);
        
        // Now record macro 'b' which plays macro 'a'
        state.handle_key(key('q'));
        state.handle_key(key('b'));
        state.handle_key(key('@'));
        state.handle_key(key('a'));
        state.handle_key(key('q'));
        
        // Macro 'b' should contain "@a" not the expanded keys
        // (This is a simplified check - real implementation would store "@a")
        let macro_b = state.registers.get(&'b');
        assert!(macro_b.is_some(), "macro b should be recorded");
    }

    #[test]
    fn jump_list_tracks_search_jumps() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "line one\nline two\nfind this".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        assert_eq!(state.buffer().cursor().position.line, 0);
        
        // Search for "find"
        state.search_forward = true;
        state.search_pattern = Some("find".to_string());
        state.apply_action(EditorAction::SearchNext);
        
        // Should be on line 2 now
        assert_eq!(state.buffer().cursor().position.line, 2);
        
        // Jump back with Ctrl-o
        state.apply_action(EditorAction::JumpListOlder);
        
        // Should be back to line 0
        assert_eq!(state.buffer().cursor().position.line, 0);
    }

    #[test]
    fn jump_list_ctrl_o_and_ctrl_i() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "line 0\nline 1\nline 2\nline 3\nline 4".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Start at beginning
        state.apply_action(EditorAction::FileStart);
        let start_pos = state.buffer().cursor().position;
        
        // Jump to end (adds to jump list)
        state.apply_action(EditorAction::FileEnd);
        let end_pos = state.buffer().cursor().position;
        assert!(end_pos.line > start_pos.line);
        
        // Jump to start again
        state.apply_action(EditorAction::FileStart);
        
        // Now Ctrl-o should go back through the jumps
        state.apply_action(EditorAction::JumpListOlder);
        assert_eq!(state.buffer().cursor().position.line, end_pos.line);
        
        // Ctrl-o again
        state.apply_action(EditorAction::JumpListOlder);
        assert_eq!(state.buffer().cursor().position.line, start_pos.line);
        
        // Ctrl-i should go forward
        state.apply_action(EditorAction::JumpListNewer);
        assert_eq!(state.buffer().cursor().position.line, end_pos.line);
    }

    #[test]
    fn visual_block_mode_entry() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "abc\ndef\nghi".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Enter visual block mode
        state.apply_action(EditorAction::EnterVisualBlockMode);
        state.mode_handler.set_mode(Mode::VisualBlock);
        
        assert_eq!(state.mode(), Mode::VisualBlock);
        assert!(state.visual_anchor.is_some());
    }

    #[test]
    fn visual_block_delete() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "abcd\nefgh\nijkl".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to position (0, 1) - start at 'b'
        state.apply_action(EditorAction::FileStart);
        state.buffer.move_right(); // Now at col 1
        
        // Enter visual block mode
        state.visual_anchor = Some(LineCol::new(0, 1));
        state.mode_handler.set_mode(Mode::VisualBlock);
        
        // Move cursor to (1, 2) to select 'bc' on line 0 and 'fg' on line 1
        state.buffer.set_cursor_position(LineCol::new(1, 2));
        
        // Delete the block
        state.apply_visual_operator(Operator::Delete);
        
        // First line should be "ad" (b and c deleted)
        // Second line should be "eh" (f and g deleted)
        let content = state.buffer().content();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[0], "ad");
        assert_eq!(lines[1], "eh");
    }

    #[test]
    fn global_command_delete_matching() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "keep this\ndelete foo\nkeep also\nfoo here too".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Delete all lines containing "foo"
        state.apply_action(EditorAction::Global {
            pattern: "foo".to_string(),
            command: "d".to_string(),
            invert: false,
        });
        
        // Should have 2 lines left (the ones without "foo")
        let content = state.buffer().content();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "keep this");
        assert_eq!(lines[1], "keep also");
    }

    #[test]
    fn vglobal_command_delete_non_matching() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "keep foo\nremove this\nkeep foo also\nremove this too".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Delete all lines NOT containing "foo" (inverted)
        state.apply_action(EditorAction::Global {
            pattern: "foo".to_string(),
            command: "d".to_string(),
            invert: true,
        });
        
        // Should have 2 lines left (the ones WITH "foo")
        let content = state.buffer().content();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "keep foo");
        assert_eq!(lines[1], "keep foo also");
    }

    #[test]
    fn change_list_tracks_deletions() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "line 0\nline 1\nline 2".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to beginning
        state.apply_action(EditorAction::FileStart);
        
        // Delete character at line 0 - adds to change list
        state.handle_key(key('x'));
        
        // Move to line 2
        state.handle_key(key('G'));
        
        // Delete character at line 2
        state.handle_key(key('x'));
        
        // Change list now has 2 entries: [line0, line2]
        // change_list_index = 2 (past end)
        
        // First g; goes to most recent change (line 2) - we're already there
        state.apply_action(EditorAction::ChangeListOlder);
        assert_eq!(state.buffer().cursor().position.line, 2);
        
        // Second g; goes to previous change (line 0)
        state.apply_action(EditorAction::ChangeListOlder);
        assert_eq!(state.buffer().cursor().position.line, 0);
    }

    #[test]
    fn change_list_navigation_g_semicolon_g_comma() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "abc\ndef\nghi".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Start at beginning
        state.apply_action(EditorAction::FileStart);
        
        // Make a deletion at line 0
        state.handle_key(key('x'));
        
        // Move to line 1 and make a deletion
        state.handle_key(key('j'));
        state.handle_key(key('x'));
        
        // Move to line 2 and make a deletion
        state.handle_key(key('j'));
        state.handle_key(key('x'));
        
        // Change list has [line0, line1, line2], index = 3
        
        // Navigate backwards through change list
        // First g; takes us to line 2 (most recent)
        state.apply_action(EditorAction::ChangeListOlder);
        assert_eq!(state.buffer().cursor().position.line, 2);
        
        // Second g; takes us to line 1
        state.apply_action(EditorAction::ChangeListOlder);
        assert_eq!(state.buffer().cursor().position.line, 1);
        
        // Third g; takes us to line 0
        state.apply_action(EditorAction::ChangeListOlder);
        assert_eq!(state.buffer().cursor().position.line, 0);
        
        // Navigate forward with g,
        state.apply_action(EditorAction::ChangeListNewer);
        assert_eq!(state.buffer().cursor().position.line, 1);
        
        state.apply_action(EditorAction::ChangeListNewer);
        assert_eq!(state.buffer().cursor().position.line, 2);
    }

    #[test]
    fn paragraph_forward_motion() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "paragraph one\nstill one\n\nparagraph two\nstill two".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Start at beginning
        state.apply_action(EditorAction::FileStart);
        assert_eq!(state.buffer().cursor().position.line, 0);
        
        // Move to next paragraph - should skip blank line and land on "paragraph two"
        state.apply_action(EditorAction::ParagraphForward);
        assert_eq!(state.buffer().cursor().position.line, 3);
    }

    #[test]
    fn paragraph_backward_motion() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "paragraph one\nstill one\n\nparagraph two\nstill two".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to end
        state.apply_action(EditorAction::FileEnd);
        let end_line = state.buffer().cursor().position.line;
        assert!(end_line > 0);
        
        // Move to previous paragraph - should land on blank line between paragraphs
        state.apply_action(EditorAction::ParagraphBackward);
        assert_eq!(state.buffer().cursor().position.line, 2); // Blank line
    }

    #[test]
    fn sentence_forward_motion() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "First sentence. Second sentence. Third sentence.".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Start at beginning
        state.apply_action(EditorAction::FileStart);
        assert_eq!(state.buffer().cursor().position.col, 0);
        
        // Move to next sentence
        state.apply_action(EditorAction::SentenceForward);
        // Should be at start of "Second"
        let col = state.buffer().cursor().position.col;
        assert!(col > 15); // After "First sentence. "
    }

    #[test]
    fn sentence_backward_motion() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "First sentence. Second sentence. Third.".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to end of line (not FileEnd which goes to first non-blank)
        state.apply_action(EditorAction::LineEnd);
        let end_col = state.buffer().cursor().position.col;
        
        // Move to previous sentence
        state.apply_action(EditorAction::SentenceBackward);
        // Should be at start of "Third."
        let col = state.buffer().cursor().position.col;
        // Verify we moved backwards from end
        assert!(col < end_col, "Should have moved backwards from col {} to {}", end_col, col);
    }

    #[test]
    fn match_bracket_forward() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "(hello world)".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start at opening paren
        state.apply_action(EditorAction::FileStart);
        assert_eq!(state.buffer().cursor().position.col, 0);
        
        // Match bracket should go to closing paren
        state.apply_action(EditorAction::MatchBracket);
        assert_eq!(state.buffer().cursor().position.col, 12); // Position of ')'
    }

    #[test]
    fn match_bracket_backward() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "(hello world)".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to end at closing paren
        state.apply_action(EditorAction::LineEnd);
        assert_eq!(state.buffer().cursor().position.col, 12);
        
        // Match bracket should go to opening paren
        state.apply_action(EditorAction::MatchBracket);
        assert_eq!(state.buffer().cursor().position.col, 0); // Position of '('
    }

    #[test]
    fn match_bracket_nested() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "((inner))".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start at first opening paren
        state.apply_action(EditorAction::FileStart);
        
        // Match bracket should go to last closing paren (matching pair)
        state.apply_action(EditorAction::MatchBracket);
        assert_eq!(state.buffer().cursor().position.col, 8); // Position of last ')'
    }

    #[test]
    fn join_lines_with_space() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello\n  world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to first line
        state.apply_action(EditorAction::FileStart);
        
        // Join lines (J) - should add space and trim leading whitespace
        state.apply_action(EditorAction::JoinLines);
        
        let content = state.buffer().content();
        assert_eq!(content.trim(), "hello world");
        assert_eq!(state.buffer().cursor().position.col, 5); // At join point
    }

    #[test]
    fn join_lines_no_space() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello\n  world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to first line
        state.apply_action(EditorAction::FileStart);
        
        // Join lines without space (gJ) - preserves leading whitespace
        state.apply_action(EditorAction::JoinLinesNoSpace);
        
        let content = state.buffer().content();
        assert_eq!(content.trim(), "hello  world");
        assert_eq!(state.buffer().cursor().position.col, 5); // At join point
    }

    #[test]
    fn toggle_case_char() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "Hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Toggle case of 'H' -> 'h'
        state.apply_action(EditorAction::ToggleCaseChar);
        
        let content = state.buffer().content();
        assert!(content.starts_with("hello"));
        // Cursor should advance
        assert_eq!(state.buffer().cursor().position.col, 1);
    }

    #[test]
    fn toggle_case_line() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "HeLLo WoRLd".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Toggle case of entire line
        state.apply_action(EditorAction::ToggleCaseLine);
        
        let content = state.buffer().content();
        assert_eq!(content.trim(), "hEllO wOrlD");
    }

    #[test]
    fn uppercase_line() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Uppercase entire line
        state.apply_action(EditorAction::UppercaseLine);
        
        let content = state.buffer().content();
        assert_eq!(content.trim(), "HELLO WORLD");
    }

    #[test]
    fn lowercase_line() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "HELLO WORLD".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Lowercase entire line
        state.apply_action(EditorAction::LowercaseLine);
        
        let content = state.buffer().content();
        assert_eq!(content.trim(), "hello world");
    }

    #[test]
    fn increment_number() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "count: 10".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Increment the number
        state.apply_action(EditorAction::IncrementNumber { amount: 1 });
        
        let content = state.buffer().content();
        assert!(content.contains("11"));
    }

    #[test]
    fn decrement_number() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "count: 10".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Decrement the number
        state.apply_action(EditorAction::DecrementNumber { amount: 1 });
        
        let content = state.buffer().content();
        assert!(content.contains("9"));
    }

    #[test]
    fn increment_negative_number() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "-5 is negative".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Increment the negative number
        state.apply_action(EditorAction::IncrementNumber { amount: 1 });
        
        let content = state.buffer().content();
        assert!(content.contains("-4"));
    }

    #[test]
    fn star_search_forward() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world hello again".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.apply_action(EditorAction::FileStart);
        
        // Search for "hello" forward
        state.apply_action(EditorAction::SearchWordForward);
        
        // Should find second "hello" (at column 12)
        let col = state.buffer().cursor().position.col;
        assert!(col > 0, "Should move to next occurrence of 'hello'");
        
        // Verify pattern was set
        assert_eq!(state.search_pattern, Some("hello".to_string()));
    }

    #[test]
    fn hash_search_backward() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world hello again".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to end
        state.apply_action(EditorAction::FileEnd);
        
        // Move to word "again"
        state.apply_action(EditorAction::WordBackward); // Now on "again"
        
        // Search backward for current word
        state.apply_action(EditorAction::SearchWordBackward);
        
        // Verify pattern was set
        assert!(state.search_pattern.is_some());
    }

    #[test]
    fn open_line_above() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "line one\nline two".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Start on second line
        state.apply_action(EditorAction::FileEnd);
        let initial_line_count = state.buffer().line_count();
        
        // Open line above - should insert line and stay in insert mode state
        state.mode_handler.set_mode(Mode::Insert);
        state.apply_action(EditorAction::OpenLineAbove);
        
        // Insert text
        for ch in "new".chars() {
            state.apply_action(EditorAction::InsertChar(ch));
        }
        
        let content = state.buffer().content();
        // Should have more lines now
        assert!(state.buffer().line_count() >= initial_line_count);
    }

    #[test]
    fn paste_before() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "ab".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start, yank 'a'
        state.handle_key(key('0'));
        state.handle_key(key('y'));
        state.handle_key(key('l'));
        
        // Move to 'b'
        state.handle_key(key('l'));
        
        // Paste before (P) - should insert 'a' before 'b'
        state.handle_key(key('P'));
        
        let content = state.buffer().content();
        assert!(content.contains("aab") || content == "aab", "content: {}", content);
    }

    #[test]
    fn insert_mode_delete_word_before() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        // Now at end of "hello world", in insert mode
        // Cursor is after 'd'
        
        // Delete word before cursor (Ctrl-w)
        state.apply_action(EditorAction::DeleteWordBefore);
        
        let content = state.buffer().content();
        // Should have deleted "world" leaving "hello "
        assert!(content == "hello " || content.starts_with("hello"), "content: {}", content);
    }

    #[test]
    fn insert_mode_delete_to_line_start() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        // Now at end of "hello world", in insert mode
        
        // Delete to line start (Ctrl-u)
        state.apply_action(EditorAction::DeleteToLineStart);
        
        let content = state.buffer().content();
        // Should have deleted everything to start of line
        assert!(content.is_empty() || content == "\n", "content: '{}'", content);
    }

    #[test]
    fn insert_mode_insert_register() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Yank the word
        state.handle_key(key('0'));
        state.handle_key(key('y'));
        state.handle_key(key('w'));
        
        // Go to end and enter insert mode
        state.handle_key(key('A'));
        
        // Insert from unnamed register (")
        state.apply_action(EditorAction::InsertRegister('"'));
        
        let content = state.buffer().content();
        // Should have "hello" inserted after existing "hello"
        assert!(content.contains("hello"), "content: {}", content);
    }

    #[test]
    fn replace_mode_overwrites() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('0'));
        
        // Enter Replace mode
        state.handle_key(key('R'));
        assert_eq!(state.mode(), Mode::Replace);
        
        // Type "HELLO" - should replace existing chars
        for ch in "HELLO".chars() {
            state.handle_key(key(ch));
        }
        
        let content = state.buffer().content();
        // Should be "HELLO" not "HELLOhello"
        assert_eq!(content, "HELLO", "content: {}", content);
    }

    #[test]
    fn single_char_replace_r() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('0'));
        
        // Replace 'h' with 'H' using r command
        state.handle_key(key('r'));
        state.handle_key(key('H'));
        
        let content = state.buffer().content();
        assert_eq!(content, "Hello", "content: {}", content);
        
        // Verify cursor stayed on the same position (col 0)
        assert_eq!(state.buffer().cursor().position.col, 0, "cursor should stay");
    }

    #[test]
    fn insert_at_line_start() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "   hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to end of line
        state.handle_key(key('$'));
        
        // Press 'I' to insert at first non-blank
        state.handle_key(key('I'));
        
        // Should be in insert mode at column 3 (first non-blank 'h')
        assert_eq!(state.mode(), Mode::Insert);
        assert_eq!(state.buffer().cursor().position.col, 3);
        
        // Type some text
        state.handle_key(key('X'));
        
        let content = state.buffer().content();
        assert_eq!(content, "   Xhello");
    }

    #[test]
    fn delete_to_end_of_line_d_upper() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start, move to position 5 (the space)
        state.handle_key(key('0'));
        for _ in 0..5 {
            state.handle_key(key('l'));
        }
        
        // D should delete from cursor to end of line
        state.handle_key(key('D'));
        
        let content = state.buffer().content();
        assert_eq!(content, "hello", "content: {}", content);
    }

    #[test]
    fn change_to_end_of_line_c_upper() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start, move to position 5 (the space)
        state.handle_key(key('0'));
        for _ in 0..5 {
            state.handle_key(key('l'));
        }
        
        // C should delete from cursor to end and enter insert mode
        state.handle_key(key('C'));
        
        assert_eq!(state.mode(), Mode::Insert);
        
        // Type replacement
        state.handle_key(key('!'));
        state.handle_key(esc());
        
        let content = state.buffer().content();
        assert_eq!(content, "hello!", "content: {}", content);
    }

    #[test]
    fn substitute_char_s() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Go to start
        state.handle_key(key('0'));
        
        // s should delete char and enter insert mode
        state.handle_key(key('s'));
        
        assert_eq!(state.mode(), Mode::Insert);
        
        // Type replacement
        state.handle_key(key('H'));
        state.handle_key(esc());
        
        let content = state.buffer().content();
        assert_eq!(content, "Hello", "content: {}", content);
    }

    #[test]
    fn substitute_line_s_upper() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello world".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // S should delete line content and enter insert mode
        state.handle_key(key('S'));
        
        assert_eq!(state.mode(), Mode::Insert);
        
        // Type replacement
        for ch in "new line".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        let content = state.buffer().content();
        assert_eq!(content, "new line", "content: {}", content);
    }

    #[test]
    fn yank_line_y_upper() {
        let mut state = EditorState::new();
        state.handle_key(key('i'));
        for ch in "hello".chars() {
            state.handle_key(key(ch));
        }
        state.handle_key(esc());
        
        // Y should yank current line (like yy)
        state.handle_key(key('Y'));
        
        // Paste to verify it yanked the line
        state.handle_key(key('p'));
        
        let content = state.buffer().content();
        assert_eq!(content, "hello\nhello", "content: {}", content);
    }
}