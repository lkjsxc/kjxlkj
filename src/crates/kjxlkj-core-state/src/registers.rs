//! Register storage.

use crate::register_types::RegisterContent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Register storage for yanked/deleted text.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registers {
    named: HashMap<char, RegisterContent>,  // Named a-z registers
    unnamed: Option<RegisterContent>,       // Unnamed register (")
    small_delete: Option<RegisterContent>,  // Small delete register (-)
    numbered: [Option<RegisterContent>; 10], // Numbered 0-9 registers
    search: Option<String>,                 // Last search pattern (/)
    last_insert: Option<String>,            // Last inserted text (.)
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
                // Uppercase appends
                if name.is_ascii_uppercase() {
                    let lower = name.to_ascii_lowercase();
                    if let Some(existing) = self.named.get_mut(&lower) {
                        existing.text.push_str(&content.text);
                    } else {
                        self.named.insert(lower, content);
                    }
                } else {
                    self.named.insert(name, content);
                }
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

    /// Yanks text - sets unnamed and register 0.
    pub fn yank(&mut self, content: RegisterContent) {
        self.numbered[0] = Some(content.clone());
        self.unnamed = Some(content);
    }

    /// Deletes text - updates numbered registers.
    pub fn delete(&mut self, content: RegisterContent) {
        // Shift numbered registers 1-9
        for i in (2..10).rev() {
            self.numbered[i] = self.numbered[i - 1].take();
        }
        self.numbered[1] = self.unnamed.take();

        // Small deletes (< 1 line) go to "-"
        if !content.text.contains('\n') && content.text.len() < 80 {
            self.small_delete = Some(content.clone());
        }

        self.unnamed = Some(content);
    }

    /// Gets the search register.
    pub fn search(&self) -> Option<&str> {
        self.search.as_deref()
    }

    /// Sets the search register.
    pub fn set_search(&mut self, pattern: String) {
        self.search = Some(pattern);
    }

    /// Gets the last insert register.
    pub fn last_insert(&self) -> Option<&str> {
        self.last_insert.as_deref()
    }

    /// Sets the last insert register.
    pub fn set_last_insert(&mut self, text: String) {
        self.last_insert = Some(text);
    }

    /// Returns all non-empty register names.
    pub fn list(&self) -> Vec<char> {
        let mut names: Vec<char> = vec![];
        if self.unnamed.is_some() {
            names.push('"');
        }
        if self.small_delete.is_some() {
            names.push('-');
        }
        for (i, reg) in self.numbered.iter().enumerate() {
            if reg.is_some() {
                names.push((b'0' + i as u8) as char);
            }
        }
        names.extend(self.named.keys().copied());
        names.sort();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_get_set() {
        let mut regs = Registers::new();
        regs.set('a', RegisterContent::char("hello"));
        assert_eq!(regs.get('a').unwrap().text, "hello");
    }

    #[test]
    fn test_unnamed_register() {
        let mut regs = Registers::new();
        regs.set_unnamed(RegisterContent::char("test"));
        assert_eq!(regs.get('"').unwrap().text, "test");
    }

    #[test]
    fn test_numbered_registers() {
        let mut regs = Registers::new();
        regs.set('0', RegisterContent::char("zero"));
        regs.set('5', RegisterContent::char("five"));
        assert_eq!(regs.get('0').unwrap().text, "zero");
        assert_eq!(regs.get('5').unwrap().text, "five");
    }

    #[test]
    fn test_yank() {
        let mut regs = Registers::new();
        regs.yank(RegisterContent::char("yanked"));
        assert_eq!(regs.get('0').unwrap().text, "yanked");
        assert_eq!(regs.unnamed().unwrap().text, "yanked");
    }

    #[test]
    fn test_delete_shifts_numbered() {
        let mut regs = Registers::new();
        regs.delete(RegisterContent::char("first"));
        regs.delete(RegisterContent::char("second"));
        assert_eq!(regs.get('1').unwrap().text, "first");
        assert_eq!(regs.unnamed().unwrap().text, "second");
    }

    #[test]
    fn test_uppercase_appends() {
        let mut regs = Registers::new();
        regs.set('a', RegisterContent::char("hello"));
        regs.set('A', RegisterContent::char(" world"));
        assert_eq!(regs.get('a').unwrap().text, "hello world");
    }

    #[test]
    fn test_register_list() {
        let mut regs = Registers::new();
        regs.set('a', RegisterContent::char("a"));
        regs.set('b', RegisterContent::char("b"));
        let list = regs.list();
        assert!(list.contains(&'a'));
        assert!(list.contains(&'b'));
    }
}
