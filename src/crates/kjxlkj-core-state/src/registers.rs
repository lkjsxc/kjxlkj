//! Register storage.

use kjxlkj_core_types::{Register, RegisterName};
use std::collections::HashMap;

/// Store for registers.
#[derive(Debug, Clone, Default)]
pub struct RegisterStore {
    /// Named registers.
    registers: HashMap<RegisterName, Register>,
    /// Last used register for yank/delete.
    last_register: RegisterName,
}

impl RegisterStore {
    /// Create a new register store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a register.
    pub fn get(&self, name: RegisterName) -> Option<&Register> {
        self.registers.get(&name)
    }

    /// Set a register.
    pub fn set(&mut self, name: RegisterName, content: String, linewise: bool) {
        if matches!(name, RegisterName::BlackHole) {
            return;
        }
        let reg = Register::with_content(content, linewise);
        self.registers.insert(name, reg);

        // Also set unnamed register for most operations
        if !matches!(name, RegisterName::Unnamed) {
            let reg_copy = self.registers.get(&name).cloned().unwrap();
            self.registers.insert(RegisterName::Unnamed, reg_copy);
        }
    }

    /// Get the unnamed register.
    pub fn unnamed(&self) -> Option<&Register> {
        self.get(RegisterName::Unnamed)
    }

    /// Set the last used register.
    pub fn set_last(&mut self, name: RegisterName) {
        self.last_register = name;
    }

    /// Get the last used register name.
    pub fn last(&self) -> RegisterName {
        self.last_register
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_set_get() {
        let mut store = RegisterStore::new();
        store.set(
            RegisterName::Named('a'),
            "hello".to_string(),
            false,
        );
        let reg = store.get(RegisterName::Named('a')).unwrap();
        assert_eq!(reg.content, "hello");
        assert!(!reg.linewise);
    }

    #[test]
    fn black_hole_register() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::BlackHole, "text".to_string(), false);
        assert!(store.get(RegisterName::BlackHole).is_none());
    }

    #[test]
    fn unnamed_register_updated() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Named('a'), "test".to_string(), false);
        let unnamed = store.unnamed().unwrap();
        assert_eq!(unnamed.content, "test");
    }

    #[test]
    fn linewise_register() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Named('b'), "line\n".to_string(), true);
        let reg = store.get(RegisterName::Named('b')).unwrap();
        assert!(reg.linewise);
    }

    #[test]
    fn overwrite_register() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Named('c'), "first".to_string(), false);
        store.set(RegisterName::Named('c'), "second".to_string(), false);
        let reg = store.get(RegisterName::Named('c')).unwrap();
        assert_eq!(reg.content, "second");
    }

    #[test]
    fn last_register_tracking() {
        let mut store = RegisterStore::new();
        store.set_last(RegisterName::Named('x'));
        assert_eq!(store.last(), RegisterName::Named('x'));
    }

    #[test]
    fn get_nonexistent_returns_none() {
        let store = RegisterStore::new();
        assert!(store.get(RegisterName::Named('z')).is_none());
    }
}
