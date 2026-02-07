//! Event types for editor input and service communication.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Top-level editor events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorEvent {
    KeyInput(KeyEvent),
    Resize(u16, u16),
    FocusGained,
    FocusLost,
    ServiceMessage(ServiceMsg),
    Tick,
}

/// A keyboard event with key code and modifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: Modifiers,
}

impl KeyEvent {
    pub const fn new(code: KeyCode, modifiers: Modifiers) -> Self {
        Self { code, modifiers }
    }

    pub const fn plain(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: Modifiers::NONE,
        }
    }

    pub fn char(c: char) -> Self {
        Self::plain(KeyCode::Char(c))
    }

    pub fn ctrl(c: char) -> Self {
        Self::new(KeyCode::Char(c), Modifiers::CTRL)
    }
}

/// Key codes for keyboard input.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char),
    Escape,
    Enter,
    Backspace,
    Tab,
    Delete,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
    Insert,
    Null,
}

/// Keyboard modifier flags.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Modifiers(u8);

impl Modifiers {
    pub const NONE: Self = Self(0);
    pub const CTRL: Self = Self(1);
    pub const ALT: Self = Self(2);
    pub const SHIFT: Self = Self(4);
    pub const META: Self = Self(8);

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.contains(Self::CTRL) {
            parts.push("CTRL");
        }
        if self.contains(Self::ALT) {
            parts.push("ALT");
        }
        if self.contains(Self::SHIFT) {
            parts.push("SHIFT");
        }
        if self.contains(Self::META) {
            parts.push("META");
        }
        if parts.is_empty() {
            write!(f, "NONE")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}

/// Messages from background services.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceMsg {
    LspResponse,
    GitUpdate,
    FsChange,
    IndexResult,
    TerminalOutput,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifier_contains() {
        let m = Modifiers::CTRL.union(Modifiers::SHIFT);
        assert!(m.contains(Modifiers::CTRL));
        assert!(m.contains(Modifiers::SHIFT));
        assert!(!m.contains(Modifiers::ALT));
    }

    #[test]
    fn key_event_helpers() {
        let k = KeyEvent::ctrl('c');
        assert_eq!(k.code, KeyCode::Char('c'));
        assert!(k.modifiers.contains(Modifiers::CTRL));
    }
}
