//! Register types for yanked/deleted text.

use serde::{Deserialize, Serialize};

/// A named register (a-z, A-Z, or special).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisterName {
    /// Unnamed register (default).
    Unnamed,
    /// Named register (a-z).
    Named(char),
    /// Numbered register (0-9).
    Numbered(u8),
    /// Small delete register.
    SmallDelete,
    /// Read-only registers.
    ReadOnly(char),
    /// Black hole register.
    BlackHole,
    /// Last search pattern register.
    LastSearch,
    /// Expression register.
    Expression,
}

impl Default for RegisterName {
    fn default() -> Self {
        Self::Unnamed
    }
}

impl RegisterName {
    /// Parse a register name from a character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '"' => Some(Self::Unnamed),
            'a'..='z' | 'A'..='Z' => Some(Self::Named(c.to_ascii_lowercase())),
            '0'..='9' => Some(Self::Numbered(c as u8 - b'0')),
            '-' => Some(Self::SmallDelete),
            '_' => Some(Self::BlackHole),
            '/' => Some(Self::LastSearch),
            '=' => Some(Self::Expression),
            '%' | '#' | ':' | '.' => Some(Self::ReadOnly(c)),
            _ => None,
        }
    }

    /// Check if this register is append-mode (uppercase letter).
    pub fn is_append(&self) -> bool {
        matches!(self, Self::Named(c) if c.is_ascii_uppercase())
    }
}

/// Contents of a register.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Register {
    /// The text content.
    pub content: String,
    /// Whether this is linewise content.
    pub linewise: bool,
}

impl Register {
    /// Create a new empty register.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a register with content.
    pub fn with_content(content: String, linewise: bool) -> Self {
        Self { content, linewise }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_name_from_char() {
        assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
        assert_eq!(
            RegisterName::from_char('a'),
            Some(RegisterName::Named('a'))
        );
        assert_eq!(
            RegisterName::from_char('0'),
            Some(RegisterName::Numbered(0))
        );
        assert_eq!(RegisterName::from_char('_'), Some(RegisterName::BlackHole));
    }

    #[test]
    fn register_name_special() {
        assert_eq!(RegisterName::from_char('/'), Some(RegisterName::LastSearch));
        assert_eq!(RegisterName::from_char('-'), Some(RegisterName::SmallDelete));
        assert_eq!(RegisterName::from_char('='), Some(RegisterName::Expression));
    }

    #[test]
    fn register_name_readonly() {
        assert_eq!(RegisterName::from_char('%'), Some(RegisterName::ReadOnly('%')));
        assert_eq!(RegisterName::from_char('#'), Some(RegisterName::ReadOnly('#')));
    }

    #[test]
    fn register_name_invalid() {
        assert!(RegisterName::from_char('~').is_none());
        assert!(RegisterName::from_char('@').is_none());
    }

    #[test]
    fn register_default() {
        assert_eq!(RegisterName::default(), RegisterName::Unnamed);
    }

    #[test]
    fn register_content() {
        let reg = Register::with_content("text".to_string(), true);
        assert_eq!(reg.content, "text");
        assert!(reg.linewise);
    }

    #[test]
    fn register_new_empty() {
        let reg = Register::new();
        assert!(reg.content.is_empty());
        assert!(!reg.linewise);
    }

    #[test]
    fn register_charwise_content() {
        let reg = Register::with_content("word".to_string(), false);
        assert!(!reg.linewise);
    }

    #[test]
    fn register_uppercase_named() {
        assert_eq!(
            RegisterName::from_char('Z'),
            Some(RegisterName::Named('z'))
        );
    }

    #[test]
    fn register_numbered_nine() {
        assert_eq!(
            RegisterName::from_char('9'),
            Some(RegisterName::Numbered(9))
        );
    }
}
