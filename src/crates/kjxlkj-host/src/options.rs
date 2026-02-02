//! Editor options (Vim-style :set options).
//!
//! Provides a configurable options system with global, buffer-local,
//! and window-local scopes.

mod options_defaults;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An option value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionValue {
    /// Boolean option.
    Bool(bool),
    /// Integer option.
    Int(i64),
    /// String option.
    String(String),
}

impl OptionValue {
    /// Returns the boolean value or default.
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Int(i) => *i != 0,
            Self::String(s) => !s.is_empty() && s != "0" && s != "false",
        }
    }

    /// Returns the integer value or default.
    pub fn as_int(&self) -> i64 {
        match self {
            Self::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            Self::Int(i) => *i,
            Self::String(s) => s.parse().unwrap_or(0),
        }
    }

    /// Returns the string value.
    pub fn as_str(&self) -> String {
        match self {
            Self::Bool(b) => b.to_string(),
            Self::Int(i) => i.to_string(),
            Self::String(s) => s.clone(),
        }
    }
}

/// Option scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionScope {
    /// Global option.
    Global,
    /// Buffer-local option.
    Buffer,
    /// Window-local option.
    Window,
}

/// Metadata for an option.
#[derive(Debug, Clone)]
pub struct OptionMeta {
    /// Full name.
    pub name: &'static str,
    /// Short name (abbreviation).
    pub short: Option<&'static str>,
    /// Description.
    pub description: &'static str,
    /// Default value.
    pub default: OptionValue,
    /// Scope.
    pub scope: OptionScope,
}

/// Built-in options registry.
pub struct Options {
    /// Global option values.
    global: HashMap<String, OptionValue>,
    /// Metadata for known options.
    meta: HashMap<String, OptionMeta>,
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    /// Creates a new options registry with defaults.
    pub fn new() -> Self {
        let mut opts = Self {
            global: HashMap::new(),
            meta: HashMap::new(),
        };
        options_defaults::register_defaults(&mut opts);
        opts
    }

    fn register(&mut self, meta: OptionMeta) {
        let name = meta.name.to_string();
        self.global.insert(name.clone(), meta.default.clone());
        if let Some(short) = meta.short {
            self.meta.insert(short.to_string(), meta.clone());
        }
        self.meta.insert(name, meta);
    }

    /// Gets an option value.
    pub fn get(&self, name: &str) -> Option<&OptionValue> {
        self.global
            .get(name)
            .or_else(|| self.meta.get(name).map(|m| &m.default))
    }

    /// Sets an option value.
    pub fn set(&mut self, name: &str, value: OptionValue) -> bool {
        let canonical = self.canonical_name(name).map(|s| s.to_string());
        if let Some(name) = canonical {
            self.global.insert(name, value);
            true
        } else {
            false
        }
    }

    /// Toggles a boolean option.
    pub fn toggle(&mut self, name: &str) -> bool {
        if let Some(val) = self.get(name).cloned() {
            let new_val = !val.as_bool();
            self.set(name, OptionValue::Bool(new_val))
        } else {
            false
        }
    }

    /// Returns the canonical name from a short name.
    fn canonical_name(&self, name: &str) -> Option<&str> {
        self.meta.get(name).map(|m| m.name)
    }

    /// Returns all option names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.meta
            .values()
            .map(|m| m.name)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
    }
}
