//! Register types.

use serde::{Deserialize, Serialize};

/// Register name (a-z, 0-9, or special).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RegisterName {
    /// Named register (a-z).
    Named(char),
    /// Numbered register (0-9).
    Numbered(u8),
    /// Unnamed (default) register.
    #[default]
    Unnamed,
    /// Black hole register (discards content).
    BlackHole,
    /// Last search pattern register.
    Search,
    /// Small delete register.
    SmallDelete,
    /// Clipboard register.
    Clipboard,
}

impl RegisterName {
    /// Parse a register name from a character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a'..='z' => Some(RegisterName::Named(c)),
            'A'..='Z' => Some(RegisterName::Named(c.to_ascii_lowercase())),
            '0'..='9' => Some(RegisterName::Numbered(c as u8 - b'0')),
            '"' => Some(RegisterName::Unnamed),
            '_' => Some(RegisterName::BlackHole),
            '/' => Some(RegisterName::Search),
            '-' => Some(RegisterName::SmallDelete),
            '+' | '*' => Some(RegisterName::Clipboard),
            _ => None,
        }
    }
}

/// Content stored in a register.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Register {
    /// The text content.
    pub content: String,
    /// Whether this is linewise content.
    pub linewise: bool,
}

impl Register {
    /// Create a new register with content.
    pub fn new(content: String, linewise: bool) -> Self {
        Self { content, linewise }
    }

    /// Create an empty register.
    pub fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_name_from_char() {
        assert_eq!(RegisterName::from_char('a'), Some(RegisterName::Named('a')));
        assert_eq!(RegisterName::from_char('A'), Some(RegisterName::Named('a')));
        assert_eq!(
            RegisterName::from_char('5'),
            Some(RegisterName::Numbered(5))
        );
        assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
        assert_eq!(RegisterName::from_char('_'), Some(RegisterName::BlackHole));
    }
}
