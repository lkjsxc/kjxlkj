//! Keybinding action tables for normal mode.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Categories of editor actions for bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionCategory {
    Motion,
    Operator,
    ModeSwitch,
    Command,
    Search,
    Scroll,
    Mark,
    Register,
    Macro,
    TextObject,
    Repeat,
    Window,
}

/// A single keybinding entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingEntry {
    pub key: String,
    pub action: String,
    pub category: ActionCategory,
    pub description: String,
}

/// A table of keybindings.
#[derive(Debug, Clone, Default)]
pub struct BindingTable {
    pub entries: Vec<BindingEntry>,
}

impl BindingTable {
    pub fn add(&mut self, key: &str, action: &str, cat: ActionCategory, desc: &str) {
        self.entries.push(BindingEntry {
            key: key.into(),
            action: action.into(),
            category: cat,
            description: desc.into(),
        });
    }

    pub fn find_by_key(&self, key: &str) -> Option<&BindingEntry> {
        self.entries.iter().find(|e| e.key == key)
    }

    pub fn find_by_category(&self, cat: ActionCategory) -> Vec<&BindingEntry> {
        self.entries.iter().filter(|e| e.category == cat).collect()
    }
}

/// Coverage statistics: count of bindings per category.
pub fn coverage_stats(table: &BindingTable) -> HashMap<ActionCategory, usize> {
    let mut m = HashMap::new();
    for e in &table.entries {
        *m.entry(e.category).or_insert(0) += 1;
    }
    m
}

// Re-export build function from dedicated module.
pub use crate::keybinding_tables_build::build_normal_table;
