//! Register file: storage for all named/numbered registers.

use std::collections::HashMap;
use kjxlkj_core_types::{RegisterContent, RegisterName, RegisterType};

/// Manages all editor registers.
pub struct RegisterFile {
    regs: HashMap<RegisterName, RegisterContent>,
    selected: Option<RegisterName>,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            regs: HashMap::new(),
            selected: None,
        }
    }

    /// Select a register for the next operation.
    pub fn select(&mut self, name: RegisterName) {
        self.selected = Some(name);
    }

    /// Take the selected register (consuming the selection).
    pub fn take_selected(&mut self) -> Option<RegisterName> {
        self.selected.take()
    }

    /// Get the content of a register.
    pub fn get(&self, name: RegisterName) -> Option<&RegisterContent> {
        self.regs.get(&name)
    }

    /// Set the content of a register (writes cascade to unnamed).
    pub fn set(&mut self, name: RegisterName, content: RegisterContent) {
        if name.is_readonly() {
            return;
        }
        if name == RegisterName::BlackHole {
            return;
        }
        // Also update unnamed register for non-special writes
        if !matches!(
            name,
            RegisterName::Unnamed | RegisterName::BlackHole
        ) {
            self.regs.insert(RegisterName::Unnamed, content.clone());
        }
        self.regs.insert(name, content);
    }

    /// Set the unnamed register (most common path).
    pub fn set_unnamed(&mut self, content: RegisterContent) {
        self.regs.insert(RegisterName::Unnamed, content);
    }

    /// Yank text into yank register and unnamed.
    pub fn yank(&mut self, text: &str, linewise: bool) {
        let content = if linewise {
            RegisterContent::linewise(text)
        } else {
            RegisterContent::charwise(text)
        };
        self.regs.insert(RegisterName::Yank, content.clone());
        self.regs.insert(RegisterName::Unnamed, content);
    }

    /// Delete text: push into numbered registers 1-9, set unnamed.
    pub fn delete(&mut self, text: &str, linewise: bool) {
        let content = if linewise {
            RegisterContent::linewise(text)
        } else {
            RegisterContent::charwise(text)
        };
        // Shift numbered regs 1→2, 2→3, ..., 8→9
        for i in (1..9).rev() {
            if let Some(c) = self.regs.get(&RegisterName::Numbered(i)).cloned() {
                self.regs.insert(RegisterName::Numbered(i + 1), c);
            }
        }
        self.regs.insert(RegisterName::Numbered(1), content.clone());
        self.regs.insert(RegisterName::Unnamed, content);
    }

    /// Get unnamed register's text.
    pub fn unnamed_text(&self) -> Option<&str> {
        self.regs.get(&RegisterName::Unnamed).map(|c| c.text.as_str())
    }

    /// Get unnamed register's type.
    pub fn unnamed_type(&self) -> Option<RegisterType> {
        self.regs.get(&RegisterName::Unnamed).map(|c| c.reg_type)
    }

    /// Display all registers for :registers command.
    pub fn display(&self) -> String {
        let mut lines = Vec::new();
        let mut entries: Vec<_> = self.regs.iter().collect();
        entries.sort_by_key(|(name, _)| format!("{:?}", name));
        for (name, content) in entries {
            let display_text: String = content
                .text
                .chars()
                .take(50)
                .collect();
            let type_char = match content.reg_type {
                RegisterType::Charwise => 'c',
                RegisterType::Linewise => 'l',
                RegisterType::Blockwise => 'b',
            };
            lines.push(format!(
                "  {:?}  {}  {}",
                name, type_char, display_text,
            ));
        }
        if lines.is_empty() {
            String::new()
        } else {
            format!(
                "--- Registers ---\n{}",
                lines.join("\n")
            )
        }
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
    fn yank_sets_unnamed_and_yank() {
        let mut rf = RegisterFile::new();
        rf.yank("hello", false);
        assert_eq!(rf.unnamed_text(), Some("hello"));
        assert_eq!(
            rf.get(RegisterName::Yank).unwrap().text,
            "hello"
        );
    }

    #[test]
    fn delete_cascades_numbered() {
        let mut rf = RegisterFile::new();
        rf.delete("first", false);
        rf.delete("second", false);
        assert_eq!(rf.unnamed_text(), Some("second"));
        assert_eq!(
            rf.get(RegisterName::Numbered(1)).unwrap().text,
            "second"
        );
        assert_eq!(
            rf.get(RegisterName::Numbered(2)).unwrap().text,
            "first"
        );
    }

    #[test]
    fn readonly_register_not_writable() {
        let mut rf = RegisterFile::new();
        rf.set(
            RegisterName::LastInserted,
            RegisterContent::charwise("x"),
        );
        assert!(rf.get(RegisterName::LastInserted).is_none());
    }

    #[test]
    fn black_hole_discards() {
        let mut rf = RegisterFile::new();
        rf.set(
            RegisterName::BlackHole,
            RegisterContent::charwise("gone"),
        );
        assert!(rf.get(RegisterName::BlackHole).is_none());
    }
}
