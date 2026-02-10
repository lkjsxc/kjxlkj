//! Register set: named, numbered, and special registers.
//! Vim register model: `"` default, `a-z`/`A-Z` named, `0` yank, `1-9` delete
//! history, `-` small delete, `_` black hole, `/ : % . + *` specials.

use std::collections::HashMap;

/// Type of register content (affects paste behavior).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RegisterType {
    #[default]
    Charwise,
    Linewise,
}

/// A single register entry.
#[derive(Debug, Clone, Default)]
pub struct RegisterEntry {
    pub content: String,
    pub reg_type: RegisterType,
}

impl RegisterEntry {
    pub fn new(content: String, reg_type: RegisterType) -> Self {
        Self { content, reg_type }
    }
    pub fn charwise(content: String) -> Self {
        Self::new(content, RegisterType::Charwise)
    }
    pub fn linewise(content: String) -> Self {
        Self::new(content, RegisterType::Linewise)
    }
}

/// The full register set.
#[derive(Debug, Default)]
pub struct RegisterSet {
    named: HashMap<char, RegisterEntry>, // a-z
    yank: RegisterEntry,                 // "0
    numbered: [RegisterEntry; 9],        // 1-9
    small_delete: RegisterEntry,         // -
    last_insert: RegisterEntry,          // .
    last_command: RegisterEntry,         // :
    last_search: RegisterEntry,          // /
    current_file: RegisterEntry,         // %
    clipboard: RegisterEntry,            // +
    primary: RegisterEntry,              // *
}

impl RegisterSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read a register by name char.
    pub fn get(&self, name: char) -> &RegisterEntry {
        match name {
            '"' => self.get_unnamed(),
            '0' => &self.yank,
            '1'..='9' => {
                let idx = (name as u8 - b'1') as usize;
                &self.numbered[idx]
            }
            'a'..='z' => self.named.get(&name).unwrap_or(&self.yank),
            'A'..='Z' => {
                let lower = name.to_ascii_lowercase();
                self.named.get(&lower).unwrap_or(&self.yank)
            }
            '-' => &self.small_delete,
            '.' => &self.last_insert,
            ':' => &self.last_command,
            '/' => &self.last_search,
            '%' => &self.current_file,
            '+' => &self.clipboard,
            '*' => &self.primary,
            '_' => &EMPTY_REG,
            _ => &EMPTY_REG,
        }
    }

    /// Unnamed register — returns most recent yank/delete.
    fn get_unnamed(&self) -> &RegisterEntry {
        &self.yank
    }

    /// Store text from a yank. Updates unnamed and `0`.
    pub fn store_yank(&mut self, target: Option<char>, content: String, reg_type: RegisterType) {
        let entry = RegisterEntry::new(content, reg_type);
        self.yank = entry.clone();
        if let Some(name) = target {
            self.write_named(name, entry);
        }
    }

    /// Store text from a delete. Rotates `1-9` or sets `-`.
    pub fn store_delete(
        &mut self,
        target: Option<char>,
        content: String,
        reg_type: RegisterType,
        is_linewise: bool,
    ) {
        let entry = RegisterEntry::new(content, reg_type);
        if let Some(name) = target {
            if name == '_' {
                return; // black hole — discard
            }
            self.write_named(name, entry.clone());
        }
        if is_linewise {
            // Rotate numbered registers 1-9
            self.rotate_numbered(entry.clone());
        } else {
            // Small delete
            self.small_delete = entry.clone();
        }
        // Always update the yank/unnamed with the most recent
        self.yank = entry;
    }

    /// Rotate numbered registers: 1->2->...->9, new content into 1.
    fn rotate_numbered(&mut self, entry: RegisterEntry) {
        for i in (1..9).rev() {
            self.numbered[i] = self.numbered[i - 1].clone();
        }
        self.numbered[0] = entry;
    }

    /// Write to a named register (a-z replaces, A-Z appends).
    fn write_named(&mut self, name: char, entry: RegisterEntry) {
        if name.is_ascii_lowercase() {
            self.named.insert(name, entry);
        } else if name.is_ascii_uppercase() {
            let lower = name.to_ascii_lowercase();
            let existing = self.named.entry(lower).or_default();
            existing.content.push_str(&entry.content);
            existing.reg_type = entry.reg_type;
        }
    }

    pub fn set_last_insert(&mut self, text: String) {
        self.last_insert = RegisterEntry::charwise(text);
    }
    pub fn set_last_command(&mut self, cmd: String) {
        self.last_command = RegisterEntry::charwise(cmd);
    }
    pub fn set_last_search(&mut self, pattern: String) {
        self.last_search = RegisterEntry::charwise(pattern);
    }
    pub fn set_current_file(&mut self, path: String) {
        self.current_file = RegisterEntry::charwise(path);
    }

    /// Read content (empty for black hole).
    pub fn read(&self, name: char) -> &str {
        if name == '_' {
            return "";
        }
        &self.get(name).content
    }
    /// Read register type.
    pub fn read_type(&self, name: char) -> RegisterType {
        self.get(name).reg_type
    }
}

static EMPTY_REG: RegisterEntry = RegisterEntry {
    content: String::new(),
    reg_type: RegisterType::Charwise,
};
