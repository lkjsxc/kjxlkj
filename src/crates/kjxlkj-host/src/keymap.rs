//! Keybinding configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Keybindings configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Keymap {
    /// Normal mode bindings.
    pub normal: HashMap<String, String>,
    /// Insert mode bindings.
    pub insert: HashMap<String, String>,
    /// Visual mode bindings.
    pub visual: HashMap<String, String>,
    /// Command mode bindings.
    pub command: HashMap<String, String>,
}

impl Keymap {
    /// Gets the action for a key in the given mode.
    pub fn lookup(&self, mode: &str, key: &str) -> Option<&String> {
        match mode {
            "normal" => self.normal.get(key),
            "insert" => self.insert.get(key),
            "visual" => self.visual.get(key),
            "command" => self.command.get(key),
            _ => None,
        }
    }

    /// Creates default keybindings.
    pub fn defaults() -> Self {
        let mut normal = HashMap::new();
        normal.insert("<Esc>".to_string(), "normal".to_string());
        normal.insert("i".to_string(), "insert".to_string());
        normal.insert(":".to_string(), "command".to_string());

        Self {
            normal,
            insert: HashMap::new(),
            visual: HashMap::new(),
            command: HashMap::new(),
        }
    }
}
