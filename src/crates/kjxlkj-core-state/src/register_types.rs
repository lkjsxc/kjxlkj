//! Register types.

use serde::{Deserialize, Serialize};

/// Register type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisterType {
    /// Character-wise.
    Char,
    /// Line-wise.
    Line,
    /// Block-wise.
    Block,
}

/// Register content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterContent {
    /// Text content.
    pub text: String,
    /// Register type.
    pub reg_type: RegisterType,
}

impl RegisterContent {
    /// Creates a character-wise register.
    pub fn char(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            reg_type: RegisterType::Char,
        }
    }

    /// Creates a line-wise register.
    pub fn line(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            reg_type: RegisterType::Line,
        }
    }

    /// Creates a block-wise register.
    pub fn block(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            reg_type: RegisterType::Block,
        }
    }

    /// Returns if this is line-wise.
    pub fn is_linewise(&self) -> bool {
        self.reg_type == RegisterType::Line
    }

    /// Returns if this is block-wise.
    pub fn is_blockwise(&self) -> bool {
        self.reg_type == RegisterType::Block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_content_char() {
        let content = RegisterContent::char("hello");
        assert_eq!(content.text, "hello");
        assert_eq!(content.reg_type, RegisterType::Char);
    }

    #[test]
    fn test_register_content_line() {
        let content = RegisterContent::line("line text");
        assert!(content.is_linewise());
        assert!(!content.is_blockwise());
    }

    #[test]
    fn test_register_content_block() {
        let content = RegisterContent::block("block text");
        assert!(content.is_blockwise());
        assert!(!content.is_linewise());
    }

    #[test]
    fn test_register_type_eq() {
        assert_eq!(RegisterType::Char, RegisterType::Char);
        assert_ne!(RegisterType::Char, RegisterType::Line);
    }
}
