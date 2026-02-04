//! Register types for text storage.

use serde::{Deserialize, Serialize};

/// A named register for storing text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisterName {
    /// Named register a-z.
    Named(char),
    /// Unnamed (default) register.
    Unnamed,
    /// Small delete register (for deletions less than one line).
    SmallDelete,
    /// Numbered register 0-9.
    Numbered(u8),
    /// Black hole register (discards content).
    BlackHole,
    /// Last search pattern register.
    Search,
    /// Command register.
    Command,
    /// Expression register.
    Expression,
    /// Clipboard register.
    Clipboard,
    /// Primary selection register.
    Primary,
    /// Last inserted text register.
    LastInserted,
    /// Filename register.
    Filename,
    /// Alternate filename register.
    AltFilename,
}

impl RegisterName {
    /// Parse a register name from a character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a'..='z' | 'A'..='Z' => Some(RegisterName::Named(c.to_ascii_lowercase())),
            '"' => Some(RegisterName::Unnamed),
            '-' => Some(RegisterName::SmallDelete),
            '0'..='9' => Some(RegisterName::Numbered(c as u8 - b'0')),
            '_' => Some(RegisterName::BlackHole),
            '/' => Some(RegisterName::Search),
            ':' => Some(RegisterName::Command),
            '=' => Some(RegisterName::Expression),
            '+' => Some(RegisterName::Clipboard),
            '*' => Some(RegisterName::Primary),
            '.' => Some(RegisterName::LastInserted),
            '%' => Some(RegisterName::Filename),
            '#' => Some(RegisterName::AltFilename),
            _ => None,
        }
    }

    /// Get the character representation of this register.
    pub fn as_char(&self) -> char {
        match self {
            RegisterName::Named(c) => *c,
            RegisterName::Unnamed => '"',
            RegisterName::SmallDelete => '-',
            RegisterName::Numbered(n) => (b'0' + n) as char,
            RegisterName::BlackHole => '_',
            RegisterName::Search => '/',
            RegisterName::Command => ':',
            RegisterName::Expression => '=',
            RegisterName::Clipboard => '+',
            RegisterName::Primary => '*',
            RegisterName::LastInserted => '.',
            RegisterName::Filename => '%',
            RegisterName::AltFilename => '#',
        }
    }
}

impl Default for RegisterName {
    fn default() -> Self {
        RegisterName::Unnamed
    }
}

/// Content stored in a register.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Register {
    /// The text content.
    pub content: String,
    /// Whether this is line-wise content.
    pub linewise: bool,
}

impl Register {
    /// Create a new register with content.
    pub fn new(content: impl Into<String>, linewise: bool) -> Self {
        Self {
            content: content.into(),
            linewise,
        }
    }

    /// Create an empty register.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Check if the register is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_name_from_char() {
        assert_eq!(RegisterName::from_char('a'), Some(RegisterName::Named('a')));
        assert_eq!(RegisterName::from_char('A'), Some(RegisterName::Named('a')));
        assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
        assert_eq!(
            RegisterName::from_char('0'),
            Some(RegisterName::Numbered(0))
        );
        assert_eq!(RegisterName::from_char('_'), Some(RegisterName::BlackHole));
        assert_eq!(RegisterName::from_char('!'), None);
    }

    #[test]
    fn test_register_content() {
        let reg = Register::new("hello", false);
        assert_eq!(reg.content, "hello");
        assert!(!reg.linewise);
    }
}
