//! Register types for text storage.

use serde::{Deserialize, Serialize};

/// Register name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RegisterName {
    /// Unnamed register (default).
    #[default]
    Unnamed,
    /// Named register (a-z, A-Z).
    Named(char),
    /// Numbered register (0-9).
    Numbered(u8),
    /// Small delete register.
    SmallDelete,
    /// Last search pattern.
    SearchPattern,
    /// Command register.
    Command,
    /// Expression register.
    Expression,
    /// Black hole register.
    BlackHole,
    /// Last inserted text.
    LastInserted,
    /// Filename register.
    Filename,
    /// Clipboard register.
    Clipboard,
}

impl RegisterName {
    /// Parse a register name from a character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '"' => Some(RegisterName::Unnamed),
            'a'..='z' | 'A'..='Z' => Some(RegisterName::Named(c)),
            '0'..='9' => Some(RegisterName::Numbered(c as u8 - b'0')),
            '-' => Some(RegisterName::SmallDelete),
            '/' => Some(RegisterName::SearchPattern),
            ':' => Some(RegisterName::Command),
            '=' => Some(RegisterName::Expression),
            '_' => Some(RegisterName::BlackHole),
            '.' => Some(RegisterName::LastInserted),
            '%' => Some(RegisterName::Filename),
            '+' | '*' => Some(RegisterName::Clipboard),
            _ => None,
        }
    }
}

/// Register content type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RegisterType {
    /// Character-wise content.
    #[default]
    Char,
    /// Line-wise content.
    Line,
    /// Block-wise content.
    Block,
}

/// Register content.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterContent {
    /// The text content.
    pub text: String,
    /// The content type.
    pub reg_type: RegisterType,
}

impl RegisterContent {
    /// Create new register content.
    pub fn new(text: String, reg_type: RegisterType) -> Self {
        Self { text, reg_type }
    }

    /// Create character-wise content.
    pub fn char_wise(text: String) -> Self {
        Self::new(text, RegisterType::Char)
    }

    /// Create line-wise content.
    pub fn line_wise(text: String) -> Self {
        Self::new(text, RegisterType::Line)
    }
}
