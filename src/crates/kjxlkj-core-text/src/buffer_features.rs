//! Buffer-local features: variables, options, file format, autocmd.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Buffer-local variables (key-value string storage).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BufferVariables {
    map: HashMap<String, String>,
}

impl BufferVariables {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }

    pub fn keys(&self) -> Vec<&str> {
        self.map.keys().map(|k| k.as_str()).collect()
    }
}

/// Buffer-local editor options.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BufferLocalOptions {
    pub tabstop: Option<usize>,
    pub shiftwidth: Option<usize>,
    pub expandtab: Option<bool>,
    pub textwidth: Option<usize>,
}

/// File format (line-ending style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileFormat {
    Unix,
    Dos,
    Mac,
}

impl FileFormat {
    /// Return the line ending string for this format.
    pub fn line_ending(self) -> &'static str {
        match self {
            Self::Unix => "\n",
            Self::Dos => "\r\n",
            Self::Mac => "\r",
        }
    }
}

impl Default for FileFormat {
    fn default() -> Self {
        Self::Unix
    }
}

/// Buffer-related events for autocmd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BufEvent {
    BufEnter,
    BufLeave,
    BufRead,
    BufWrite,
    BufNew,
    BufDelete,
    WinEnter,
    WinLeave,
    Modified,
}

/// A registered autocmd entry.
#[derive(Debug, Clone)]
struct AutoCmd {
    event: BufEvent,
    pattern: String,
    command: String,
}

/// Registry for buffer-local autocommands.
#[derive(Debug, Clone, Default)]
pub struct AutoCmdRegistry {
    entries: Vec<AutoCmd>,
}

impl AutoCmdRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new autocmd.
    pub fn register(&mut self, event: BufEvent, pattern: String, command: String) {
        self.entries.push(AutoCmd {
            event,
            pattern,
            command,
        });
    }

    /// Query commands matching event and filename pattern.
    pub fn query(&self, event: BufEvent, filename: &str) -> Vec<String> {
        self.entries
            .iter()
            .filter(|ac| {
                ac.event == event && Self::pattern_matches(&ac.pattern, filename)
            })
            .map(|ac| ac.command.clone())
            .collect()
    }

    /// Remove all autocmds matching the given event.
    pub fn remove(&mut self, event: BufEvent) {
        self.entries.retain(|ac| ac.event != event);
    }

    /// Clear all autocmds.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Simple glob-like pattern matching: `*` matches everything,
    /// `*.ext` matches suffix, otherwise exact match.
    fn pattern_matches(pattern: &str, filename: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        if let Some(ext) = pattern.strip_prefix('*') {
            return filename.ends_with(ext);
        }
        pattern == filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_variables() {
        let mut v = BufferVariables::new();
        v.set("ft".into(), "rust".into());
        assert_eq!(v.get("ft"), Some("rust"));
        v.remove("ft");
        assert_eq!(v.get("ft"), None);
    }

    #[test]
    fn file_format_endings() {
        assert_eq!(FileFormat::Unix.line_ending(), "\n");
        assert_eq!(FileFormat::Dos.line_ending(), "\r\n");
        assert_eq!(FileFormat::Mac.line_ending(), "\r");
    }

    #[test]
    fn autocmd_registry() {
        let mut reg = AutoCmdRegistry::new();
        reg.register(BufEvent::BufRead, "*.rs".into(), "set ft=rust".into());
        let cmds = reg.query(BufEvent::BufRead, "main.rs");
        assert_eq!(cmds, vec!["set ft=rust"]);
        assert!(reg.query(BufEvent::BufWrite, "main.rs").is_empty());
        reg.remove(BufEvent::BufRead);
        assert!(reg.query(BufEvent::BufRead, "main.rs").is_empty());
    }
}
