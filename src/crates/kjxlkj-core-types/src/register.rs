//! Register types for kjxlkj editor.

use serde::{Deserialize, Serialize};

/// Named register identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum Register {
    /// Unnamed register (default for yank/delete).
    #[default]
    Unnamed,
    /// Numbered registers 0-9.
    Numbered(u8),
    /// Named registers a-z (or A-Z for append).
    Named(char),
    /// Small delete register (- register).
    SmallDelete,
    /// Expression register (= register).
    Expression,
    /// Selection registers (* and +).
    Selection(SelectionRegister),
    /// Black hole register (_ register).
    BlackHole,
    /// Last search pattern register (/ register).
    LastSearch,
    /// Last inserted text register (. register).
    LastInserted,
    /// Last command register (: register).
    LastCommand,
    /// Current file name register (% register).
    CurrentFile,
    /// Alternate file name register (# register).
    AlternateFile,
}


impl Register {
    /// Parses a register from a character.
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '"' => Self::Unnamed,
            '0'..='9' => Self::Numbered(c as u8 - b'0'),
            'a'..='z' | 'A'..='Z' => Self::Named(c.to_ascii_lowercase()),
            '-' => Self::SmallDelete,
            '=' => Self::Expression,
            '*' => Self::Selection(SelectionRegister::Primary),
            '+' => Self::Selection(SelectionRegister::Clipboard),
            '_' => Self::BlackHole,
            '/' => Self::LastSearch,
            '.' => Self::LastInserted,
            ':' => Self::LastCommand,
            '%' => Self::CurrentFile,
            '#' => Self::AlternateFile,
            _ => return None,
        })
    }

    /// Returns the character representation.
    pub fn to_char(&self) -> char {
        match self {
            Self::Unnamed => '"',
            Self::Numbered(n) => (b'0' + n) as char,
            Self::Named(c) => *c,
            Self::SmallDelete => '-',
            Self::Expression => '=',
            Self::Selection(SelectionRegister::Primary) => '*',
            Self::Selection(SelectionRegister::Clipboard) => '+',
            Self::BlackHole => '_',
            Self::LastSearch => '/',
            Self::LastInserted => '.',
            Self::LastCommand => ':',
            Self::CurrentFile => '%',
            Self::AlternateFile => '#',
        }
    }

    /// Returns true if this register is read-only.
    pub fn is_readonly(&self) -> bool {
        matches!(
            self,
            Self::LastInserted
                | Self::LastCommand
                | Self::CurrentFile
                | Self::AlternateFile
        )
    }

    /// Returns true if writes should append (uppercase named registers).
    pub fn should_append(&self, original_char: char) -> bool {
        matches!(self, Self::Named(_)) && original_char.is_ascii_uppercase()
    }
}

/// Selection/clipboard register type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionRegister {
    /// Primary selection (* register).
    Primary,
    /// System clipboard (+ register).
    Clipboard,
}

/// Type of content stored in a register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RegisterType {
    /// Character-wise content.
    #[default]
    Char,
    /// Line-wise content.
    Line,
    /// Block-wise (visual block) content.
    Block,
}

/// Content of a register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterContent {
    /// The text content.
    pub text: String,
    /// The type of content.
    pub register_type: RegisterType,
}

impl Default for RegisterContent {
    fn default() -> Self {
        Self {
            text: String::new(),
            register_type: RegisterType::Char,
        }
    }
}

impl RegisterContent {
    /// Creates new register content.
    pub fn new(text: String, register_type: RegisterType) -> Self {
        Self { text, register_type }
    }

    /// Creates character-wise content.
    pub fn char_wise(text: String) -> Self {
        Self::new(text, RegisterType::Char)
    }

    /// Creates line-wise content.
    pub fn line_wise(text: String) -> Self {
        Self::new(text, RegisterType::Line)
    }
}
