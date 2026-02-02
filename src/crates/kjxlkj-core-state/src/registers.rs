//! Register storage.

use std::collections::HashMap;
use kjxlkj_core_types::register::{Register, RegisterContent};

/// Stores register contents.
#[derive(Debug, Default)]
pub struct RegisterStore {
    /// Register contents by register.
    registers: HashMap<Register, RegisterContent>,
    /// Numbered delete history (for numbered registers).
    delete_history: Vec<RegisterContent>,
}

impl RegisterStore {
    /// Creates a new register store.
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
            delete_history: Vec::with_capacity(10),
        }
    }

    /// Gets register content.
    pub fn get(&self, register: Register) -> Option<&RegisterContent> {
        // For numbered registers 1-9, use delete history
        if let Register::Numbered(n) = register {
            if (1..=9).contains(&n) {
                return self.delete_history.get(n as usize - 1);
            }
        }
        self.registers.get(&register)
    }

    /// Sets register content.
    pub fn set(&mut self, register: Register, content: RegisterContent) {
        // Black hole register discards content
        if matches!(register, Register::BlackHole) {
            return;
        }

        // Read-only registers can't be written
        if register.is_readonly() {
            return;
        }

        // Numbered register 0 stores last yank
        if matches!(register, Register::Numbered(0)) {
            self.registers.insert(register, content);
            return;
        }

        // For unnamed register and deletes, update numbered registers
        if matches!(register, Register::Unnamed) {
            // Shift numbered registers down
            if self.delete_history.len() >= 9 {
                self.delete_history.pop();
            }
            self.delete_history.insert(0, content.clone());
        }

        self.registers.insert(register, content);
    }

    /// Sets content for yank operations (updates register 0).
    pub fn set_yank(&mut self, register: Register, content: RegisterContent) {
        // Always update register 0 for yanks
        self.registers
            .insert(Register::Numbered(0), content.clone());
        self.set(register, content);
    }

    /// Sets content for delete operations.
    pub fn set_delete(&mut self, register: Register, content: RegisterContent) {
        // Small deletes go to small delete register
        let is_small = !content.text.contains('\n')
            && content.text.chars().count() < 80;

        if is_small && matches!(register, Register::Unnamed) {
            self.registers.insert(Register::SmallDelete, content.clone());
        }

        self.set(register, content);
    }

    /// Clears a register.
    pub fn clear(&mut self, register: Register) {
        self.registers.remove(&register);
    }

    /// Clears all registers.
    pub fn clear_all(&mut self) {
        self.registers.clear();
        self.delete_history.clear();
    }
}
