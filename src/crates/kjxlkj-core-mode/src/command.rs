//! Command-line mode state: input buffer, cursor, history.

use kjxlkj_core_types::{
    Action, ActionCommandKind, Direction, Key, KeyCode, KeyModifiers,
};

/// State maintained during Command-line mode (`:`, `/`, `?`).
#[derive(Debug)]
pub struct CommandModeState {
    /// What kind of command line this is.
    pub kind: ActionCommandKind,
    /// The current input buffer.
    pub buffer: String,
    /// Cursor position within the buffer (byte offset).
    pub cursor: usize,
    /// History of previous commands for this kind.
    pub history: Vec<String>,
    /// Current history navigation index (None = not navigating).
    pub history_index: Option<usize>,
    /// Saved input when starting history navigation.
    pub saved_input: String,
}

impl CommandModeState {
    pub fn new(kind: ActionCommandKind) -> Self {
        Self {
            kind,
            buffer: String::new(),
            cursor: 0,
            history: Vec::new(),
            history_index: None,
            saved_input: String::new(),
        }
    }

    /// Get the prompt character for display.
    pub fn prompt_char(&self) -> char {
        match self.kind {
            ActionCommandKind::Ex => ':',
            ActionCommandKind::SearchForward => '/',
            ActionCommandKind::SearchBackward => '?',
        }
    }

    /// Process a key event in Command-line mode.
    pub fn process_key(&mut self, key: &Key) -> Option<Action> {
        match (&key.code, key.modifiers) {
            // Escape → cancel, return to Normal.
            (KeyCode::Esc, _) => {
                self.buffer.clear();
                self.cursor = 0;
                self.history_index = None;
                Some(Action::ReturnToNormal)
            }

            // Ctrl-C → same as Escape.
            (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CTRL) => {
                self.buffer.clear();
                self.cursor = 0;
                self.history_index = None;
                Some(Action::ReturnToNormal)
            }

            // Enter → execute command / search.
            (KeyCode::Enter, _) => {
                let text = self.buffer.clone();
                if !text.is_empty() {
                    self.history.push(text.clone());
                }
                self.buffer.clear();
                self.cursor = 0;
                self.history_index = None;
                match self.kind {
                    ActionCommandKind::Ex => {
                        Some(Action::ExecuteCommand(text))
                    }
                    ActionCommandKind::SearchForward => {
                        Some(Action::SearchForward(text))
                    }
                    ActionCommandKind::SearchBackward => {
                        Some(Action::SearchBackward(text))
                    }
                }
            }

            // Backspace → remove char before cursor.
            (KeyCode::Backspace, _) => {
                if self.cursor > 0 {
                    let prev = self.prev_char_boundary();
                    self.buffer.drain(prev..self.cursor);
                    self.cursor = prev;
                } else {
                    // Empty buffer + backspace → cancel.
                    return Some(Action::ReturnToNormal);
                }
                None
            }

            // Delete → remove char at cursor.
            (KeyCode::Delete, _) => {
                if self.cursor < self.buffer.len() {
                    let next = self.next_char_boundary();
                    self.buffer.drain(self.cursor..next);
                }
                None
            }

            // Left → move cursor left.
            (KeyCode::Left, _) => {
                if self.cursor > 0 {
                    self.cursor = self.prev_char_boundary();
                }
                None
            }

            // Right → move cursor right.
            (KeyCode::Right, _) => {
                if self.cursor < self.buffer.len() {
                    self.cursor = self.next_char_boundary();
                }
                None
            }

            // Home → start.
            (KeyCode::Home, _) => {
                self.cursor = 0;
                None
            }

            // End → end.
            (KeyCode::End, _) => {
                self.cursor = self.buffer.len();
                None
            }

            // Up → history navigate backward.
            (KeyCode::Up, _) => {
                self.navigate_history(Direction::Up);
                None
            }

            // Down → history navigate forward.
            (KeyCode::Down, _) => {
                self.navigate_history(Direction::Down);
                None
            }

            // Ctrl-W → delete word backward.
            (KeyCode::Char('w'), m) if m.contains(KeyModifiers::CTRL) => {
                self.delete_word_backward();
                None
            }

            // Ctrl-U → delete to start.
            (KeyCode::Char('u'), m) if m.contains(KeyModifiers::CTRL) => {
                self.buffer.drain(..self.cursor);
                self.cursor = 0;
                None
            }

            // Tab → completion.
            (KeyCode::Tab, _) => Some(Action::CmdlineComplete),

            // Regular character.
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                self.buffer.insert(self.cursor, *c);
                self.cursor += c.len_utf8();
                None
            }

            _ => None,
        }
    }
}
