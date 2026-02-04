//! Register storage.

use kjxlkj_core_types::{Register, RegisterName};
use std::collections::HashMap;

/// Storage for all registers.
#[derive(Debug, Clone, Default)]
pub struct RegisterStore {
    registers: HashMap<RegisterName, Register>,
    last_search: String,
}

impl RegisterStore {
    /// Create empty register store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a register's contents.
    pub fn get(&self, name: RegisterName) -> Option<&Register> {
        if name == RegisterName::BlackHole {
            return None;
        }
        self.registers.get(&name)
    }

    /// Set a register's contents.
    pub fn set(&mut self, name: RegisterName, reg: Register) {
        if name == RegisterName::BlackHole {
            return;
        }
        // Also update unnamed register
        if !matches!(name, RegisterName::Unnamed) {
            self.registers.insert(RegisterName::Unnamed, reg.clone());
        }
        self.registers.insert(name, reg);
    }

    /// Get the unnamed register.
    pub fn unnamed(&self) -> Option<&Register> {
        self.get(RegisterName::Unnamed)
    }

    /// Set the last search pattern.
    pub fn set_search(&mut self, pattern: String) {
        self.last_search = pattern;
    }

    /// Get the last search pattern.
    pub fn search(&self) -> &str {
        &self.last_search
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_store() {
        let mut store = RegisterStore::new();
        let reg = Register::new("hello".to_string(), false);
        store.set(RegisterName::Named('a'), reg);

        let retrieved = store.get(RegisterName::Named('a')).unwrap();
        assert_eq!(retrieved.content, "hello");

        // Unnamed should also be updated
        let unnamed = store.unnamed().unwrap();
        assert_eq!(unnamed.content, "hello");
    }

    #[test]
    fn test_black_hole_register() {
        let mut store = RegisterStore::new();
        store.set(
            RegisterName::BlackHole,
            Register::new("test".to_string(), false),
        );
        assert!(store.get(RegisterName::BlackHole).is_none());
    }
}
