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

    #[test]
    fn last_register_default() {
        let store = RegisterStore::new();
        assert_eq!(store.last(), RegisterName::Unnamed);
    }

    #[test]
    fn set_unnamed_directly() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Unnamed, "direct".to_string(), false);
        let reg = store.unnamed().unwrap();
        assert_eq!(reg.content, "direct");
    }

    #[test]
    fn numbered_register() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Numbered(5), "num".to_string(), false);
        let reg = store.get(RegisterName::Numbered(5)).unwrap();
        assert_eq!(reg.content, "num");
    }

    #[test]
    fn last_search_register() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::LastSearch, "pattern".to_string(), false);
        let reg = store.get(RegisterName::LastSearch).unwrap();
        assert_eq!(reg.content, "pattern");
    }

    #[test]
    fn small_delete_register() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::SmallDelete, "del".to_string(), false);
        let reg = store.get(RegisterName::SmallDelete).unwrap();
        assert_eq!(reg.content, "del");
    }

    #[test]
    fn overwrite_unnamed() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Unnamed, "first".to_string(), false);
        store.set(RegisterName::Unnamed, "second".to_string(), false);
        assert_eq!(store.unnamed().unwrap().content, "second");
    }

    #[test]
    fn multiple_named_registers() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Named('a'), "aa".to_string(), false);
        store.set(RegisterName::Named('b'), "bb".to_string(), false);
        assert_eq!(store.get(RegisterName::Named('a')).unwrap().content, "aa");
        assert_eq!(store.get(RegisterName::Named('b')).unwrap().content, "bb");
    }

    #[test]
    fn set_linewise() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Unnamed, "line".to_string(), true);
        assert!(store.unnamed().unwrap().linewise);
    }

    #[test]
    fn set_not_linewise() {
        let mut store = RegisterStore::new();
        store.set(RegisterName::Unnamed, "text".to_string(), false);
        assert!(!store.unnamed().unwrap().linewise);
    }

    #[test]
    fn register_cloneable() {
        let reg = Register::with_content("hello".to_string(), false);
        let cloned = reg.clone();
        assert_eq!(cloned.content, "hello");
    }

    #[test]
    fn register_new_empty() {
        let reg = Register::new();
        assert!(reg.content.is_empty());
    }
}
