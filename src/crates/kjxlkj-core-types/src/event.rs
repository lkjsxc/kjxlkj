//! Editor events and key types.

use serde::{Deserialize, Serialize};

/// Modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Modifier {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Modifier {
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
}

/// Key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyEvent {
    /// Character key with modifiers.
    Char(char, Modifier),
    /// Escape key.
    Escape,
    /// Enter key.
    Enter,
    /// Backspace key.
    Backspace,
    /// Tab key.
    Tab,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up.
    PageUp,
    /// Page down.
    PageDown,
    /// Delete key.
    Delete,
}

/// Editor event.
#[derive(Debug, Clone)]
pub enum EditorEvent {
    /// Key press.
    Key(KeyEvent),
    /// Terminal resize.
    Resize(u16, u16),
    /// Quit request.
    Quit,
}
