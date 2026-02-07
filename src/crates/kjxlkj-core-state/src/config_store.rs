//! Persistent configuration option store with scope support.

use std::collections::HashMap;

/// The scope at which an option lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionScope {
    Global,
    Buffer,
    Window,
}

/// A single option definition with default and current value.
#[derive(Debug, Clone)]
pub struct OptionDef {
    pub name: String,
    pub scope: OptionScope,
    pub default_value: String,
    pub value: String,
}

/// Configuration store holding all editor options.
#[derive(Debug, Clone)]
pub struct ConfigStore {
    options: HashMap<String, OptionDef>,
}

impl ConfigStore {
    pub fn new() -> Self {
        Self { options: HashMap::new() }
    }

    /// Register an option with its default value.
    pub fn register(&mut self, name: &str, scope: OptionScope, default: &str) {
        self.options.insert(name.to_string(), OptionDef {
            name: name.to_string(),
            scope,
            default_value: default.to_string(),
            value: default.to_string(),
        });
    }

    /// Get the current value of an option.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.options.get(name).map(|o| o.value.as_str())
    }

    /// Set an option value. Returns false if option does not exist.
    pub fn set(&mut self, name: &str, value: &str) -> bool {
        if let Some(opt) = self.options.get_mut(name) {
            opt.value = value.to_string();
            true
        } else {
            false
        }
    }

    /// Resolve an option, returning its current value or None.
    pub fn resolve(&self, name: &str) -> Option<&str> {
        self.get(name)
    }

    /// List all option names.
    pub fn names(&self) -> Vec<&str> {
        self.options.keys().map(|s| s.as_str()).collect()
    }

    /// Reset an option to its default.
    pub fn reset(&mut self, name: &str) -> bool {
        if let Some(opt) = self.options.get_mut(name) {
            opt.value = opt.default_value.clone();
            true
        } else {
            false
        }
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a ConfigStore with built-in defaults.
pub fn build_defaults() -> ConfigStore {
    let mut store = ConfigStore::new();
    store.register("number", OptionScope::Window, "true");
    store.register("relativenumber", OptionScope::Window, "false");
    store.register("wrap", OptionScope::Window, "true");
    store.register("tabstop", OptionScope::Buffer, "8");
    store.register("shiftwidth", OptionScope::Buffer, "8");
    store.register("expandtab", OptionScope::Buffer, "false");
    store.register("scrolloff", OptionScope::Global, "5");
    store.register("ignorecase", OptionScope::Global, "false");
    store.register("smartcase", OptionScope::Global, "false");
    store.register("hlsearch", OptionScope::Global, "true");
    store.register("incsearch", OptionScope::Global, "true");
    store.register("autoindent", OptionScope::Buffer, "true");
    store.register("syntax", OptionScope::Global, "true");
    store
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let store = build_defaults();
        assert_eq!(store.get("number"), Some("true"));
        assert_eq!(store.get("tabstop"), Some("8"));
        assert_eq!(store.get("nonexistent"), None);
    }

    #[test]
    fn set_and_reset() {
        let mut store = build_defaults();
        store.set("tabstop", "4");
        assert_eq!(store.get("tabstop"), Some("4"));
        store.reset("tabstop");
        assert_eq!(store.get("tabstop"), Some("8"));
    }

    #[test]
    fn names_list() {
        let store = build_defaults();
        assert!(store.names().len() >= 10);
    }
}
