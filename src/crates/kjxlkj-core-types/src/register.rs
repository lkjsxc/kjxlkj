//! Register types for text storage.

use serde::{Deserialize, Serialize};

/// A named register for storing text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RegisterName {
    /// Named register a-z.
    Named(char),
    /// Unnamed (default) register.
    #[default]
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

    #[test]
    fn test_register_name_as_char() {
        assert_eq!(RegisterName::Named('a').as_char(), 'a');
        assert_eq!(RegisterName::Unnamed.as_char(), '"');
        assert_eq!(RegisterName::BlackHole.as_char(), '_');
        assert_eq!(RegisterName::Numbered(5).as_char(), '5');
    }

    #[test]
    fn test_register_name_default() {
        assert_eq!(RegisterName::default(), RegisterName::Unnamed);
    }

    #[test]
    fn test_register_linewise() {
        let reg = Register::new("line1\nline2", true);
        assert!(reg.linewise);
    }

    #[test]
    fn test_register_empty() {
        let reg = Register::empty();
        assert!(reg.is_empty());
    }

    #[test]
    fn test_register_is_not_empty() {
        let reg = Register::new("text", false);
        assert!(!reg.is_empty());
    }

    #[test]
    fn test_register_name_search() {
        assert_eq!(RegisterName::from_char('/'), Some(RegisterName::Search));
        assert_eq!(RegisterName::Search.as_char(), '/');
    }

    #[test]
    fn test_register_name_clipboard() {
        assert_eq!(RegisterName::from_char('+'), Some(RegisterName::Clipboard));
        assert_eq!(RegisterName::Clipboard.as_char(), '+');
    }

    #[test]
    fn test_register_name_filename() {
        assert_eq!(RegisterName::from_char('%'), Some(RegisterName::Filename));
        assert_eq!(RegisterName::Filename.as_char(), '%');
    }

    #[test]
    fn test_register_clone() {
        let reg = Register::new("test", false);
        let cloned = reg.clone();
        assert_eq!(reg, cloned);
    }

    #[test]
    fn test_register_name_primary() {
        assert_eq!(RegisterName::from_char('*'), Some(RegisterName::Primary));
        assert_eq!(RegisterName::Primary.as_char(), '*');
    }

    #[test]
    fn test_register_name_command() {
        assert_eq!(RegisterName::from_char(':'), Some(RegisterName::Command));
        assert_eq!(RegisterName::Command.as_char(), ':');
    }

    #[test]
    fn test_register_name_expression() {
        assert_eq!(RegisterName::from_char('='), Some(RegisterName::Expression));
        assert_eq!(RegisterName::Expression.as_char(), '=');
    }

    #[test]
    fn test_register_name_last_inserted() {
        assert_eq!(RegisterName::from_char('.'), Some(RegisterName::LastInserted));
        assert_eq!(RegisterName::LastInserted.as_char(), '.');
    }

    #[test]
    fn test_register_name_alt_filename() {
        assert_eq!(RegisterName::from_char('#'), Some(RegisterName::AltFilename));
        assert_eq!(RegisterName::AltFilename.as_char(), '#');
    }

    #[test]
    fn test_register_name_small_delete() {
        assert_eq!(RegisterName::from_char('-'), Some(RegisterName::SmallDelete));
        assert_eq!(RegisterName::SmallDelete.as_char(), '-');
    }

    #[test]
    fn test_register_name_numbered_all() {
        for n in 0..=9u8 {
            let c = (b'0' + n) as char;
            assert_eq!(RegisterName::from_char(c), Some(RegisterName::Numbered(n)));
            assert_eq!(RegisterName::Numbered(n).as_char(), c);
        }
    }

    #[test]
    fn test_register_default() {
        let reg = Register::default();
        assert!(reg.content.is_empty());
        assert!(!reg.linewise);
    }

    #[test]
    fn test_register_name_debug() {
        let name = RegisterName::Named('x');
        let debug = format!("{:?}", name);
        assert!(debug.contains("Named"));
    }

    #[test]
    fn test_register_content_with_newlines() {
        let reg = Register::new("line1\nline2\nline3", true);
        assert!(reg.content.contains('\n'));
    }

    #[test]
    fn test_register_name_equality() {
        assert_eq!(RegisterName::Named('a'), RegisterName::Named('a'));
        assert_ne!(RegisterName::Named('a'), RegisterName::Named('b'));
    }

    #[test]
    fn test_register_name_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(RegisterName::Named('a'));
        assert!(set.contains(&RegisterName::Named('a')));
    }

    #[test]
    fn test_register_name_uppercase_normalizes() {
        // Uppercase letters should normalize to lowercase
        assert_eq!(RegisterName::from_char('Z'), Some(RegisterName::Named('z')));
        assert_eq!(RegisterName::from_char('M'), Some(RegisterName::Named('m')));
    }

    #[test]
    fn test_register_from_string() {
        let reg = Register::new(String::from("dynamic"), false);
        assert_eq!(reg.content, "dynamic");
    }

    #[test]
    fn test_register_name_all_named() {
        for c in 'a'..='z' {
            let name = RegisterName::from_char(c);
            assert!(matches!(name, Some(RegisterName::Named(_))));
        }
    }

    #[test]
    fn test_register_name_invalid_chars() {
        assert_eq!(RegisterName::from_char('!'), None);
        assert_eq!(RegisterName::from_char('@'), None);
        assert_eq!(RegisterName::from_char('$'), None);
        assert_eq!(RegisterName::from_char('^'), None);
    }

    #[test]
    fn test_register_clone_content() {
        let reg = Register::new("hello", true);
        let cloned = reg.clone();
        assert_eq!(reg.content, cloned.content);
        assert_eq!(reg.linewise, cloned.linewise);
    }

    #[test]
    fn test_register_equality() {
        let reg1 = Register::new("hello", true);
        let reg2 = Register::new("hello", true);
        let reg3 = Register::new("hello", false);
        assert_eq!(reg1, reg2);
        assert_ne!(reg1, reg3);
    }

    #[test]
    fn test_register_debug() {
        let reg = Register::new("test", false);
        let debug = format!("{:?}", reg);
        assert!(debug.contains("Register"));
    }

    #[test]
    fn test_register_name_copy() {
        let name = RegisterName::Named('x');
        let copied = name; // Copy trait
        assert_eq!(name, copied);
    }

    #[test]
    fn test_register_empty_content() {
        let reg = Register::new("", false);
        assert!(reg.content.is_empty());
    }

    #[test]
    fn test_register_unicode_content() {
        let reg = Register::new("你好世界🌍", false);
        assert_eq!(reg.content, "你好世界🌍");
    }

    #[test]
    fn test_register_name_filename_parse() {
        assert_eq!(RegisterName::from_char('%'), Some(RegisterName::Filename));
        assert_eq!(RegisterName::Filename.as_char(), '%');
    }

    #[test]
    fn test_register_name_default_value() {
        let name = RegisterName::default();
        assert_eq!(name, RegisterName::Unnamed);
    }
}
