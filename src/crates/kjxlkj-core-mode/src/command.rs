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

    fn prev_char_boundary(&self) -> usize {
        let mut idx = self.cursor.saturating_sub(1);
        while idx > 0 && !self.buffer.is_char_boundary(idx) {
            idx -= 1;
        }
        idx
    }

    fn next_char_boundary(&self) -> usize {
        let mut idx = self.cursor + 1;
        while idx < self.buffer.len() && !self.buffer.is_char_boundary(idx) {
            idx += 1;
        }
        idx.min(self.buffer.len())
    }

    fn navigate_history(&mut self, direction: Direction) {
        if self.history.is_empty() {
            return;
        }
        match direction {
            Direction::Up => {
                let new_idx = match self.history_index {
                    None => {
                        self.saved_input = self.buffer.clone();
                        self.history.len() - 1
                    }
                    Some(0) => return,
                    Some(i) => i - 1,
                };
                self.history_index = Some(new_idx);
                self.buffer = self.history[new_idx].clone();
                self.cursor = self.buffer.len();
            }
            Direction::Down => {
                match self.history_index {
                    None => {}
                    Some(i) if i + 1 >= self.history.len() => {
                        self.history_index = None;
                        self.buffer = self.saved_input.clone();
                        self.cursor = self.buffer.len();
                    }
                    Some(i) => {
                        self.history_index = Some(i + 1);
                        self.buffer = self.history[i + 1].clone();
                        self.cursor = self.buffer.len();
                    }
                }
            }
            _ => {}
        }
    }

    fn delete_word_backward(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let bytes = self.buffer.as_bytes();
        let mut pos = self.cursor;
        // Skip trailing spaces.
        while pos > 0 && bytes[pos - 1] == b' ' {
            pos -= 1;
        }
        // Skip word chars.
        while pos > 0 && bytes[pos - 1] != b' ' {
            pos -= 1;
        }
        self.buffer.drain(pos..self.cursor);
        self.cursor = pos;
    }

    /// Get the current buffer content.
    pub fn content(&self) -> &str {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_characters() {
        let ex = CommandModeState::new(ActionCommandKind::Ex);
        assert_eq!(ex.prompt_char(), ':');
        let fwd = CommandModeState::new(ActionCommandKind::SearchForward);
        assert_eq!(fwd.prompt_char(), '/');
    }

    #[test]
    fn insert_and_execute() {
        let mut s = CommandModeState::new(ActionCommandKind::Ex);
        s.process_key(&Key::char('q'));
        assert_eq!(s.content(), "q");
        let action = s.process_key(&Key::enter());
        assert!(matches!(action, Some(Action::ExecuteCommand(c)) if c == "q"));
    }

    #[test]
    fn backspace_empty_cancels() {
        let mut s = CommandModeState::new(ActionCommandKind::Ex);
        let action = s.process_key(&Key::new(KeyCode::Backspace));
        assert!(matches!(action, Some(Action::ReturnToNormal)));
    }

    #[test]
    fn escape_cancels() {
        let mut s = CommandModeState::new(ActionCommandKind::Ex);
        s.process_key(&Key::char('f'));
        let action = s.process_key(&Key::esc());
        assert!(matches!(action, Some(Action::ReturnToNormal)));
        assert!(s.buffer.is_empty());
    }
}
