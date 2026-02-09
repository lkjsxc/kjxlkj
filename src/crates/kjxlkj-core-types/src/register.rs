//! Register types for yank/paste operations.

use serde::{Deserialize, Serialize};

/// Register name identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisterName {
    /// Named register `a`–`z`.
    Named(char),
    /// Numbered register `0`–`9`.
    Numbered(u8),
    /// Unnamed register `"`.
    Unnamed,
    /// Black hole register `_`.
    BlackHole,
    /// System clipboard `+`.
    Clipboard,
    /// Primary selection `*`.
    PrimarySelection,
    /// Last search pattern `/`.
    LastSearch,
    /// Small delete register `-`.
    SmallDelete,
    /// Last inserted text `.`.
    LastInserted,
    /// Current file name `%`.
    FileName,
    /// Alternate file name `#`.
    AlternateFileName,
    /// Last command `:`.
    LastCommand,
    /// Expression register `=`.
    Expression,
}

/// Contents stored in a register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Register {
    /// The text content.
    pub content: String,
    /// Whether the register holds linewise content.
    pub linewise: bool,
}

impl Register {
    /// Create a new register with the given content.
    pub fn new(content: String, linewise: bool) -> Self {
        Self { content, linewise }
    }

    /// Create an empty register.
    pub fn empty() -> Self {
        Self {
            content: String::new(),
            linewise: false,
        }
    }
}

impl RegisterName {
    /// Parse a character into a register name.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a'..='z' | 'A'..='Z' => Some(RegisterName::Named(c.to_ascii_lowercase())),
            '0'..='9' => Some(RegisterName::Numbered(c as u8 - b'0')),
            '"' => Some(RegisterName::Unnamed),
            '_' => Some(RegisterName::BlackHole),
            '+' => Some(RegisterName::Clipboard),
            '*' => Some(RegisterName::PrimarySelection),
            '/' => Some(RegisterName::LastSearch),
            '-' => Some(RegisterName::SmallDelete),
            '.' => Some(RegisterName::LastInserted),
            '%' => Some(RegisterName::FileName),
            '#' => Some(RegisterName::AlternateFileName),
            ':' => Some(RegisterName::LastCommand),
            '=' => Some(RegisterName::Expression),
            _ => None,
        }
    }

    /// Whether this register is read-only.
    pub fn is_readonly(&self) -> bool {
        matches!(
            self,
            RegisterName::LastInserted
                | RegisterName::FileName
                | RegisterName::AlternateFileName
                | RegisterName::LastCommand
                | RegisterName::LastSearch
        )
    }

    /// Whether writes to append (uppercase named registers).
    pub fn is_append(&self) -> bool {
        false // Append is handled at the call site
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_named_register() {
        assert_eq!(
            RegisterName::from_char('a'),
            Some(RegisterName::Named('a'))
        );
    }

    #[test]
    fn parse_numbered_register() {
        assert_eq!(
            RegisterName::from_char('5'),
            Some(RegisterName::Numbered(5))
        );
    }

    #[test]
    fn parse_special_registers() {
        assert_eq!(
            RegisterName::from_char('"'),
            Some(RegisterName::Unnamed)
        );
        assert_eq!(
            RegisterName::from_char('_'),
            Some(RegisterName::BlackHole)
        );
        assert_eq!(
            RegisterName::from_char('+'),
            Some(RegisterName::Clipboard)
        );
    }

    #[test]
    fn readonly_registers() {
        assert!(RegisterName::FileName.is_readonly());
        assert!(!RegisterName::Named('a').is_readonly());
    }
}
