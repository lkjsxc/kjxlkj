//! Mode state machine.

use kjxlkj_core_types::Mode;

use crate::intent::{Intent, IntentKind};
use crate::key::Key;
use crate::parser::KeyParser;

/// Mode state machine.
#[derive(Debug)]
pub struct ModeState {
    /// Current mode.
    mode: Mode,
    /// Key parser for current mode.
    parser: KeyParser,
    /// Pending count prefix.
    count: Option<usize>,
    /// Pending register.
    register: Option<char>,
}

impl ModeState {
    /// Create a new mode state starting in Normal mode.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            parser: KeyParser::new(),
            count: None,
            register: None,
        }
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Process a key press and return an intent.
    pub fn process_key(&mut self, key: Key) -> Intent {
        // Handle escape in any mode
        if key.is_escape() && self.mode != Mode::Normal {
            return self.transition_to(Mode::Normal);
        }

        match self.mode {
            Mode::Normal => self.process_normal(key),
            Mode::Insert => self.process_insert(key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                self.process_visual(key)
            }
            Mode::Command => self.process_command(key),
            Mode::Replace => self.process_replace(key),
        }
    }

    /// Transition to a new mode.
    fn transition_to(&mut self, new_mode: Mode) -> Intent {
        self.mode = new_mode;
        self.parser.reset();
        self.count = None;
        self.register = None;
        Intent::new(IntentKind::ExitToNormal)
    }

    /// Process a key in Normal mode.
    fn process_normal(&mut self, key: Key) -> Intent {
        self.parser.process_normal(key, &mut self.count, &mut self.register, &mut self.mode)
    }

    /// Process a key in Insert mode.
    fn process_insert(&mut self, key: Key) -> Intent {
        if key.is_escape() {
            self.mode = Mode::Normal;
            return Intent::new(IntentKind::ExitToNormal);
        }

        if key.is_printable() {
            if let crate::key::KeyCode::Char(c) = key.code {
                return Intent::new(IntentKind::InsertChar(c));
            }
        }

        match key.code {
            crate::key::KeyCode::Enter => Intent::new(IntentKind::NewlineBelow),
            crate::key::KeyCode::Backspace => Intent::new(IntentKind::DeleteCharBefore),
            crate::key::KeyCode::Delete => Intent::new(IntentKind::DeleteChar),
            crate::key::KeyCode::Tab => Intent::new(IntentKind::InsertChar('\t')),
            _ => Intent::noop(),
        }
    }

    /// Process a key in Visual mode.
    fn process_visual(&mut self, key: Key) -> Intent {
        if key.is_escape() {
            self.mode = Mode::Normal;
            return Intent::new(IntentKind::ExitToNormal);
        }
        // Visual mode uses similar motions to Normal
        self.parser.process_visual(key, &mut self.count, &mut self.mode)
    }

    /// Process a key in Command mode.
    fn process_command(&mut self, key: Key) -> Intent {
        if key.is_escape() {
            self.mode = Mode::Normal;
            return Intent::new(IntentKind::ExitToNormal);
        }
        self.parser.process_command(key, &mut self.mode)
    }

    /// Process a key in Replace mode.
    fn process_replace(&mut self, key: Key) -> Intent {
        if key.is_escape() {
            self.mode = Mode::Normal;
            return Intent::new(IntentKind::ExitToNormal);
        }

        if key.is_printable() {
            if let crate::key::KeyCode::Char(c) = key.code {
                // In replace mode, typing replaces the character under cursor
                return Intent::new(IntentKind::InsertChar(c));
            }
        }
        Intent::noop()
    }
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_normal_mode() {
        let state = ModeState::new();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn escape_returns_to_normal() {
        let mut state = ModeState::new();
        state.mode = Mode::Insert;
        let intent = state.process_key(Key::escape());
        assert_eq!(state.mode(), Mode::Normal);
        assert!(matches!(intent.kind, IntentKind::ExitToNormal));
    }

    #[test]
    fn insert_char_in_insert_mode() {
        let mut state = ModeState::new();
        state.mode = Mode::Insert;
        let intent = state.process_key(Key::char('a'));
        assert!(matches!(intent.kind, IntentKind::InsertChar('a')));
    }
}
