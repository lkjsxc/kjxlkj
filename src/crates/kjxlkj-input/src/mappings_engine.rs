/// Key mapping engine — :map/:nmap/:imap, recursive/non-recursive, timeout.

use std::collections::HashMap;

/// Map mode context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapMode { Normal, Insert, Visual, Command, OperatorPending, Terminal }

impl MapMode {
    pub fn from_prefix(s: &str) -> Option<Self> {
        match s {
            "n" | "nmap" | "nnoremap" => Some(Self::Normal),
            "i" | "imap" | "inoremap" => Some(Self::Insert),
            "v" | "vmap" | "vnoremap" => Some(Self::Visual),
            "c" | "cmap" | "cnoremap" => Some(Self::Command),
            "o" | "omap" | "onoremap" => Some(Self::OperatorPending),
            "t" | "tmap" | "tnoremap" => Some(Self::Terminal),
            _ => None,
        }
    }
}

/// A single key mapping definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub lhs: String,
    pub rhs: String,
    pub mode: MapMode,
    pub noremap: bool,
    pub silent: bool,
    pub buffer_local: bool,
    pub nowait: bool,
}

/// Key mapping store per mode.
#[derive(Debug, Default)]
pub struct MappingStore { maps: HashMap<MapMode, Vec<Mapping>> }

impl MappingStore {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, mapping: Mapping) {
        let mode = mapping.mode;
        let vec = self.maps.entry(mode).or_default();
        vec.retain(|m| m.lhs != mapping.lhs || m.buffer_local != mapping.buffer_local);
        vec.push(mapping);
    }

    pub fn remove(&mut self, mode: MapMode, lhs: &str) -> bool {
        if let Some(vec) = self.maps.get_mut(&mode) {
            let before = vec.len();
            vec.retain(|m| m.lhs != lhs);
            return vec.len() < before;
        }
        false
    }

    pub fn lookup(&self, mode: MapMode, lhs: &str) -> Option<&Mapping> {
        self.maps.get(&mode)?.iter().find(|m| m.lhs == lhs)
    }

    /// Check if any mapping starts with the given prefix.
    pub fn has_prefix(&self, mode: MapMode, prefix: &str) -> bool {
        self.maps.get(&mode).map_or(false, |v| v.iter().any(|m| m.lhs.starts_with(prefix)))
    }

    pub fn list(&self, mode: MapMode) -> Vec<&Mapping> {
        self.maps.get(&mode).map_or(vec![], |v| v.iter().collect())
    }

    pub fn clear(&mut self, mode: MapMode) { self.maps.remove(&mode); }
}

/// Parse a :map command line into a Mapping.
pub fn parse_map_command(cmd: &str) -> Option<Mapping> {
    let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
    if parts.len() < 3 { return None; }
    let prefix = parts[0];
    let noremap = prefix.contains("noremap");
    let mode = MapMode::from_prefix(prefix)?;
    let lhs = parts[1].to_string();
    let rhs = parts[2].to_string();
    Some(Mapping { lhs, rhs, mode, noremap, silent: false, buffer_local: false, nowait: false })
}

/// Resolve a key sequence through mappings (non-recursive for noremap).
pub fn resolve_mapping<'a>(store: &'a MappingStore, mode: MapMode, keys: &str, depth: usize) -> Option<&'a str> {
    if depth > 20 { return None; } // Prevent infinite recursion
    let mapping = store.lookup(mode, keys)?;
    if mapping.noremap { return Some(&mapping.rhs); }
    // For recursive mappings, check if rhs also maps
    store.lookup(mode, &mapping.rhs)
        .map(|m| m.rhs.as_str())
        .or(Some(&mapping.rhs))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nmap(lhs: &str, rhs: &str) -> Mapping {
        Mapping { lhs: lhs.into(), rhs: rhs.into(), mode: MapMode::Normal,
            noremap: false, silent: false, buffer_local: false, nowait: false }
    }

    #[test]
    fn add_and_lookup() {
        let mut store = MappingStore::new();
        store.add(nmap("jk", "<Esc>"));
        assert_eq!(store.lookup(MapMode::Normal, "jk").unwrap().rhs, "<Esc>");
    }

    #[test]
    fn remove_mapping() {
        let mut store = MappingStore::new();
        store.add(nmap(",w", ":w<CR>"));
        assert!(store.remove(MapMode::Normal, ",w"));
        assert!(store.lookup(MapMode::Normal, ",w").is_none());
    }

    #[test]
    fn prefix_check() {
        let mut store = MappingStore::new();
        store.add(nmap("<leader>ff", ":find"));
        assert!(store.has_prefix(MapMode::Normal, "<leader>"));
    }

    #[test]
    fn parse_nmap() {
        let m = parse_map_command("nmap jk <Esc>").unwrap();
        assert_eq!(m.mode, MapMode::Normal);
        assert_eq!(m.lhs, "jk");
        assert!(!m.noremap);
    }

    #[test]
    fn parse_nnoremap() {
        let m = parse_map_command("nnoremap ,w :w<CR>").unwrap();
        assert!(m.noremap);
    }

    #[test]
    fn resolve_noremap() {
        let mut store = MappingStore::new();
        let mut m = nmap("x", "dd");
        m.noremap = true;
        store.add(m);
        assert_eq!(resolve_mapping(&store, MapMode::Normal, "x", 0).unwrap(), "dd");
    }

    #[test]
    fn overwrite_mapping() {
        let mut store = MappingStore::new();
        store.add(nmap("jk", "old"));
        store.add(nmap("jk", "new"));
        assert_eq!(store.lookup(MapMode::Normal, "jk").unwrap().rhs, "new");
    }

    #[test]
    fn list_mode_mappings() {
        let mut store = MappingStore::new();
        store.add(nmap("a", "b"));
        store.add(nmap("c", "d"));
        assert_eq!(store.list(MapMode::Normal).len(), 2);
    }

    #[test]
    fn mode_from_prefix() {
        assert_eq!(MapMode::from_prefix("imap"), Some(MapMode::Insert));
        assert_eq!(MapMode::from_prefix("vnoremap"), Some(MapMode::Visual));
        assert_eq!(MapMode::from_prefix("xyz"), None);
    }
}
