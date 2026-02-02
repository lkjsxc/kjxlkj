//! Register storage.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

/// Register storage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registers {
    /// Named registers.
    named: HashMap<char, RegisterContent>,
    /// Unnamed register (").
    unnamed: Option<RegisterContent>,
    /// Small delete register (-).
    small_delete: Option<RegisterContent>,
    /// Numbered registers (0-9).
    numbered: [Option<RegisterContent>; 10],
}

impl Registers {
    /// Creates new register storage.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a register.
    pub fn get(&self, name: char) -> Option<&RegisterContent> {
        match name {
            '"' => self.unnamed.as_ref(),
            '-' => self.small_delete.as_ref(),
            '0'..='9' => {
                let idx = (name as u8 - b'0') as usize;
                self.numbered[idx].as_ref()
            }
            _ => self.named.get(&name),
        }
    }

    /// Sets a register.
    pub fn set(&mut self, name: char, content: RegisterContent) {
        match name {
            '"' => self.unnamed = Some(content),
            '-' => self.small_delete = Some(content),
            '0'..='9' => {
                let idx = (name as u8 - b'0') as usize;
                self.numbered[idx] = Some(content);
            }
            _ => {
                self.named.insert(name, content);
            }
        }
    }

    /// Gets the unnamed register.
    pub fn unnamed(&self) -> Option<&RegisterContent> {
        self.unnamed.as_ref()
    }

    /// Sets the unnamed register.
    pub fn set_unnamed(&mut self, content: RegisterContent) {
        self.unnamed = Some(content);
    }
}
