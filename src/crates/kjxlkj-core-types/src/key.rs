//! Key event types for input handling.

use serde::{Deserialize, Serialize};

/// A key event from the terminal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Key {
    /// The key code.
    pub code: KeyCode,
    /// Modifier keys held.
    pub mods: KeyModifiers,
}

impl Key {
    /// Create a new key with no modifiers.
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            mods: KeyModifiers::NONE,
        }
    }

    /// Create a key with Ctrl modifier.
    pub fn ctrl(code: KeyCode) -> Self {
        Self {
            code,
            mods: KeyModifiers::CTRL,
        }
    }

    /// Create a key with Shift modifier.
    pub fn shift(code: KeyCode) -> Self {
        Self {
            code,
            mods: KeyModifiers::SHIFT,
        }
    }

    /// Create a character key.
    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c))
    }
}

/// Key code enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    /// Character key.
    Char(char),
    /// Function key (F1-F12).
    F(u8),
    /// Escape key.
    Esc,
    /// Enter/Return key.
    Enter,
    /// Tab key.
    Tab,
    /// Backspace key.
    Backspace,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Arrow keys.
    Left,
    Right,
    Up,
    Down,
}

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct KeyModifiers {
    bits: u8,
}

impl KeyModifiers {
    pub const NONE: Self = Self { bits: 0 };
    pub const CTRL: Self = Self { bits: 1 };
    pub const ALT: Self = Self { bits: 2 };
    pub const SHIFT: Self = Self { bits: 4 };

    /// Check if Ctrl is held.
    pub fn ctrl(self) -> bool {
        self.bits & 1 != 0
    }

    /// Check if Alt is held.
    pub fn alt(self) -> bool {
        self.bits & 2 != 0
    }

    /// Check if Shift is held.
    pub fn shift(self) -> bool {
        self.bits & 4 != 0
    }

    /// Combine modifiers.
    pub fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}
