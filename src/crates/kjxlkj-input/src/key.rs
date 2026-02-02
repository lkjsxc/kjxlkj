//! Key types.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Modifiers {
    /// Control key.
    pub ctrl: bool,
    /// Alt/Meta key.
    pub alt: bool,
    /// Shift key.
    pub shift: bool,
}

impl Modifiers {
    /// No modifiers.
    pub fn none() -> Self {
        Self::default()
    }

    /// Control modifier.
    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            ..Self::default()
        }
    }

    /// Alt modifier.
    pub fn alt() -> Self {
        Self {
            alt: true,
            ..Self::default()
        }
    }
}

impl From<KeyModifiers> for Modifiers {
    fn from(mods: KeyModifiers) -> Self {
        Self {
            ctrl: mods.contains(KeyModifiers::CONTROL),
            alt: mods.contains(KeyModifiers::ALT),
            shift: mods.contains(KeyModifiers::SHIFT),
        }
    }
}

/// A single key press.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Key {
    /// Key code.
    pub code: KeyCodeWrapper,
    /// Modifiers.
    pub modifiers: Modifiers,
}

/// Wrapper for key codes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCodeWrapper {
    Char(char),
    Esc,
    Enter,
    Tab,
    Backspace,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
    Other,
}

impl From<KeyCode> for KeyCodeWrapper {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Char(c) => Self::Char(c),
            KeyCode::Esc => Self::Esc,
            KeyCode::Enter => Self::Enter,
            KeyCode::Tab => Self::Tab,
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Delete => Self::Delete,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            KeyCode::Home => Self::Home,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::F(n) => Self::F(n),
            _ => Self::Other,
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        Self {
            code: event.code.into(),
            modifiers: event.modifiers.into(),
        }
    }
}

impl Key {
    /// Creates a character key.
    pub fn char(c: char) -> Self {
        Self {
            code: KeyCodeWrapper::Char(c),
            modifiers: Modifiers::none(),
        }
    }

    /// Creates a key with control.
    pub fn ctrl(c: char) -> Self {
        Self {
            code: KeyCodeWrapper::Char(c),
            modifiers: Modifiers::ctrl(),
        }
    }

    /// Creates an escape key.
    pub fn esc() -> Self {
        Self {
            code: KeyCodeWrapper::Esc,
            modifiers: Modifiers::none(),
        }
    }

    /// Returns true if this is escape.
    pub fn is_esc(&self) -> bool {
        matches!(self.code, KeyCodeWrapper::Esc)
    }
}

/// A sequence of keys.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeySequence {
    /// Keys in the sequence.
    pub keys: Vec<Key>,
}

impl KeySequence {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a key.
    pub fn push(&mut self, key: Key) {
        self.keys.push(key);
    }

    /// Clears the sequence.
    pub fn clear(&mut self) {
        self.keys.clear();
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Returns the length.
    pub fn len(&self) -> usize {
        self.keys.len()
    }
}
