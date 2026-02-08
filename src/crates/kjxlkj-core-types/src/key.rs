//! Key representation and modifier handling.

use serde::{Deserialize, Serialize};

use bitflags::bitflags;

bitflags! {
    /// Modifier keys attached to a key event.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct KeyModifiers: u8 {
        /// No modifiers.
        const NONE    = 0b0000_0000;
        /// Shift key.
        const SHIFT   = 0b0000_0001;
        /// Control key.
        const CTRL    = 0b0000_0010;
        /// Alt/Meta key.
        const ALT     = 0b0000_0100;
    }
}

/// Terminal key codes after decoding.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    /// A printable character.
    Char(char),
    /// Function key F1..F12.
    F(u8),
    /// Backspace (BS).
    Backspace,
    /// Enter/Return (CR).
    Enter,
    /// Tab.
    Tab,
    /// Shift+Tab.
    BackTab,
    /// Escape.
    Esc,
    /// Delete.
    Delete,
    /// Insert.
    Insert,
    /// Home.
    Home,
    /// End.
    End,
    /// Page Up.
    PageUp,
    /// Page Down.
    PageDown,
    /// Arrow Up.
    Up,
    /// Arrow Down.
    Down,
    /// Arrow Left.
    Left,
    /// Arrow Right.
    Right,
    /// Null (ignored).
    Null,
}

/// A decoded key event with modifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Key {
    /// The key code.
    pub code: KeyCode,
    /// Active modifier keys.
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

    /// Create a key with Ctrl modifier.
    pub fn ctrl(c: char) -> Self {
        Self {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::CTRL,
        }
    }

    /// Create a plain character key.
    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c))
    }

    /// Create the Escape key.
    pub fn esc() -> Self {
        Self::new(KeyCode::Esc)
    }

    /// Create the Enter key.
    pub fn enter() -> Self {
        Self::new(KeyCode::Enter)
    }

    /// Create the Backspace key.
    pub fn backspace() -> Self {
        Self::new(KeyCode::Backspace)
    }

    /// Create the Tab key.
    pub fn tab() -> Self {
        Self::new(KeyCode::Tab)
    }

    /// Check if this is a digit character for count parsing.
    pub fn is_digit(&self) -> bool {
        matches!(
            self,
            Key {
                code: KeyCode::Char('0'..='9'),
                modifiers,
            } if *modifiers == KeyModifiers::NONE
        )
    }

    /// Extract the digit value if this is a digit key.
    pub fn digit_value(&self) -> Option<u32> {
        if let Key {
            code: KeyCode::Char(c),
            modifiers,
        } = self
        {
            if *modifiers == KeyModifiers::NONE {
                return c.to_digit(10);
            }
        }
        None
    }

    /// Format key for display (e.g., "C-w", "M-x").
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if self.modifiers.contains(KeyModifiers::CTRL) {
            parts.push("C-");
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("M-");
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("S-");
        }
        let code_str = match &self.code {
            KeyCode::Char(c) => format!("{c}"),
            KeyCode::F(n) => format!("F{n}"),
            KeyCode::Backspace => "BS".into(),
            KeyCode::Enter => "CR".into(),
            KeyCode::Tab => "Tab".into(),
            KeyCode::BackTab => "S-Tab".into(),
            KeyCode::Esc => "Esc".into(),
            KeyCode::Delete => "Del".into(),
            KeyCode::Insert => "Insert".into(),
            KeyCode::Home => "Home".into(),
            KeyCode::End => "End".into(),
            KeyCode::PageUp => "PageUp".into(),
            KeyCode::PageDown => "PageDown".into(),
            KeyCode::Up => "Up".into(),
            KeyCode::Down => "Down".into(),
            KeyCode::Left => "Left".into(),
            KeyCode::Right => "Right".into(),
            KeyCode::Null => "Null".into(),
        };
        format!("{}{}", parts.join(""), code_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_detection() {
        assert!(Key::char('5').is_digit());
        assert!(!Key::char('a').is_digit());
        assert!(!Key::ctrl('5').is_digit());
    }

    #[test]
    fn digit_value_extraction() {
        assert_eq!(Key::char('3').digit_value(), Some(3));
        assert_eq!(Key::char('a').digit_value(), None);
    }

    #[test]
    fn display_format() {
        assert_eq!(Key::ctrl('w').display(), "C-w");
        assert_eq!(Key::char('j').display(), "j");
        assert_eq!(Key::esc().display(), "Esc");
    }
}
