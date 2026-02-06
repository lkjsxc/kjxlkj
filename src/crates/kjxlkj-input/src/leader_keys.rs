/// Leader key system — leader key configuration and chord dispatch.

/// Leader key configuration.
#[derive(Debug, Clone)]
pub struct LeaderConfig { pub key: char, pub timeout_ms: u64 }

impl Default for LeaderConfig {
    fn default() -> Self { Self { key: ' ', timeout_ms: 1000 } }
}

/// A leader chord binding.
#[derive(Debug, Clone)]
pub struct LeaderBinding { pub chord: Vec<char>, pub action: String, pub description: String }

impl LeaderBinding {
    pub fn new(chord: &[char], action: impl Into<String>, desc: impl Into<String>) -> Self {
        Self { chord: chord.to_vec(), action: action.into(), description: desc.into() }
    }
}

/// Leader binding registry.
#[derive(Debug)]
pub struct LeaderRegistry { pub config: LeaderConfig, bindings: Vec<LeaderBinding> }

impl LeaderRegistry {
    pub fn new(config: LeaderConfig) -> Self { Self { config, bindings: Vec::new() } }

    pub fn bind(&mut self, chord: &[char], action: impl Into<String>, desc: impl Into<String>) {
        self.bindings.push(LeaderBinding::new(chord, action, desc));
    }

    pub fn resolve(&self, chord: &[char]) -> Option<&LeaderBinding> {
        self.bindings.iter().find(|b| b.chord == chord)
    }

    pub fn partial_matches(&self, prefix: &[char]) -> Vec<&LeaderBinding> {
        self.bindings.iter().filter(|b| b.chord.starts_with(prefix) && b.chord.len() > prefix.len()).collect()
    }

    pub fn count(&self) -> usize { self.bindings.len() }

    pub fn all_bindings(&self) -> &[LeaderBinding] { &self.bindings }
}

/// Build default leader key bindings.
pub fn default_leader_bindings() -> LeaderRegistry {
    let mut r = LeaderRegistry::new(LeaderConfig::default());
    r.bind(&['f', 'f'], "find_file", "Find file");
    r.bind(&['f', 'g'], "live_grep", "Live grep");
    r.bind(&['f', 'b'], "find_buffer", "Find buffer");
    r.bind(&['f', 'h'], "find_help", "Find help");
    r.bind(&['e'], "toggle_explorer", "Toggle file explorer");
    r.bind(&['t'], "toggle_terminal", "Toggle terminal");
    r.bind(&['w'], "save_file", "Save file");
    r.bind(&['q'], "close_window", "Close window");
    r.bind(&['b', 'n'], "next_buffer", "Next buffer");
    r.bind(&['b', 'p'], "prev_buffer", "Previous buffer");
    r.bind(&['b', 'd'], "delete_buffer", "Delete buffer");
    r.bind(&['g', 's'], "git_status", "Git status");
    r.bind(&['g', 'd'], "git_diff", "Git diff");
    r.bind(&['g', 'b'], "git_blame", "Git blame");
    r.bind(&['l', 'r'], "lsp_rename", "LSP rename");
    r.bind(&['l', 'a'], "lsp_code_action", "LSP code action");
    r.bind(&['l', 'd'], "lsp_definition", "Go to definition");
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() { let c = LeaderConfig::default(); assert_eq!(c.key, ' '); }

    #[test]
    fn build_defaults() { let r = default_leader_bindings(); assert!(r.count() >= 15); }

    #[test]
    fn resolve_exact() {
        let r = default_leader_bindings();
        let b = r.resolve(&['e']).unwrap();
        assert_eq!(b.action, "toggle_explorer");
    }

    #[test]
    fn resolve_multi_char() {
        let r = default_leader_bindings();
        assert!(r.resolve(&['f', 'f']).is_some());
    }

    #[test]
    fn partial_matches() {
        let r = default_leader_bindings();
        let matches = r.partial_matches(&['f']);
        assert!(matches.len() >= 3);
    }

    #[test]
    fn no_match() {
        let r = default_leader_bindings();
        assert!(r.resolve(&['z', 'z']).is_none());
    }

    #[test]
    fn empty_partial() {
        let r = default_leader_bindings();
        let matches = r.partial_matches(&['z']);
        assert!(matches.is_empty());
    }
}
