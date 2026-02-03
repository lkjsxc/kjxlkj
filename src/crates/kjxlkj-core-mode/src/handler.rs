//! Mode input handling.

use kjxlkj_core_types::{EditorAction, Mode};

use crate::ModeState;

/// Command line input state.
#[derive(Debug, Default, Clone)]
pub struct CommandLineState {
    pub content: String,
    pub cursor_pos: usize,
}

impl CommandLineState {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_pos = 0;
    }

    pub fn insert(&mut self, ch: char) {
        self.content.insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.content.remove(self.cursor_pos);
        }
    }
}

/// Handles mode-specific input interpretation.
#[derive(Debug, Default)]
pub struct ModeHandler {
    state: ModeState,
    command_line: CommandLineState,
}

impl ModeHandler {
    pub fn new() -> Self {
        Self {
            state: ModeState::new(),
            command_line: CommandLineState::new(),
        }
    }

    pub fn mode(&self) -> Mode {
        self.state.mode()
    }

    pub fn command_line(&self) -> &CommandLineState {
        &self.command_line
    }

    pub fn command_line_mut(&mut self) -> &mut CommandLineState {
        &mut self.command_line
    }

    /// Handles a key press and returns the resulting action.
    pub fn handle_key(&mut self, key: KeyInput) -> EditorAction {
        match self.state.mode() {
            Mode::Normal => self.handle_normal(key),
            Mode::Insert => self.handle_insert(key),
            Mode::Visual => self.handle_visual(key),
            Mode::VisualLine => self.handle_visual(key),
            Mode::VisualBlock => self.handle_visual(key),
            Mode::Command => self.handle_command(key),
            Mode::Replace => self.handle_replace(key),
        }
    }

    fn handle_normal(&mut self, key: KeyInput) -> EditorAction {
        if key.is_escape() {
            self.state.clear_pending();
            return EditorAction::Nop;
        }

        if let Some(ch) = key.char() {
            if ch.is_ascii_digit() && ch != '0' {
                let digit = ch.to_digit(10).unwrap();
                let count = self.state.take_count().unwrap_or(0) * 10 + digit;
                self.state.set_count(count);
                return EditorAction::Nop;
            }

            self.state.push_key(ch);
            let pending: String = self.state.pending_keys().iter().collect();
            let action = match pending.as_str() {
                "h" => EditorAction::CursorLeft,
                "j" => EditorAction::CursorDown,
                "k" => EditorAction::CursorUp,
                "l" => EditorAction::CursorRight,
                "0" => EditorAction::LineStart,
                "$" => EditorAction::LineEnd,
                "i" => {
                    self.state.set_mode(Mode::Insert);
                    EditorAction::EnterInsertMode
                }
                "a" => {
                    self.state.set_mode(Mode::Insert);
                    EditorAction::EnterInsertModeAfter
                }
                "A" => {
                    self.state.set_mode(Mode::Insert);
                    EditorAction::EnterInsertModeEndOfLine
                }
                "o" => {
                    self.state.set_mode(Mode::Insert);
                    EditorAction::OpenLineBelow
                }
                "v" => {
                    self.state.set_mode(Mode::Visual);
                    EditorAction::EnterVisualMode
                }
                "V" => {
                    self.state.set_mode(Mode::VisualLine);
                    EditorAction::EnterVisualLineMode
                }
                "R" => {
                    self.state.set_mode(Mode::Replace);
                    EditorAction::EnterReplaceMode
                }
                ":" => {
                    self.state.set_mode(Mode::Command);
                    self.command_line.clear();
                    EditorAction::EnterCommandMode
                }
                "x" => EditorAction::DeleteCharAt,
                "dd" => EditorAction::DeleteLine,
                "yy" => EditorAction::YankLine,
                "p" => EditorAction::PasteAfter,
                "u" => EditorAction::Undo,
                _ => {
                    if pending.len() >= 2 {
                        self.state.clear_pending();
                    }
                    return EditorAction::Nop;
                }
            };
            self.state.clear_pending();
            action
        } else if key.is_ctrl('r') {
            EditorAction::Redo
        } else if key.is_arrow_left() {
            EditorAction::CursorLeft
        } else if key.is_arrow_right() {
            EditorAction::CursorRight
        } else if key.is_arrow_up() {
            EditorAction::CursorUp
        } else if key.is_arrow_down() {
            EditorAction::CursorDown
        } else {
            EditorAction::Nop
        }
    }

    fn handle_insert(&mut self, key: KeyInput) -> EditorAction {
        if key.is_escape() {
            self.state.set_mode(Mode::Normal);
            return EditorAction::ReturnToNormalMode;
        }
        if key.is_backspace() {
            return EditorAction::DeleteCharBefore;
        }
        if key.is_enter() {
            return EditorAction::InsertNewline;
        }
        if let Some(ch) = key.char() {
            return EditorAction::InsertChar(ch);
        }
        if key.is_arrow_left() {
            return EditorAction::CursorLeft;
        }
        if key.is_arrow_right() {
            return EditorAction::CursorRight;
        }
        if key.is_arrow_up() {
            return EditorAction::CursorUp;
        }
        if key.is_arrow_down() {
            return EditorAction::CursorDown;
        }
        EditorAction::Nop
    }

    fn handle_visual(&mut self, key: KeyInput) -> EditorAction {
        if key.is_escape() {
            self.state.set_mode(Mode::Normal);
            return EditorAction::ReturnToNormalMode;
        }
        if let Some(ch) = key.char() {
            match ch {
                'h' => return EditorAction::CursorLeft,
                'j' => return EditorAction::CursorDown,
                'k' => return EditorAction::CursorUp,
                'l' => return EditorAction::CursorRight,
                _ => {}
            }
        }
        EditorAction::Nop
    }

    fn handle_command(&mut self, key: KeyInput) -> EditorAction {
        if key.is_escape() {
            self.state.set_mode(Mode::Normal);
            self.command_line.clear();
            return EditorAction::ReturnToNormalMode;
        }
        if key.is_enter() {
            let cmd = self.command_line.content.clone();
            self.state.set_mode(Mode::Normal);
            self.command_line.clear();
            return EditorAction::ExecuteCommand(cmd);
        }
        if key.is_backspace() {
            if self.command_line.content.is_empty() {
                self.state.set_mode(Mode::Normal);
                return EditorAction::ReturnToNormalMode;
            }
            self.command_line.backspace();
            return EditorAction::Nop;
        }
        if let Some(ch) = key.char() {
            self.command_line.insert(ch);
        }
        EditorAction::Nop
    }

    fn handle_replace(&mut self, key: KeyInput) -> EditorAction {
        if key.is_escape() {
            self.state.set_mode(Mode::Normal);
            return EditorAction::ReturnToNormalMode;
        }
        if let Some(ch) = key.char() {
            return EditorAction::InsertChar(ch);
        }
        EditorAction::Nop
    }
}

/// Abstraction over key input.
#[derive(Debug, Clone)]
pub struct KeyInput {
    pub code: KeyCode,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Escape,
    Enter,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Tab,
    Other,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyInput {
    pub fn char(&self) -> Option<char> {
        if let KeyCode::Char(c) = self.code {
            if !self.modifiers.ctrl && !self.modifiers.alt {
                return Some(c);
            }
        }
        None
    }

    pub fn is_escape(&self) -> bool {
        self.code == KeyCode::Escape
    }

    pub fn is_enter(&self) -> bool {
        self.code == KeyCode::Enter
    }

    pub fn is_backspace(&self) -> bool {
        self.code == KeyCode::Backspace
    }

    pub fn is_arrow_left(&self) -> bool {
        self.code == KeyCode::Left
    }

    pub fn is_arrow_right(&self) -> bool {
        self.code == KeyCode::Right
    }

    pub fn is_arrow_up(&self) -> bool {
        self.code == KeyCode::Up
    }

    pub fn is_arrow_down(&self) -> bool {
        self.code == KeyCode::Down
    }

    pub fn is_ctrl(&self, ch: char) -> bool {
        if let KeyCode::Char(c) = self.code {
            return self.modifiers.ctrl && c == ch;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(ch: char) -> KeyInput {
        KeyInput {
            code: KeyCode::Char(ch),
            modifiers: Modifiers::default(),
        }
    }

    fn escape() -> KeyInput {
        KeyInput {
            code: KeyCode::Escape,
            modifiers: Modifiers::default(),
        }
    }

    #[test]
    fn normal_to_insert() {
        let mut handler = ModeHandler::new();
        let action = handler.handle_key(key('i'));
        assert!(matches!(action, EditorAction::EnterInsertMode));
        assert_eq!(handler.mode(), Mode::Insert);
    }

    #[test]
    fn insert_escape_to_normal() {
        let mut handler = ModeHandler::new();
        handler.handle_key(key('i'));
        let action = handler.handle_key(escape());
        assert!(matches!(action, EditorAction::ReturnToNormalMode));
        assert_eq!(handler.mode(), Mode::Normal);
    }

    #[test]
    fn command_mode_entry() {
        let mut handler = ModeHandler::new();
        handler.handle_key(key(':'));
        assert_eq!(handler.mode(), Mode::Command);
    }
}
