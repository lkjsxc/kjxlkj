//! Register types for yank/paste operations.

use serde::{Deserialize, Serialize};

/// A register name (a-z, 0-9, or special).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisterName {
    /// Named register a-z.
    Named(char),
    /// Numbered register 0-9.
    Numbered(u8),
    /// Unnamed (default) register.
    Unnamed,
    /// Black hole register (discards content).
    BlackHole,
    /// Last search pattern register.
    Search,
    /// Clipboard register.
    Clipboard,
    /// Last inserted text.
    LastInsert,
    /// Current file name.
    FileName,
    /// Alternate file name.
    AltFileName,
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
            'a'..='z' | 'A'..='Z' => Some(Self::Named(c.to_ascii_lowercase())),
            '0'..='9' => Some(Self::Numbered(c as u8 - b'0')),
            '"' => Some(Self::Unnamed),
            '_' => Some(Self::BlackHole),
            '/' => Some(Self::Search),
            '+' | '*' => Some(Self::Clipboard),
            '.' => Some(Self::LastInsert),
            '%' => Some(Self::FileName),
            '#' => Some(Self::AltFileName),
            '=' => Some(Self::Expression),
            _ => None,
        }
    }
}

/// Contents of a register.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Register {
    /// The text content.
    pub content: String,
    /// Whether this is a linewise register.
    pub linewise: bool,
    /// Whether this is a blockwise register.
    pub blockwise: bool,
}

impl Register {
    /// Create a new charwise register.
    pub fn charwise(content: String) -> Self {
        Self {
            content,
            linewise: false,
            blockwise: false,
        }
    }

    /// Create a new linewise register.
    pub fn linewise(content: String) -> Self {
        Self {
            content,
            linewise: true,
            blockwise: false,
        }
    }

    /// Create a new blockwise register.
    pub fn blockwise(content: String) -> Self {
        Self {
            content,
            linewise: false,
            blockwise: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_name_parse() {
        assert_eq!(RegisterName::from_char('a'), Some(RegisterName::Named('a')));
        assert_eq!(RegisterName::from_char('0'), Some(RegisterName::Numbered(0)));
        assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
    }

    #[test]
    fn register_linewise() {
        let r = Register::linewise("hello\n".to_string());
        assert!(r.linewise);
        assert!(!r.blockwise);
    }
}
