//! Key event types for input handling.

/// Modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    /// Shift key pressed.
    pub shift: bool,
    /// Control key pressed.
    pub ctrl: bool,
    /// Alt key pressed.
    pub alt: bool,
}

impl Modifiers {
    /// No modifiers.
    pub const NONE: Self = Self {
        shift: false,
        ctrl: false,
        alt: false,
    };

    /// Shift only.
    pub const SHIFT: Self = Self {
        shift: true,
        ctrl: false,
        alt: false,
    };

    /// Control only.
    pub const CTRL: Self = Self {
        shift: false,
        ctrl: true,
        alt: false,
    };

    /// Alt only.
    pub const ALT: Self = Self {
        shift: false,
        ctrl: false,
        alt: true,
    };

    /// Check if any modifier is pressed.
    pub fn any(&self) -> bool {
        self.shift || self.ctrl || self.alt
    }
}

/// Special keys that are not printable characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialKey {
    /// Escape key.
    Escape,
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
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Function key.
    F(u8),
}

/// Normalized key event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    /// Printable character (already shift-normalized).
    Char(char),
    /// Special non-printable key.
    Special(SpecialKey),
}

/// Key event with modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// The key pressed.
    pub key: Key,
    /// Modifier state (shift excluded for printable chars).
    pub modifiers: Modifiers,
}

impl KeyEvent {
    /// Create a new key event.
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    /// Create a simple char key event without modifiers.
    pub fn char(c: char) -> Self {
        Self {
            key: Key::Char(c),
            modifiers: Modifiers::NONE,
        }
    }

    /// Create a special key event.
    pub fn special(key: SpecialKey) -> Self {
        Self {
            key: Key::Special(key),
            modifiers: Modifiers::NONE,
        }
    }

    /// Create a ctrl+char key event.
    pub fn ctrl(c: char) -> Self {
        Self {
            key: Key::Char(c),
            modifiers: Modifiers::CTRL,
        }
    }
}
