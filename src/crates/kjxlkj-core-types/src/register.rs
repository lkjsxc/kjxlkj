//! Register types for text storage.

use serde::{Deserialize, Serialize};

/// Register identifier covering all register types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisterName {
    /// `"` unnamed register (default target).
    Unnamed,
    /// `0` last yank register.
    Yank,
    /// `1`-`9` numbered delete history.
    Numbered(u8),
    /// `a`-`z` named registers.
    Named(char),
    /// `_` black hole register (discards content).
    BlackHole,
    /// `+` system clipboard.
    Clipboard,
    /// `*` X11 primary selection.
    Primary,
    /// `/` last search pattern.
    Search,
    /// `.` last inserted text (read-only).
    LastInserted,
    /// `:` last ex command (read-only).
    LastCommand,
    /// `%` current filename (read-only).
    CurrentFile,
    /// `#` alternate filename (read-only).
    AlternateFile,
    /// `=` expression register.
    Expression,
    /// `-` small delete register.
    SmallDelete,
}

impl RegisterName {
    /// Parse a character into a register name.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '"' => Some(Self::Unnamed),
            '_' => Some(Self::BlackHole),
            '+' => Some(Self::Clipboard),
            '*' => Some(Self::Primary),
            '/' => Some(Self::Search),
            '.' => Some(Self::LastInserted),
            ':' => Some(Self::LastCommand),
            '%' => Some(Self::CurrentFile),
            '#' => Some(Self::AlternateFile),
            '=' => Some(Self::Expression),
            '-' => Some(Self::SmallDelete),
            '0' => Some(Self::Yank),
            '1'..='9' => Some(Self::Numbered(c as u8 - b'0')),
            'a'..='z' => Some(Self::Named(c)),
            'A'..='Z' => Some(Self::Named(c.to_ascii_lowercase())),
            _ => None,
        }
    }

    /// Whether this is a read-only register.
    pub fn is_readonly(&self) -> bool {
        matches!(
            self,
            Self::LastInserted
                | Self::LastCommand
                | Self::CurrentFile
                | Self::AlternateFile
        )
    }

    /// Whether writing to `A`-`Z` appends to the named register.
    pub fn is_append(c: char) -> bool {
        c.is_ascii_uppercase()
    }
}

/// The type of content stored in a register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisterType {
    Charwise,
    Linewise,
    Blockwise,
}

/// Content of a register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterContent {
    pub text: String,
    pub reg_type: RegisterType,
}

impl RegisterContent {
    pub fn charwise(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            reg_type: RegisterType::Charwise,
        }
    }

    pub fn linewise(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            reg_type: RegisterType::Linewise,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_from_char() {
        assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
        assert_eq!(RegisterName::from_char('a'), Some(RegisterName::Named('a')));
        assert_eq!(
            RegisterName::from_char('A'),
            Some(RegisterName::Named('a'))
        );
        assert_eq!(
            RegisterName::from_char('1'),
            Some(RegisterName::Numbered(1))
        );
        assert_eq!(RegisterName::from_char('_'), Some(RegisterName::BlackHole));
        assert_eq!(RegisterName::from_char('!'), None);
    }

    #[test]
    fn register_readonly() {
        assert!(RegisterName::LastInserted.is_readonly());
        assert!(!RegisterName::Named('a').is_readonly());
    }

    #[test]
    fn register_append() {
        assert!(RegisterName::is_append('A'));
        assert!(!RegisterName::is_append('a'));
    }
}
