//! Key types.

use serde::{Deserialize, Serialize};

/// Key code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char),
    Enter,
    Esc,
    Backspace,
    Tab,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8),
}

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Modifiers {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            ..Self::default()
        }
    }

    pub fn alt() -> Self {
        Self {
            alt: true,
            ..Self::default()
        }
    }

    pub fn shift() -> Self {
        Self {
            shift: true,
            ..Self::default()
        }
    }
}

/// A key press.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Key {
    pub code: KeyCode,
    pub mods: Modifiers,
}

impl Key {
    pub fn new(code: KeyCode, mods: Modifiers) -> Self {
        Self { code, mods }
    }

    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c), Modifiers::none())
    }

    pub fn ctrl(c: char) -> Self {
        Self::new(KeyCode::Char(c), Modifiers::ctrl())
    }

    pub fn esc() -> Self {
        Self::new(KeyCode::Esc, Modifiers::none())
    }

    pub fn enter() -> Self {
        Self::new(KeyCode::Enter, Modifiers::none())
    }

    pub fn backspace() -> Self {
        Self::new(KeyCode::Backspace, Modifiers::none())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_creation() {
        let k = Key::char('a');
        assert_eq!(k.code, KeyCode::Char('a'));
        assert!(!k.mods.ctrl);

        let k = Key::ctrl('r');
        assert_eq!(k.code, KeyCode::Char('r'));
        assert!(k.mods.ctrl);
    }
}
