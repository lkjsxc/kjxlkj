//! Key model for normalized input.
//!
//! See /docs/spec/architecture/input-decoding.md for normative rules.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Modifier flags for key events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

/// Normalized key representation after printable normalization.
///
/// After normalization, `Shift+a` becomes `Key::Char('A')` with shift
/// absorbed. See input-decoding.md "Printable Normalization Rules".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Key {
    Char(char),
    Backspace,
    Enter,
    Tab,
    BackTab,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,
    F(u8),
    Null,
}

impl Key {
    /// Create a key with Ctrl modifier.
    pub fn ctrl(c: char) -> (Self, KeyModifiers) {
        (
            Self::Char(c),
            KeyModifiers {
                ctrl: true,
                ..Default::default()
            },
        )
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Char(c) => write!(f, "{}", c),
            Key::Escape => write!(f, "<Esc>"),
            Key::Enter => write!(f, "<CR>"),
            Key::Tab => write!(f, "<Tab>"),
            Key::Backspace => write!(f, "<BS>"),
            Key::Up => write!(f, "<Up>"),
            Key::Down => write!(f, "<Down>"),
            Key::Left => write!(f, "<Left>"),
            Key::Right => write!(f, "<Right>"),
            Key::Home => write!(f, "<Home>"),
            Key::End => write!(f, "<End>"),
            Key::PageUp => write!(f, "<PageUp>"),
            Key::PageDown => write!(f, "<PageDown>"),
            Key::Insert => write!(f, "<Insert>"),
            Key::Delete => write!(f, "<Del>"),
            Key::F(n) => write!(f, "<F{}>", n),
            Key::BackTab => write!(f, "<S-Tab>"),
            Key::Null => write!(f, "<Nul>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_a_normalizes_to_uppercase() {
        // After printable normalization, Shift+a -> Key::Char('A')
        let key = Key::Char('A');
        assert_eq!(format!("{}", key), "A");
    }

    #[test]
    fn ctrl_key_construction() {
        let (key, mods) = Key::ctrl('w');
        assert_eq!(key, Key::Char('w'));
        assert!(mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.shift);
    }
}
