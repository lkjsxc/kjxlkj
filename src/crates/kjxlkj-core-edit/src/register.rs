use std::collections::HashMap;

/// Named register identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterName {
    /// Named register a-z, A-Z
    Named(char),
    /// Unnamed register (default)
    Unnamed,
    /// Black hole register _
    BlackHole,
    /// System clipboard +
    Clipboard,
    /// Selection clipboard *
    Selection,
    /// Last search /
    LastSearch,
    /// Small delete -
    SmallDelete,
    /// Numbered 0-9
    Numbered(u8),
    /// Last inserted text .
    LastInserted,
    /// Current file name %
    FileName,
    /// Last command :
    LastCommand,
    /// Expression =
    Expression,
}

/// Content stored in a register.
#[derive(Debug, Clone)]
pub struct Register {
    pub content: String,
    pub linewise: bool,
}

impl Register {
    pub fn new(content: String, linewise: bool) -> Self {
        Self { content, linewise }
    }
}

/// Collection of all registers.
#[derive(Debug, Clone)]
pub struct RegisterFile {
    registers: HashMap<RegisterName, Register>,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
        }
    }

    pub fn get(&self, name: RegisterName) -> Option<&Register> {
        self.registers.get(&name)
    }

    pub fn set(&mut self, name: RegisterName, reg: Register) {
        if matches!(name, RegisterName::BlackHole) {
            return; // black hole discards
        }
        self.registers.insert(name, reg);
    }

    /// Set the unnamed register.
    pub fn set_unnamed(&mut self, content: String, linewise: bool) {
        self.set(RegisterName::Unnamed, Register::new(content, linewise));
    }

    /// Get unnamed register content.
    pub fn get_unnamed(&self) -> Option<&Register> {
        self.get(RegisterName::Unnamed)
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}
