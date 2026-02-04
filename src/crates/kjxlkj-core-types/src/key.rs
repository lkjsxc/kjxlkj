//! Key input types.

use serde::{Deserialize, Serialize};

/// Key code for input events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char),
    Backspace,
    Enter,
    Tab,
    Esc,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8),
    Null,
}

/// Modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Modifiers {
    /// No modifiers.
    pub const NONE: Self = Self {
        ctrl: false,
        alt: false,
        shift: false,
    };

    /// Ctrl modifier only.
    pub const CTRL: Self = Self {
        ctrl: true,
        alt: false,
        shift: false,
    };

    /// Check if no modifiers are active.
    pub fn is_empty(&self) -> bool {
        !self.ctrl && !self.alt && !self.shift
    }
}

/// A key event with code and modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Key {
    pub code: KeyCode,
    pub mods: Modifiers,
}

impl Key {
    /// Create a key with no modifiers.
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            mods: Modifiers::NONE,
        }
    }

    /// Create a key with modifiers.
    pub fn with_mods(code: KeyCode, mods: Modifiers) -> Self {
        Self { code, mods }
    }

    /// Create a plain character key.
    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c))
    }

    /// Create a Ctrl+char key.
    pub fn ctrl(c: char) -> Self {
        Self::with_mods(KeyCode::Char(c), Modifiers::CTRL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_char() {
        let k = Key::char('a');
        assert_eq!(k.code, KeyCode::Char('a'));
        assert!(k.mods.is_empty());
    }

    #[test]
    fn key_ctrl() {
        let k = Key::ctrl('r');
        assert_eq!(k.code, KeyCode::Char('r'));
        assert!(k.mods.ctrl);
    }
}
