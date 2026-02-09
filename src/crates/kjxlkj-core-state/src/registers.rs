//! Named registers for yank/delete/paste operations.
//!
//! Implements the register file with named, numbered, and special registers.

use std::collections::HashMap;

/// Register content type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterContent {
    /// Character-wise register content.
    Chars(String),
    /// Line-wise register content (includes trailing newline semantics).
    Lines(Vec<String>),
    /// Block-wise register content (visual block yank).
    Block(Vec<String>),
}

impl RegisterContent {
    /// Get the text content as a single string.
    pub fn to_string_content(&self) -> String {
        match self {
            Self::Chars(s) => s.clone(),
            Self::Lines(lines) => {
                let mut s = lines.join("\n");
                s.push('\n');
                s
            }
            Self::Block(lines) => lines.join("\n"),
        }
    }

    /// Check if content is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Chars(s) => s.is_empty(),
            Self::Lines(l) => l.is_empty(),
            Self::Block(l) => l.is_empty(),
        }
    }

    /// Whether this is line-wise content.
    pub fn is_linewise(&self) -> bool {
        matches!(self, Self::Lines(_))
    }
}

/// The register file.
#[derive(Debug, Clone)]
pub struct RegisterFile {
    /// Named registers: a-z, plus special registers.
    named: HashMap<char, RegisterContent>,
    /// Numbered registers 0-9 (0 = last yank, 1-9 = delete history).
    numbered: [Option<RegisterContent>; 10],
    /// Small delete register (deletes less than one line).
    small_delete: Option<RegisterContent>,
    /// Last search pattern register.
    search: Option<String>,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            named: HashMap::new(),
            numbered: Default::default(),
            small_delete: None,
            search: None,
        }
    }

    /// Get content of a register.
    pub fn get(&self, name: char) -> Option<&RegisterContent> {
        match name {
            '0'..='9' => {
                let idx = (name as u8 - b'0') as usize;
                self.numbered[idx].as_ref()
            }
            '-' => self.small_delete.as_ref(),
            '"' => {
                // Unnamed register: last yank or delete
                self.numbered[0]
                    .as_ref()
                    .or(self.numbered[1].as_ref())
                    .or(self.small_delete.as_ref())
            }
            _ => self.named.get(&name),
        }
    }

    /// Set content of a named register.
    pub fn set(&mut self, name: char, content: RegisterContent) {
        match name {
            '0'..='9' => {
                let idx = (name as u8 - b'0') as usize;
                self.numbered[idx] = Some(content);
            }
            '-' => {
                self.small_delete = Some(content);
            }
            'A'..='Z' => {
                // Uppercase appends to lowercase register
                let lower = name.to_ascii_lowercase();
                let entry = self
                    .named
                    .entry(lower)
                    .or_insert(RegisterContent::Chars(String::new()));
                match (entry, &content) {
                    (RegisterContent::Chars(existing), RegisterContent::Chars(new)) => {
                        existing.push_str(new);
                    }
                    (RegisterContent::Lines(existing), RegisterContent::Lines(new)) => {
                        existing.extend(new.iter().cloned());
                    }
                    (dest, _) => {
                        *dest = content;
                    }
                }
            }
            _ => {
                self.named.insert(name, content);
            }
        }
    }

    /// Record a yank operation (stores in register 0).
    pub fn yank(&mut self, content: RegisterContent) {
        self.numbered[0] = Some(content);
    }

    /// Record a delete operation (rotates numbered registers 1-9).
    pub fn delete(&mut self, content: RegisterContent, is_small: bool) {
        if is_small {
            self.small_delete = Some(content);
        } else {
            // Shift 1→2, 2→3, ..., 8→9
            for i in (2..=9).rev() {
                self.numbered[i] = self.numbered[i - 1].take();
            }
            self.numbered[1] = Some(content);
        }
    }

    /// Set the search register.
    pub fn set_search(&mut self, pattern: String) {
        self.search = Some(pattern);
    }

    /// Get the search register.
    pub fn get_search(&self) -> Option<&str> {
        self.search.as_deref()
    }

    /// Get content of the unnamed register (").
    pub fn unnamed(&self) -> Option<&RegisterContent> {
        self.get('"')
    }

    /// List all non-empty registers.
    pub fn list_nonempty(&self) -> Vec<(String, &RegisterContent)> {
        let mut result = Vec::new();

        // Numbered
        for i in 0..=9 {
            if let Some(ref content) = self.numbered[i] {
                if !content.is_empty() {
                    result.push((format!("{i}"), content));
                }
            }
        }

        // Small delete
        if let Some(ref content) = self.small_delete {
            if !content.is_empty() {
                result.push(("-".to_string(), content));
            }
        }

        // Named (sorted)
        let mut names: Vec<_> = self.named.keys().collect();
        names.sort();
        for &name in &names {
            if let Some(content) = self.named.get(name) {
                if !content.is_empty() {
                    result.push((name.to_string(), content));
                }
            }
        }

        result
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}
