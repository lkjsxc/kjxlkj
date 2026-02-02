//! Key types.

use crossterm::event::{KeyCode, KeyModifiers as CtKeyModifiers};

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers {
    /// Control key.
    pub ctrl: bool,
    /// Alt/Meta key.
    pub alt: bool,
    /// Shift key.
    pub shift: bool,
}

impl KeyModifiers {
    /// No modifiers.
    pub const NONE: Self = Self {
        ctrl: false,
        alt: false,
        shift: false,
    };

    /// Control modifier.
    pub const CTRL: Self = Self {
        ctrl: true,
        alt: false,
        shift: false,
    };

    /// Alt modifier.
    pub const ALT: Self = Self {
        ctrl: false,
        alt: true,
        shift: false,
    };

    /// Shift modifier.
    pub const SHIFT: Self = Self {
        ctrl: false,
        alt: false,
        shift: true,
    };

    /// Creates from crossterm modifiers.
    pub fn from_crossterm(mods: CtKeyModifiers) -> Self {
        Self {
            ctrl: mods.contains(CtKeyModifiers::CONTROL),
            alt: mods.contains(CtKeyModifiers::ALT),
            shift: mods.contains(CtKeyModifiers::SHIFT),
        }
    }
}

/// A key code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// A character key.
    Char(char),
    /// Function keys.
    F(u8),
    /// Special keys.
    Backspace,
    Enter,
    Tab,
    Escape,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
}

impl Key {
    /// Creates from crossterm key code.
    pub fn from_crossterm(code: KeyCode) -> Option<Self> {
        Some(match code {
            KeyCode::Char(c) => Self::Char(c),
            KeyCode::F(n) => Self::F(n),
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Enter => Self::Enter,
            KeyCode::Tab => Self::Tab,
            KeyCode::Esc => Self::Escape,
            KeyCode::Insert => Self::Insert,
            KeyCode::Delete => Self::Delete,
            KeyCode::Home => Self::Home,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            _ => return None,
        })
    }
}

/// A key event combining key and modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    /// The key.
    pub key: Key,
    /// Modifiers.
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    /// Creates a new key event.
    pub fn new(key: Key, modifiers: KeyModifiers) -> Self {
        Self { key, modifiers }
    }

    /// Creates from crossterm event.
    pub fn from_crossterm(event: crossterm::event::KeyEvent) -> Option<Self> {
        Some(Self {
            key: Key::from_crossterm(event.code)?,
            modifiers: KeyModifiers::from_crossterm(event.modifiers),
        })
    }

    /// Returns true if this is a character key with no modifiers.
    pub fn is_plain_char(&self) -> bool {
        matches!(self.key, Key::Char(_))
            && !self.modifiers.ctrl
            && !self.modifiers.alt
    }

    /// Returns the character if this is a plain char key.
    pub fn as_char(&self) -> Option<char> {
        if let Key::Char(c) = self.key {
            Some(c)
        } else {
            None
        }
    }
}
