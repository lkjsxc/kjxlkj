//! Register storage.

use kjxlkj_core_types::{Register, RegisterName};
use std::collections::HashMap;

/// Register storage for yank/paste operations.
#[derive(Debug, Clone, Default)]
pub struct Registers {
    /// Named registers.
    registers: HashMap<RegisterName, Register>,
    /// Currently selected register.
    selected: RegisterName,
    /// Last search pattern.
    search_pattern: Option<String>,
}

impl Registers {
    /// Create new register storage.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the currently selected register.
    pub fn selected(&self) -> RegisterName {
        self.selected
    }

    /// Select a register for the next operation.
    pub fn select(&mut self, name: RegisterName) {
        self.selected = name;
    }

    /// Reset to the default (unnamed) register.
    pub fn reset_selection(&mut self) {
        self.selected = RegisterName::Unnamed;
    }

    /// Get a register's content.
    pub fn get(&self, name: RegisterName) -> Option<&Register> {
        self.registers.get(&name)
    }

    /// Get the selected register's content.
    pub fn get_selected(&self) -> Option<&Register> {
        self.get(self.selected)
    }

    /// Set a register's content.
    pub fn set(&mut self, name: RegisterName, content: Register) {
        // Don't store to black hole register
        if name == RegisterName::BlackHole {
            return;
        }

        // Also update unnamed register for most operations
        if !matches!(
            name,
            RegisterName::Unnamed
                | RegisterName::Search
                | RegisterName::Command
                | RegisterName::Expression
        ) {
            self.registers
                .insert(RegisterName::Unnamed, content.clone());
        }

        self.registers.insert(name, content);
    }

    /// Set the selected register's content and reset selection.
    pub fn set_selected(&mut self, content: Register) {
        let name = self.selected;
        self.set(name, content);
        self.reset_selection();
    }

    /// Get the last search pattern.
    pub fn search_pattern(&self) -> Option<&str> {
        self.search_pattern.as_deref()
    }

    /// Set the search pattern.
    pub fn set_search_pattern(&mut self, pattern: String) {
        self.search_pattern = Some(pattern.clone());
        self.registers.insert(
            RegisterName::Search,
            Register::new(pattern, false),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_set_get() {
        let mut regs = Registers::new();
        regs.set(RegisterName::Named('a'), Register::new("hello", false));
        assert_eq!(regs.get(RegisterName::Named('a')).unwrap().content, "hello");
    }

    #[test]
    fn test_register_unnamed_update() {
        let mut regs = Registers::new();
        regs.set(RegisterName::Named('a'), Register::new("hello", false));
        assert_eq!(regs.get(RegisterName::Unnamed).unwrap().content, "hello");
    }

    #[test]
    fn test_register_black_hole() {
        let mut regs = Registers::new();
        regs.set(RegisterName::BlackHole, Register::new("gone", false));
        assert!(regs.get(RegisterName::BlackHole).is_none());
    }
}
