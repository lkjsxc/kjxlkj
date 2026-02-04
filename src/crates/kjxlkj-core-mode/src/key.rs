//! Key representation.

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyModifiers {
    pub const NONE: Self = Self {
        ctrl: false,
        alt: false,
        shift: false,
    };

    pub const CTRL: Self = Self {
        ctrl: true,
        alt: false,
        shift: false,
    };

    pub const ALT: Self = Self {
        ctrl: false,
        alt: true,
        shift: false,
    };

    pub const SHIFT: Self = Self {
        ctrl: false,
        alt: false,
        shift: true,
    };
}

/// Key code enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Char(char),
    Escape,
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
}

/// A key event with code and modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Key {
    /// Create a key with no modifiers.
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }

    /// Create a character key.
    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c))
    }

    /// Create a Ctrl+char key.
    pub fn ctrl(c: char) -> Self {
        Self {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::CTRL,
        }
    }

    /// Create escape key.
    pub fn escape() -> Self {
        Self::new(KeyCode::Escape)
    }

    /// Create enter key.
    pub fn enter() -> Self {
        Self::new(KeyCode::Enter)
    }

    /// Check if this is the escape key.
    pub fn is_escape(&self) -> bool {
        self.code == KeyCode::Escape && self.modifiers == KeyModifiers::NONE
    }

    /// Check if this is a printable character.
    pub fn is_printable(&self) -> bool {
        matches!(self.code, KeyCode::Char(c) if !c.is_control())
            && self.modifiers == KeyModifiers::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_key() {
        let key = Key::escape();
        assert!(key.is_escape());
    }

    #[test]
    fn printable_char() {
        let key = Key::char('a');
        assert!(key.is_printable());
    }

    #[test]
    fn ctrl_not_printable() {
        let key = Key::ctrl('a');
        assert!(!key.is_printable());
    }
}
