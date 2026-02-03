//! Register storage.

use std::collections::HashMap;

use kjxlkj_core_types::{RegisterContent, RegisterName};

/// Register storage.
#[derive(Debug, Default)]
pub struct RegisterStore {
    registers: HashMap<RegisterName, RegisterContent>,
}

impl RegisterStore {
    /// Create a new register store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a register's content.
    pub fn get(&self, name: RegisterName) -> Option<&RegisterContent> {
        // Black hole register never returns content
        if name == RegisterName::BlackHole {
            return None;
        }
        self.registers.get(&name)
    }

    /// Set a register's content.
    pub fn set(&mut self, name: RegisterName, content: RegisterContent) {
        // Black hole register discards content
        if name == RegisterName::BlackHole {
            return;
        }

        // Uppercase named registers append
        if let RegisterName::Named(c) = name {
            if c.is_ascii_uppercase() {
                let lower = RegisterName::Named(c.to_ascii_lowercase());
                if let Some(existing) = self.registers.get_mut(&lower) {
                    existing.text.push_str(&content.text);
                    return;
                }
            }
        }

        self.registers.insert(name, content);
    }

    /// Clear all registers.
    pub fn clear(&mut self) {
        self.registers.clear();
    }
}
