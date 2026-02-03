//! Mode input handling.

use kjxlkj_core_types::{EditorAction, Mode, Motion, Operator, TextObject};

use crate::{ModeState, PendingOperator};

/// Check if a character is a valid register name.
/// Valid registers: a-z, A-Z, 0-9, ", +, *, _, /, etc.
fn is_valid_register(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(ch, '"' | '+' | '*' | '_' | '/' | '-' | '.' | ':' | '%' | '#')
}

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

    /// Sets the current mode.
    pub fn set_mode(&mut self, mode: Mode) {
        self.state.set_mode(mode);
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
            Mode::SearchForward | Mode::SearchBackward => self.handle_search(key),
        }
    }

    fn handle_normal(&mut self, key: KeyInput) -> EditorAction {
        if key.is_escape() {
            self.state.clear_pending();
            self.state.take_pending_operator();
            return EditorAction::Nop;
        }

        if let Some(ch) = key.char() {
            // Handle count prefix
            if ch.is_ascii_digit() && ch != '0' {
                let digit = ch.to_digit(10).unwrap();
                let count = self.state.take_count().unwrap_or(0) * 10 + digit;
                self.state.set_count(count);
                return EditorAction::Nop;
            }

            // Check if we're in operator-pending state
            if self.state.is_operator_pending() {
                return self.handle_operator_pending(ch);
            }

            self.state.push_key(ch);
            let pending: String = self.state.pending_keys().iter().collect();
            
            // Check for operator entry
            let action = match pending.as_str() {
                // Operators that enter operator-pending state
                "d" => {
                    self.state.set_pending_operator(PendingOperator::Delete);
                    self.state.clear_pending();
                    return EditorAction::Nop;
                }
                "y" => {
                    self.state.set_pending_operator(PendingOperator::Yank);
                    self.state.clear_pending();
                    return EditorAction::Nop;
                }
                "c" => {
                    self.state.set_pending_operator(PendingOperator::Change);
                    self.state.clear_pending();
                    return EditorAction::Nop;
                }
                ">" => {
                    self.state.set_pending_operator(PendingOperator::Indent);
                    self.state.clear_pending();
                    return EditorAction::Nop;
                }
                "<" => {
                    self.state.set_pending_operator(PendingOperator::Outdent);
                    self.state.clear_pending();
                    return EditorAction::Nop;
                }

                // Double-operator for line operations
                "dd" => {
                    let count = self.state.take_count();
                    self.state.clear_pending();
                    EditorAction::OperatorMotion {
                        operator: Operator::Delete,
                        motion: Motion::CurrentLine,
                        count,
                    }
                }
                "yy" => {
                    let count = self.state.take_count();
                    self.state.clear_pending();
                    EditorAction::OperatorMotion {
                        operator: Operator::Yank,
                        motion: Motion::CurrentLine,
                        count,
                    }
                }
                "cc" => {
                    let count = self.state.take_count();
                    self.state.clear_pending();
                    self.state.set_mode(Mode::Insert);
                    EditorAction::OperatorMotion {
                        operator: Operator::Change,
                        motion: Motion::CurrentLine,
                        count,
                    }
                }
                ">>" => {
                    let count = self.state.take_count();
                    self.state.clear_pending();
                    EditorAction::OperatorMotion {
                        operator: Operator::Indent,
                        motion: Motion::CurrentLine,
                        count,
                    }
                }
                "<<" => {
                    let count = self.state.take_count();
                    self.state.clear_pending();
                    EditorAction::OperatorMotion {
                        operator: Operator::Outdent,
                        motion: Motion::CurrentLine,
                        count,
                    }
                }

                // Simple motions
                "h" => EditorAction::CursorLeft,
                "j" => EditorAction::CursorDown,
                "k" => EditorAction::CursorUp,
                "l" => EditorAction::CursorRight,
                "0" => EditorAction::LineStart,
                "^" => EditorAction::FirstNonBlank,
                "$" => EditorAction::LineEnd,
                "w" => EditorAction::WordForward,
                "W" => EditorAction::WORDForward,
                "b" => EditorAction::WordBackward,
                "B" => EditorAction::WORDBackward,
                "e" => EditorAction::WordEnd,
                "E" => EditorAction::WORDEnd,
                "gg" => EditorAction::FileStart,
                "G" => EditorAction::FileEnd,
                
                // Mode entries
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
                "/" => {
                    self.state.set_mode(Mode::SearchForward);
                    self.command_line.clear();
                    EditorAction::EnterSearchForward
                }
                "?" => {
                    self.state.set_mode(Mode::SearchBackward);
                    self.command_line.clear();
                    EditorAction::EnterSearchBackward
                }
                "n" => EditorAction::SearchNext,
                "N" => EditorAction::SearchPrev,
                
                // Find char motions (pending for next char)
                "f" | "t" | "F" | "T" => {
                    // Wait for the target character
                    return EditorAction::Nop;
                }
                
                // Register selection (pending for next char)
                "\"" => {
                    // Wait for the register name
                    return EditorAction::Nop;
                }
                
                // Macro recording (pending for next char)
                "q" => {
                    // Wait for the register name
                    return EditorAction::Nop;
                }
                
                // Macro playback (pending for next char)
                "@" => {
                    // Wait for the register name
                    return EditorAction::Nop;
                }
                
                // Repeat find char
                ";" => EditorAction::RepeatFindChar,
                "," => EditorAction::RepeatFindCharReverse,
                
                // Character operations
                "x" => EditorAction::DeleteCharAt,
                "p" => EditorAction::PasteAfter,
                "u" => EditorAction::Undo,
                "." => EditorAction::RepeatLastChange,
                
                _ => {
                    // Handle @@ for repeat last macro
                    if pending == "@@" {
                        self.state.clear_pending();
                        return EditorAction::RepeatLastMacro;
                    }
                    
                    // Handle f{char}, t{char}, F{char}, T{char}, m{mark}, `{mark}, '{mark}, "{reg}, q{reg}, @{reg}
                    if pending.len() == 2 {
                        let chars: Vec<char> = pending.chars().collect();
                        let cmd = chars[0];
                        let target = chars[1];
                        self.state.clear_pending();
                        
                        return match cmd {
                            'f' => EditorAction::FindCharForward(target),
                            'F' => EditorAction::FindCharBackward(target),
                            't' => EditorAction::TillCharForward(target),
                            'T' => EditorAction::TillCharBackward(target),
                            'm' if target.is_ascii_alphabetic() => EditorAction::SetMark(target),
                            '`' if target.is_ascii_alphabetic() => EditorAction::JumpToMarkExact(target),
                            '\'' if target.is_ascii_alphabetic() => EditorAction::JumpToMarkLine(target),
                            '"' if is_valid_register(target) => EditorAction::SetPendingRegister(target),
                            'q' if target.is_ascii_alphabetic() => EditorAction::ToggleMacroRecording(target),
                            '@' if target.is_ascii_alphabetic() => EditorAction::PlayMacro(target),
                            _ => EditorAction::Nop,
                        };
                    }
                    
                    // Wait for more keys if needed (e.g., for gg, f{char}, m{mark}, "{reg}, q{reg}, @{reg})
                    if pending.len() >= 2 && !matches!(pending.as_str(), "g" | "f" | "F" | "t" | "T" | "m" | "`" | "\'" | "\"" | "q" | "@") {
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

    /// Handle motion after operator (operator-pending state).
    fn handle_operator_pending(&mut self, ch: char) -> EditorAction {
        let pending_op = match self.state.take_pending_operator() {
            Some(op) => op,
            None => return EditorAction::Nop,
        };
        
        let operator = match pending_op {
            PendingOperator::Delete => Operator::Delete,
            PendingOperator::Yank => Operator::Yank,
            PendingOperator::Change => Operator::Change,
            PendingOperator::Indent => Operator::Indent,
            PendingOperator::Outdent => Operator::Outdent,
            _ => return EditorAction::Nop,
        };

        // Check for double-operator (dd, yy, cc, etc.)
        if ch == pending_op.char() {
            let count = self.state.take_count();
            if operator == Operator::Change {
                self.state.set_mode(Mode::Insert);
            }
            return EditorAction::OperatorMotion {
                operator,
                motion: Motion::CurrentLine,
                count,
            };
        }

        // Map character to motion or text object
        self.state.push_key(ch);
        let pending_str: String = self.state.pending_keys().iter().collect();
        
        // Check for text objects first
        let text_object = match pending_str.as_str() {
            "iw" => Some(TextObject::InnerWord),
            "aw" => Some(TextObject::AroundWord),
            "iW" => Some(TextObject::InnerWORD),
            "aW" => Some(TextObject::AroundWORD),
            "i(" | "i)" | "ib" => Some(TextObject::InnerParen),
            "a(" | "a)" | "ab" => Some(TextObject::AroundParen),
            "i[" | "i]" => Some(TextObject::InnerBracket),
            "a[" | "a]" => Some(TextObject::AroundBracket),
            "i{" | "i}" | "iB" => Some(TextObject::InnerBrace),
            "a{" | "a}" | "aB" => Some(TextObject::AroundBrace),
            "i\"" => Some(TextObject::InnerDoubleQuote),
            "a\"" => Some(TextObject::AroundDoubleQuote),
            "i'" => Some(TextObject::InnerSingleQuote),
            "a'" => Some(TextObject::AroundSingleQuote),
            "i" | "a" => {
                // Wait for second character (text object identifier)
                self.state.set_pending_operator(pending_op);
                return EditorAction::Nop;
            }
            _ => None,
        };

        if let Some(obj) = text_object {
            self.state.clear_pending();
            if operator == Operator::Change {
                self.state.set_mode(Mode::Insert);
            }
            return EditorAction::OperatorTextObject {
                operator,
                text_object: obj,
            };
        }
        
        let motion = match pending_str.as_str() {
            "h" => Some(Motion::Left),
            "j" => Some(Motion::Down),
            "k" => Some(Motion::Up),
            "l" => Some(Motion::Right),
            "0" => Some(Motion::LineStart),
            "^" => Some(Motion::FirstNonBlank),
            "$" => Some(Motion::LineEnd),
            "w" => Some(Motion::WordForward),
            "b" => Some(Motion::WordBackward),
            "e" => Some(Motion::WordEnd),
            "gg" => Some(Motion::FileStart),
            "G" => Some(Motion::FileEnd),
            "g" => {
                // Wait for second character
                self.state.set_pending_operator(pending_op);
                return EditorAction::Nop;
            }
            "f" | "F" | "t" | "T" => {
                // Wait for target character
                self.state.set_pending_operator(pending_op);
                return EditorAction::Nop;
            }
            _ => None,
        };

        // Check for f{char}, t{char}, F{char}, T{char}
        if pending_str.len() == 2 {
            let chars: Vec<char> = pending_str.chars().collect();
            let cmd = chars[0];
            let target = chars[1];
            
            let motion = match cmd {
                'f' => Some(Motion::FindCharForward(target)),
                'F' => Some(Motion::FindCharBackward(target)),
                't' => Some(Motion::TillCharForward(target)),
                'T' => Some(Motion::TillCharBackward(target)),
                _ => None,
            };
            
            if let Some(m) = motion {
                self.state.clear_pending();
                let count = self.state.take_count();
                if operator == Operator::Change {
                    self.state.set_mode(Mode::Insert);
                }
                return EditorAction::OperatorMotion {
                    operator,
                    motion: m,
                    count,
                };
            }
        }

        self.state.clear_pending();

        match motion {
            Some(m) => {
                let count = self.state.take_count();
                if operator == Operator::Change {
                    self.state.set_mode(Mode::Insert);
                }
                EditorAction::OperatorMotion {
                    operator,
                    motion: m,
                    count,
                }
            }
            None => EditorAction::Nop,
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
        
        // Handle pending g for gg
        if self.state.pending_keys() == ['g'] {
            self.state.clear_pending();
            if let Some('g') = key.char() {
                return EditorAction::FileStart;
            }
            // Invalid after g, just ignore
            return EditorAction::Nop;
        }
        
        if let Some(ch) = key.char() {
            match ch {
                // Motions
                'h' => return EditorAction::CursorLeft,
                'j' => return EditorAction::CursorDown,
                'k' => return EditorAction::CursorUp,
                'l' => return EditorAction::CursorRight,
                'w' => return EditorAction::WordForward,
                'W' => return EditorAction::WORDForward,
                'b' => return EditorAction::WordBackward,
                'B' => return EditorAction::WORDBackward,
                'e' => return EditorAction::WordEnd,
                'E' => return EditorAction::WORDEnd,
                '0' => return EditorAction::LineStart,
                '^' => return EditorAction::FirstNonBlank,
                '$' => return EditorAction::LineEnd,
                'g' => {
                    // gg - file start (pending)
                    self.state.push_key('g');
                    return EditorAction::Nop;
                }
                'G' => return EditorAction::FileEnd,
                // Operators
                'd' => return EditorAction::VisualDelete,
                'x' => return EditorAction::VisualDelete,
                'y' => return EditorAction::VisualYank,
                'c' => return EditorAction::VisualChange,
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

    fn handle_search(&mut self, key: KeyInput) -> EditorAction {
        if key.is_escape() {
            self.state.set_mode(Mode::Normal);
            self.command_line.clear();
            return EditorAction::ReturnToNormalMode;
        }
        if key.is_enter() {
            let pattern = self.command_line.content.clone();
            self.state.set_mode(Mode::Normal);
            self.command_line.clear();
            return EditorAction::ExecuteSearch(pattern);
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

impl std::fmt::Display for KeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            KeyCode::Char(c) => {
                if self.modifiers.ctrl {
                    write!(f, "<C-{}>", c)
                } else {
                    write!(f, "{}", c)
                }
            }
            KeyCode::Escape => write!(f, "<Esc>"),
            KeyCode::Enter => write!(f, "<CR>"),
            KeyCode::Backspace => write!(f, "<BS>"),
            KeyCode::Left => write!(f, "<Left>"),
            KeyCode::Right => write!(f, "<Right>"),
            KeyCode::Up => write!(f, "<Up>"),
            KeyCode::Down => write!(f, "<Down>"),
            KeyCode::Tab => write!(f, "<Tab>"),
            KeyCode::Other => write!(f, "<Other>"),
        }
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
