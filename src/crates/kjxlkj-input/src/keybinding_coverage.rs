//! Keybinding coverage map — tracks which keys are bound and tested.

/// A keybinding entry in the coverage map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBinding {
    pub mode: String,
    pub keys: String,
    pub description: String,
    pub tested: bool,
    pub documented: bool,
}

/// The full coverage map.
#[derive(Debug, Clone)]
pub struct CoverageMap {
    pub bindings: Vec<KeyBinding>,
}

impl CoverageMap {
    pub fn new() -> Self { Self { bindings: Vec::new() } }

    /// Add a keybinding to the map.
    pub fn add(&mut self, mode: &str, keys: &str, desc: &str, tested: bool, documented: bool) {
        self.bindings.push(KeyBinding {
            mode: mode.into(), keys: keys.into(), description: desc.into(), tested, documented,
        });
    }

    /// Find all untested bindings.
    pub fn untested(&self) -> Vec<&KeyBinding> {
        self.bindings.iter().filter(|b| !b.tested).collect()
    }

    /// Find all undocumented bindings.
    pub fn undocumented(&self) -> Vec<&KeyBinding> {
        self.bindings.iter().filter(|b| !b.documented).collect()
    }

    /// Find bindings for a specific mode.
    pub fn for_mode(&self, mode: &str) -> Vec<&KeyBinding> {
        self.bindings.iter().filter(|b| b.mode == mode).collect()
    }

    /// Coverage percentage (tested / total).
    pub fn coverage_pct(&self) -> f64 {
        if self.bindings.is_empty() { return 100.0; }
        let tested = self.bindings.iter().filter(|b| b.tested).count();
        (tested as f64 / self.bindings.len() as f64) * 100.0
    }

    /// Generate a coverage summary string.
    pub fn summary(&self) -> String {
        let total = self.bindings.len();
        let tested = self.bindings.iter().filter(|b| b.tested).count();
        let documented = self.bindings.iter().filter(|b| b.documented).count();
        format!("{}/{} tested, {}/{} documented ({:.1}% coverage)",
            tested, total, documented, total, self.coverage_pct())
    }

    /// Find a binding by mode and key sequence.
    pub fn find(&self, mode: &str, keys: &str) -> Option<&KeyBinding> {
        self.bindings.iter().find(|b| b.mode == mode && b.keys == keys)
    }

    /// Check for duplicate key bindings within the same mode.
    pub fn find_duplicates(&self) -> Vec<(String, String)> {
        let mut seen = std::collections::HashSet::new();
        let mut dupes = Vec::new();
        for b in &self.bindings {
            let key = (b.mode.clone(), b.keys.clone());
            if !seen.insert(key.clone()) {
                dupes.push(key);
            }
        }
        dupes
    }
}

impl Default for CoverageMap { fn default() -> Self { Self::new() } }

/// Build a default Normal mode coverage map with common keys.
pub fn build_default_normal_coverage() -> CoverageMap {
    let mut m = CoverageMap::new();
    let keys = [
        ("h", "move left"), ("j", "move down"), ("k", "move up"), ("l", "move right"),
        ("w", "word forward"), ("b", "word back"), ("e", "word end"),
        ("0", "line start"), ("$", "line end"), ("^", "first non-blank"),
        ("gg", "file start"), ("G", "file end"),
        ("i", "insert before"), ("a", "append after"), ("o", "open below"), ("O", "open above"),
        ("d", "delete operator"), ("y", "yank operator"), ("c", "change operator"),
        ("x", "delete char"), ("p", "paste after"), ("P", "paste before"),
        ("u", "undo"), ("Ctrl-r", "redo"), (".", "repeat"),
    ];
    for (k, desc) in &keys {
        m.add("normal", k, desc, true, true);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_map() {
        let m = CoverageMap::new();
        assert_eq!(m.coverage_pct(), 100.0);
        assert!(m.untested().is_empty());
    }

    #[test]
    fn add_and_find() {
        let mut m = CoverageMap::new();
        m.add("normal", "j", "move down", true, true);
        assert!(m.find("normal", "j").is_some());
        assert!(m.find("normal", "k").is_none());
    }

    #[test]
    fn coverage_percentage() {
        let mut m = CoverageMap::new();
        m.add("normal", "j", "down", true, true);
        m.add("normal", "k", "up", false, true);
        assert!((m.coverage_pct() - 50.0).abs() < 0.01);
    }

    #[test]
    fn untested_bindings() {
        let mut m = CoverageMap::new();
        m.add("normal", "j", "down", true, true);
        m.add("normal", "k", "up", false, true);
        assert_eq!(m.untested().len(), 1);
        assert_eq!(m.untested()[0].keys, "k");
    }

    #[test]
    fn mode_filter() {
        let mut m = CoverageMap::new();
        m.add("normal", "j", "down", true, true);
        m.add("insert", "Esc", "exit", true, true);
        assert_eq!(m.for_mode("normal").len(), 1);
        assert_eq!(m.for_mode("insert").len(), 1);
    }

    #[test]
    fn summary_format() {
        let mut m = CoverageMap::new();
        m.add("normal", "j", "down", true, true);
        m.add("normal", "k", "up", true, false);
        let s = m.summary();
        assert!(s.contains("2/2 tested"));
        assert!(s.contains("1/2 documented"));
    }

    #[test]
    fn default_normal_coverage() {
        let m = build_default_normal_coverage();
        assert!(m.bindings.len() >= 20);
        assert_eq!(m.coverage_pct(), 100.0);
    }

    #[test]
    fn find_duplicates() {
        let mut m = CoverageMap::new();
        m.add("normal", "j", "down", true, true);
        m.add("normal", "j", "down dup", true, true);
        let dupes = m.find_duplicates();
        assert_eq!(dupes.len(), 1);
    }
}
