//! Register file: storage and retrieval for all register types.

use std::collections::HashMap;

use kjxlkj_core_types::{Register, RegisterName};

/// The register file holds all registers.
///
/// Per /docs/spec/editing/registers/README.md, the editor maintains
/// named (a-z), numbered (0-9), and special registers.
pub struct RegisterFile {
    /// Named and special registers.
    registers: HashMap<RegisterName, Register>,
    /// Last used register for `@@`.
    last_macro_register: Option<char>,
}

impl RegisterFile {
    /// Create a new empty register file.
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
            last_macro_register: None,
        }
    }

    /// Store content into a register.
    pub fn store(
        &mut self,
        name: RegisterName,
        content: String,
        linewise: bool,
    ) {
        if name == RegisterName::BlackHole {
            return; // Black hole discards
        }
        if name.is_readonly() {
            return; // Cannot write to read-only registers
        }

        let reg = Register::new(content.clone(), linewise);
        self.registers.insert(name, reg.clone());

        // Also update the unnamed register (unless explicitly using a named one)
        if !matches!(name, RegisterName::Unnamed) {
            self.registers.insert(RegisterName::Unnamed, reg.clone());
        }

        // For delete/yank, update numbered registers
        if matches!(name, RegisterName::Unnamed) {
            self.rotate_numbered(content, linewise);
        }
    }

    /// Get the content of a register.
    pub fn get(&self, name: RegisterName) -> Option<&Register> {
        self.registers.get(&name)
    }

    /// Get content of the unnamed register.
    pub fn unnamed(&self) -> Option<&Register> {
        self.registers.get(&RegisterName::Unnamed)
    }

    /// Set the last macro register for `@@`.
    pub fn set_last_macro(&mut self, reg: char) {
        self.last_macro_register = Some(reg);
    }

    /// Get the last macro register.
    pub fn last_macro(&self) -> Option<char> {
        self.last_macro_register
    }

    /// Append to a named register (uppercase letter).
    pub fn append(
        &mut self,
        name: RegisterName,
        content: &str,
        linewise: bool,
    ) {
        if let Some(existing) = self.registers.get_mut(&name) {
            existing.content.push_str(content);
            existing.linewise = existing.linewise || linewise;
        } else {
            self.store(name, content.to_string(), linewise);
        }
    }

    /// Rotate numbered registers (0→1→2→...→9).
    fn rotate_numbered(&mut self, content: String, linewise: bool) {
        // Shift 8→9, 7→8, ..., 1→2
        for i in (1..9).rev() {
            let src = RegisterName::Numbered(i);
            let dst = RegisterName::Numbered(i + 1);
            if let Some(reg) = self.registers.get(&src).cloned() {
                self.registers.insert(dst, reg);
            }
        }
        // 0→1 only for delete (not yank)
        if let Some(reg) = self.registers.get(&RegisterName::Numbered(0)).cloned() {
            self.registers.insert(RegisterName::Numbered(1), reg);
        }
        // New content → 0
        self.registers.insert(
            RegisterName::Numbered(0),
            Register::new(content, linewise),
        );
    }

    /// Clear all registers.
    pub fn clear(&mut self) {
        self.registers.clear();
        self.last_macro_register = None;
    }

    /// Get mutable access to registers map.
    pub fn registers_mut(
        &mut self,
    ) -> &mut HashMap<RegisterName, Register> {
        &mut self.registers
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_and_get() {
        let mut rf = RegisterFile::new();
        rf.store(RegisterName::Named('a'), "hello".into(), false);
        let reg = rf.get(RegisterName::Named('a')).unwrap();
        assert_eq!(reg.content, "hello");
        assert!(!reg.linewise);
    }

    #[test]
    fn black_hole_discards() {
        let mut rf = RegisterFile::new();
        rf.store(RegisterName::BlackHole, "discard".into(), false);
        assert!(rf.get(RegisterName::BlackHole).is_none());
    }

    #[test]
    fn unnamed_updated() {
        let mut rf = RegisterFile::new();
        rf.store(RegisterName::Named('a'), "text".into(), false);
        let unnamed = rf.unnamed().unwrap();
        assert_eq!(unnamed.content, "text");
    }

    #[test]
    fn numbered_rotation() {
        let mut rf = RegisterFile::new();
        rf.store(RegisterName::Unnamed, "first".into(), false);
        rf.store(RegisterName::Unnamed, "second".into(), false);
        let r0 = rf.get(RegisterName::Numbered(0)).unwrap();
        assert_eq!(r0.content, "second");
        let r1 = rf.get(RegisterName::Numbered(1)).unwrap();
        assert_eq!(r1.content, "first");
    }
}
