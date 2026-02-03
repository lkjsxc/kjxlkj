//! Register types for text storage.

use serde::{Deserialize, Serialize};

/// Register name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisterName {
    /// Unnamed register (default).
    Unnamed,
    /// Named register (a-z, A-Z).
    Named(char),
    /// Numbered register (0-9).
    Numbered(u8),
    /// Black hole register (_).
    BlackHole,
    /// Clipboard register (+).
    Clipboard,
    /// Selection register (*).
    Selection,
    /// Last search register (/).
    Search,
    /// Last command register (:).
    Command,
    /// Last inserted text register (.).
    Insert,
    /// Current file name register (%).
    FileName,
    /// Alternate file name register (#).
    AltFileName,
    /// Expression register (=).
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
            'a'..='z' | 'A'..='Z' => Some(Self::Named(c)),
            '0'..='9' => Some(Self::Numbered(c as u8 - b'0')),
            '_' => Some(Self::BlackHole),
            '+' => Some(Self::Clipboard),
            '*' => Some(Self::Selection),
            '/' => Some(Self::Search),
            ':' => Some(Self::Command),
            '.' => Some(Self::Insert),
            '%' => Some(Self::FileName),
            '#' => Some(Self::AltFileName),
            '=' => Some(Self::Expression),
            _ => None,
        }
    }
}

/// Register content type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterContent {
    /// The text content.
    pub text: String,
    /// The content type.
    pub reg_type: RegisterType,
}

impl RegisterContent {
    /// Create new register content.
    pub fn new(text: impl Into<String>, reg_type: RegisterType) -> Self {
        Self {
            text: text.into(),
            reg_type,
        }
    }

    /// Create character-wise content.
    pub fn char(text: impl Into<String>) -> Self {
        Self::new(text, RegisterType::Char)
    }

    /// Create line-wise content.
    pub fn line(text: impl Into<String>) -> Self {
        Self::new(text, RegisterType::Line)
    }
}

impl Default for RegisterContent {
    fn default() -> Self {
        Self::char(String::new())
    }
}
