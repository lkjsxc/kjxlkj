//! Key mapping support (`:map`, `:nmap`, `:imap`, etc.).

use serde::{Deserialize, Serialize};

/// Mode in which a mapping is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapMode {
    Normal, Insert, Visual, Command, OperatorPending, Terminal,
}

/// A single key mapping entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingEntry {
    pub mode: MapMode,
    pub from: String,
    pub to: String,
    pub noremap: bool,
    pub buffer_local: bool,
}

/// Store for key mappings.
#[derive(Debug, Clone, Default)]
pub struct MappingStore {
    pub entries: Vec<MappingEntry>,
}

impl MappingStore {
    /// Add or replace a mapping.
    pub fn add(&mut self, entry: MappingEntry) {
        self.remove(entry.mode, &entry.from);
        self.entries.push(entry);
    }

    /// Remove a mapping by mode and source keys.
    pub fn remove(&mut self, mode: MapMode, from: &str) {
        self.entries.retain(|e| !(e.mode == mode && e.from == from));
    }

    /// Look up a mapping by mode and source keys.
    pub fn lookup(&self, mode: MapMode, from: &str) -> Option<&MappingEntry> {
        self.entries.iter().find(|e| e.mode == mode && e.from == from)
    }

    /// Check if any mapping starts with the given prefix (but is longer).
    pub fn has_prefix(&self, mode: MapMode, prefix: &str) -> bool {
        self.entries.iter().any(|e| e.mode == mode && e.from.starts_with(prefix) && e.from != prefix)
    }

    /// List all mappings for a mode.
    pub fn list(&self, mode: MapMode) -> Vec<&MappingEntry> {
        self.entries.iter().filter(|e| e.mode == mode).collect()
    }

    /// Remove all mappings for a mode.
    pub fn clear(&mut self, mode: MapMode) {
        self.entries.retain(|e| e.mode != mode);
    }
}

/// Parse a map command string like `nmap jj <Esc>` or `nnoremap <C-s> :w<CR>`.
pub fn parse_map_command(input: &str) -> Option<(MapMode, String, String, bool)> {
    let parts: Vec<&str> = input.splitn(3, char::is_whitespace).collect();
    if parts.len() < 3 { return None; }
    let (mode, noremap) = match parts[0] {
        "map" | "nmap" => (MapMode::Normal, false),
        "noremap" | "nnoremap" => (MapMode::Normal, true),
        "imap" => (MapMode::Insert, false),
        "inoremap" => (MapMode::Insert, true),
        "vmap" => (MapMode::Visual, false),
        "vnoremap" => (MapMode::Visual, true),
        "cmap" => (MapMode::Command, false),
        "cnoremap" => (MapMode::Command, true),
        "omap" => (MapMode::OperatorPending, false),
        "onoremap" => (MapMode::OperatorPending, true),
        "tmap" => (MapMode::Terminal, false),
        "tnoremap" => (MapMode::Terminal, true),
        _ => return None,
    };
    Some((mode, parts[1].to_string(), parts[2].to_string(), noremap))
}

/// Resolve a mapping from the store.
pub fn resolve_mapping(store: &MappingStore, mode: MapMode, keys: &str) -> Option<String> {
    store.lookup(mode, keys).map(|e| e.to.clone())
}

/// Expand a mapping recursively (up to `depth_limit` for remappable bindings).
pub fn expand_recursive(
    store: &MappingStore, mode: MapMode, keys: &str, depth_limit: usize,
) -> Result<String, ()> {
    let mut current = keys.to_string();
    for _ in 0..depth_limit {
        match store.lookup(mode, &current) {
            Some(entry) => {
                current = entry.to.clone();
                if entry.noremap { return Ok(current); }
            }
            None => return Ok(current),
        }
    }
    Err(()) // infinite loop detected
}

#[cfg(test)]
mod tests {
    use super::*;

    fn me(mode: MapMode, from: &str, to: &str, noremap: bool) -> MappingEntry {
        MappingEntry { mode, from: from.into(), to: to.into(), noremap, buffer_local: false }
    }

    #[test]
    fn parse_nmap() {
        let (mode, from, to, noremap) = parse_map_command("nmap jj <Esc>").unwrap();
        assert_eq!(mode, MapMode::Normal);
        assert_eq!(from, "jj");
        assert_eq!(to, "<Esc>");
        assert!(!noremap);
    }

    #[test]
    fn parse_nnoremap() {
        let (_, _, _, noremap) = parse_map_command("nnoremap <C-s> :w<CR>").unwrap();
        assert!(noremap);
    }

    #[test]
    fn store_add_lookup() {
        let mut s = MappingStore::default();
        s.add(me(MapMode::Normal, "jj", "<Esc>", false));
        assert_eq!(s.lookup(MapMode::Normal, "jj").unwrap().to, "<Esc>");
    }

    #[test]
    fn expand_noremap() {
        let mut s = MappingStore::default();
        s.add(me(MapMode::Normal, "a", "b", true));
        assert_eq!(expand_recursive(&s, MapMode::Normal, "a", 10).unwrap(), "b");
    }

    #[test]
    fn expand_loop_detection() {
        let mut s = MappingStore::default();
        s.add(me(MapMode::Normal, "a", "b", false));
        s.add(me(MapMode::Normal, "b", "a", false));
        assert!(expand_recursive(&s, MapMode::Normal, "a", 10).is_err());
    }

    #[test]
    fn has_prefix_works() {
        let mut s = MappingStore::default();
        s.add(me(MapMode::Normal, "gc", "comment", true));
        assert!(s.has_prefix(MapMode::Normal, "g"));
        assert!(!s.has_prefix(MapMode::Normal, "gc"));
    }

    #[test]
    fn clear_mode() {
        let mut s = MappingStore::default();
        s.add(me(MapMode::Normal, "a", "b", true));
        s.add(me(MapMode::Insert, "c", "d", true));
        s.clear(MapMode::Normal);
        assert!(s.list(MapMode::Normal).is_empty());
        assert_eq!(s.list(MapMode::Insert).len(), 1);
    }
}
