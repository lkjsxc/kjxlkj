//! Leader key management and registry.

use serde::{Deserialize, Serialize};

/// Configuration for the leader key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderConfig {
    pub key: String,
    pub timeout_ms: u64,
}

impl Default for LeaderConfig {
    fn default() -> Self {
        Self {
            key: "Space".into(),
            timeout_ms: 1000,
        }
    }
}

/// A single leader key binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaderBinding {
    pub chord: String,
    pub action: String,
    pub description: String,
}

/// Registry of leader key bindings.
#[derive(Debug, Clone, Default)]
pub struct LeaderRegistry {
    pub bindings: Vec<LeaderBinding>,
}

impl LeaderRegistry {
    /// Look up a binding by exact chord match.
    pub fn resolve(&self, chord: &str) -> Option<&LeaderBinding> {
        self.bindings.iter().find(|b| b.chord == chord)
    }

    /// Find bindings whose chord starts with `prefix` but is not equal to it.
    pub fn partial_matches(&self, prefix: &str) -> Vec<&LeaderBinding> {
        self.bindings
            .iter()
            .filter(|b| b.chord.starts_with(prefix) && b.chord != prefix)
            .collect()
    }

    /// Add or replace a binding.
    pub fn bind(&mut self, chord: &str, action: &str, desc: &str) {
        self.unbind(chord);
        self.bindings.push(LeaderBinding {
            chord: chord.into(),
            action: action.into(),
            description: desc.into(),
        });
    }

    /// Remove a binding by chord.
    pub fn unbind(&mut self, chord: &str) {
        self.bindings.retain(|b| b.chord != chord);
    }
}

/// Default leader bindings (17 bindings).
pub fn default_leader_bindings() -> Vec<LeaderBinding> {
    let b = |c: &str, a: &str, d: &str| LeaderBinding {
        chord: c.into(),
        action: a.into(),
        description: d.into(),
    };
    vec![
        b("f", "find_file", "Find file"),
        b("g", "live_grep", "Live grep"),
        b("b", "buffers", "Buffer list"),
        b("e", "explorer", "File explorer"),
        b("t", "terminal", "Toggle terminal"),
        b("u", "undo_tree", "Undo tree"),
        b("w", "save", "Save file"),
        b("q", "quit", "Quit"),
        b("h", "help", "Help"),
        b("s", "hsplit", "Horizontal split"),
        b("v", "vsplit", "Vertical split"),
        b("c", "close", "Close window"),
        b("n", "new_buffer", "New buffer"),
        b("p", "paste_clipboard", "Paste clipboard"),
        b("r", "recent_files", "Recent files"),
        b("l", "lsp_actions", "LSP actions"),
        b("d", "diagnostics", "Diagnostics"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let cfg = LeaderConfig::default();
        assert_eq!(cfg.key, "Space");
        assert_eq!(cfg.timeout_ms, 1000);
    }

    #[test]
    fn registry_resolve() {
        let reg = LeaderRegistry { bindings: default_leader_bindings() };
        assert_eq!(reg.resolve("f").unwrap().action, "find_file");
        assert!(reg.resolve("z").is_none());
    }

    #[test]
    fn registry_partial_matches() {
        let mut reg = LeaderRegistry::default();
        reg.bind("ff", "find_file", "find");
        reg.bind("fg", "find_grep", "grep");
        reg.bind("b", "buffers", "bufs");
        let partial = reg.partial_matches("f");
        assert_eq!(partial.len(), 2);
    }

    #[test]
    fn bind_and_unbind() {
        let mut reg = LeaderRegistry::default();
        reg.bind("x", "action_x", "desc x");
        assert!(reg.resolve("x").is_some());
        reg.unbind("x");
        assert!(reg.resolve("x").is_none());
    }

    #[test]
    fn default_has_17() {
        assert_eq!(default_leader_bindings().len(), 17);
    }
}
