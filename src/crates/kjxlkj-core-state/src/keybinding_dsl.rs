//! Keybinding DSL and description system.
//!
//! Per /docs/spec/ux/keybinding-dsl.md and
//! /docs/spec/ux/keybindings/*.md.

/// A keybinding description for documentation/which-key.
#[derive(Debug, Clone)]
pub struct KeybindingDesc {
    /// Key sequence string (e.g., "<leader>e").
    pub keys: String,
    /// Mode this applies to.
    pub mode: String,
    /// Description of what the key does.
    pub description: String,
    /// Category for grouping.
    pub category: String,
}

/// Which-key popup state.
#[derive(Debug, Clone, Default)]
pub struct WhichKeyState {
    /// Whether which-key popup is active.
    pub active: bool,
    /// Current prefix typed.
    pub prefix: String,
    /// Matching bindings for current prefix.
    pub matches: Vec<KeybindingDesc>,
    /// Timeout before showing (ms).
    pub timeout: u64,
}

impl WhichKeyState {
    /// Create new state.
    pub fn new() -> Self {
        Self {
            timeout: 500,
            ..Default::default()
        }
    }

    /// Start which-key with a prefix.
    pub fn start(&mut self, prefix: &str) {
        self.active = true;
        self.prefix = prefix.to_string();
    }

    /// Cancel which-key.
    pub fn cancel(&mut self) {
        self.active = false;
        self.prefix.clear();
        self.matches.clear();
    }
}

/// Leader key configuration.
#[derive(Debug, Clone)]
pub struct LeaderConfig {
    /// Leader key character.
    pub leader: char,
    /// Local leader key character.
    pub local_leader: char,
}

impl Default for LeaderConfig {
    fn default() -> Self {
        Self {
            leader: '\\',
            local_leader: ',',
        }
    }
}

/// Command palette state.
#[derive(Debug, Clone, Default)]
pub struct CommandPalette {
    /// Whether palette is active.
    pub active: bool,
    /// Current filter string.
    pub filter: String,
    /// Matching commands.
    pub commands: Vec<String>,
    /// Selected index.
    pub selected: usize,
}

impl CommandPalette {
    /// Create new palette.
    pub fn new() -> Self {
        Self::default()
    }

    /// Open palette with available commands.
    pub fn open(&mut self, cmds: Vec<String>) {
        self.active = true;
        self.commands = cmds;
        self.filter.clear();
        self.selected = 0;
    }

    /// Filter commands by input.
    pub fn set_filter(&mut self, f: &str) {
        self.filter = f.to_string();
    }

    /// Get filtered commands.
    pub fn filtered(&self) -> Vec<&str> {
        if self.filter.is_empty() {
            return self
                .commands
                .iter()
                .map(|s| s.as_str())
                .collect();
        }
        let lower = self.filter.to_lowercase();
        self.commands
            .iter()
            .filter(|c| {
                c.to_lowercase().contains(&lower)
            })
            .map(|s| s.as_str())
            .collect()
    }

    /// Close palette.
    pub fn close(&mut self) {
        self.active = false;
        self.commands.clear();
        self.filter.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn which_key_lifecycle() {
        let mut wk = WhichKeyState::new();
        wk.start("<leader>");
        assert!(wk.active);
        wk.cancel();
        assert!(!wk.active);
    }

    #[test]
    fn command_palette_filter() {
        let mut cp = CommandPalette::new();
        cp.open(vec![
            "write".into(),
            "quit".into(),
            "wqa".into(),
        ]);
        cp.set_filter("w");
        let f = cp.filtered();
        assert_eq!(f.len(), 2);
        assert!(f.contains(&"write"));
        assert!(f.contains(&"wqa"));
    }

    #[test]
    fn leader_default() {
        let lc = LeaderConfig::default();
        assert_eq!(lc.leader, '\\');
    }
}
