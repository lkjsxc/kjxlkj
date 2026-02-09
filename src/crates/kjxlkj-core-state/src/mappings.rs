//! Keybinding mapping infrastructure.

use kjxlkj_core_types::Mode;

/// A user-defined key mapping.
#[derive(Debug, Clone)]
pub struct KeyMapping {
    /// Modes where this mapping is active.
    pub modes: Vec<MappingMode>,
    /// The key sequence to match (lhs).
    pub lhs: String,
    /// The replacement key sequence or command (rhs).
    pub rhs: String,
    /// Whether this mapping is recursive (nmap vs nnoremap).
    pub noremap: bool,
    /// Silent: don't show in command line.
    pub silent: bool,
    /// Buffer-local only.
    pub buffer_local: bool,
    /// Expression mapping (rhs is evaluated).
    pub expr: bool,
}

/// Mapping mode identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingMode {
    Normal,
    Insert,
    Visual,
    Command,
    OperatorPending,
    Terminal,
}

/// Mapping registry.
#[derive(Debug, Default)]
pub struct MappingRegistry {
    /// All registered mappings.
    mappings: Vec<KeyMapping>,
}

impl MappingRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a mapping.
    pub fn add(&mut self, mapping: KeyMapping) {
        // Remove existing mapping with same lhs/modes.
        self.mappings
            .retain(|m| m.lhs != mapping.lhs || m.modes != mapping.modes);
        self.mappings.push(mapping);
    }

    /// Remove a mapping by lhs and mode.
    pub fn remove(&mut self, lhs: &str, mode: MappingMode) {
        self.mappings
            .retain(|m| !(m.lhs == lhs && m.modes.contains(&mode)));
    }

    /// Find a mapping for a key sequence in a mode.
    pub fn find(&self, lhs: &str, mode: MappingMode) -> Option<&KeyMapping> {
        self.mappings
            .iter()
            .find(|m| m.lhs == lhs && m.modes.contains(&mode))
    }

    /// List all mappings for a mode.
    pub fn list(&self, mode: MappingMode) -> Vec<&KeyMapping> {
        self.mappings
            .iter()
            .filter(|m| m.modes.contains(&mode))
            .collect()
    }

    /// Clear all mappings.
    pub fn clear(&mut self) {
        self.mappings.clear();
    }
}

/// Parse mapping command (`:nmap`, `:nnoremap`, etc).
pub fn parse_mapping_cmd(cmd: &str, args: &str) -> Option<KeyMapping> {
    let (modes, noremap) = match cmd {
        "map" => (
            vec![
                MappingMode::Normal,
                MappingMode::Visual,
                MappingMode::OperatorPending,
            ],
            false,
        ),
        "noremap" => (
            vec![
                MappingMode::Normal,
                MappingMode::Visual,
                MappingMode::OperatorPending,
            ],
            true,
        ),
        "nmap" => (vec![MappingMode::Normal], false),
        "nnoremap" => (vec![MappingMode::Normal], true),
        "imap" => (vec![MappingMode::Insert], false),
        "inoremap" => (vec![MappingMode::Insert], true),
        "vmap" => (vec![MappingMode::Visual], false),
        "vnoremap" => (vec![MappingMode::Visual], true),
        "cmap" => (vec![MappingMode::Command], false),
        "cnoremap" => (vec![MappingMode::Command], true),
        "tmap" => (vec![MappingMode::Terminal], false),
        "tnoremap" => (vec![MappingMode::Terminal], true),
        "omap" => (vec![MappingMode::OperatorPending], false),
        "onoremap" => (vec![MappingMode::OperatorPending], true),
        _ => return None,
    };
    let parts: Vec<&str> = args.splitn(2, char::is_whitespace).collect();
    if parts.len() < 2 {
        return None;
    }
    Some(KeyMapping {
        modes,
        lhs: parts[0].to_string(),
        rhs: parts[1].to_string(),
        noremap,
        silent: false,
        buffer_local: false,
        expr: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nnoremap() {
        let m = parse_mapping_cmd("nnoremap", "<leader>e :e .");
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.lhs, "<leader>e");
        assert_eq!(m.rhs, ":e .");
        assert!(m.noremap);
    }

    #[test]
    fn registry_add_find() {
        let mut reg = MappingRegistry::new();
        reg.add(KeyMapping {
            modes: vec![MappingMode::Normal],
            lhs: "jk".to_string(),
            rhs: "<Esc>".to_string(),
            noremap: true,
            silent: false,
            buffer_local: false,
            expr: false,
        });
        let found = reg.find("jk", MappingMode::Normal);
        assert!(found.is_some());
        let not_found = reg.find("jk", MappingMode::Insert);
        assert!(not_found.is_none());
    }
}
