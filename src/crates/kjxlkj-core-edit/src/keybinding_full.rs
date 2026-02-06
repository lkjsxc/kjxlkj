/// Full keybinding coverage map — tracks which bindings are implemented/tested.

use std::collections::HashMap;

/// Implementation status of a keybinding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingStatus { Implemented, Partial, Stub, NotImplemented }

/// A keybinding entry in the coverage map.
#[derive(Debug, Clone, PartialEq)]
pub struct BindingEntry {
    pub key: String,
    pub mode: String,
    pub description: String,
    pub status: BindingStatus,
    pub test_count: usize,
}

/// Keybinding coverage tracker.
#[derive(Debug, Default)]
pub struct BindingCoverage { entries: HashMap<String, BindingEntry> }

impl BindingCoverage {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, key: &str, mode: &str, desc: &str, status: BindingStatus) {
        let id = format!("{}:{}", mode, key);
        self.entries.insert(id, BindingEntry {
            key: key.into(), mode: mode.into(), description: desc.into(),
            status, test_count: 0,
        });
    }

    pub fn set_status(&mut self, mode: &str, key: &str, status: BindingStatus) {
        let id = format!("{}:{}", mode, key);
        if let Some(e) = self.entries.get_mut(&id) { e.status = status; }
    }

    pub fn add_test(&mut self, mode: &str, key: &str) {
        let id = format!("{}:{}", mode, key);
        if let Some(e) = self.entries.get_mut(&id) { e.test_count += 1; }
    }

    pub fn coverage_percent(&self) -> f64 {
        if self.entries.is_empty() { return 100.0; }
        let impl_count = self.entries.values()
            .filter(|e| matches!(e.status, BindingStatus::Implemented | BindingStatus::Partial)).count();
        (impl_count as f64 / self.entries.len() as f64) * 100.0
    }

    pub fn untested(&self) -> Vec<&BindingEntry> {
        self.entries.values().filter(|e| e.test_count == 0
            && matches!(e.status, BindingStatus::Implemented | BindingStatus::Partial)).collect()
    }

    pub fn not_implemented(&self) -> Vec<&BindingEntry> {
        self.entries.values().filter(|e| e.status == BindingStatus::NotImplemented).collect()
    }

    pub fn total(&self) -> usize { self.entries.len() }

    pub fn by_mode(&self, mode: &str) -> Vec<&BindingEntry> {
        self.entries.values().filter(|e| e.mode == mode).collect()
    }
}

/// Build the default normal-mode keybinding coverage map.
pub fn build_normal_coverage() -> BindingCoverage {
    let mut c = BindingCoverage::new();
    let bindings = [
        ("h", "Move left"), ("j", "Move down"), ("k", "Move up"), ("l", "Move right"),
        ("w", "Word forward"), ("b", "Word backward"), ("e", "Word end"),
        ("0", "Line start"), ("$", "Line end"), ("^", "First non-blank"),
        ("gg", "File start"), ("G", "File end"),
        ("dd", "Delete line"), ("yy", "Yank line"), ("p", "Paste after"), ("P", "Paste before"),
        ("x", "Delete char"), ("r", "Replace char"),
        ("i", "Insert before"), ("a", "Insert after"), ("o", "Open below"), ("O", "Open above"),
        ("u", "Undo"), ("<C-r>", "Redo"),
        ("v", "Visual char"), ("V", "Visual line"), ("<C-v>", "Visual block"),
        ("/", "Search forward"), ("?", "Search backward"), ("n", "Next match"), ("N", "Prev match"),
        (":", "Command mode"), (".", "Repeat"), ("%", "Match bracket"),
        ("f", "Find char"), ("t", "Till char"), (";", "Repeat find"), (",", "Reverse find"),
        (">>", "Indent"), ("<<", "Outdent"), ("J", "Join lines"),
    ];
    for (key, desc) in bindings {
        c.register(key, "normal", desc, BindingStatus::Implemented);
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_query() {
        let mut c = BindingCoverage::new();
        c.register("h", "normal", "Move left", BindingStatus::Implemented);
        assert_eq!(c.total(), 1);
    }

    #[test]
    fn coverage_percent_full() {
        let c = build_normal_coverage();
        assert!((c.coverage_percent() - 100.0).abs() < 0.01);
    }

    #[test]
    fn untested_includes_no_tests() {
        let c = build_normal_coverage();
        assert!(!c.untested().is_empty()); // None have test_count > 0 yet
    }

    #[test]
    fn add_test_reduces_untested() {
        let mut c = BindingCoverage::new();
        c.register("h", "normal", "left", BindingStatus::Implemented);
        c.add_test("normal", "h");
        assert!(c.untested().is_empty());
    }

    #[test]
    fn not_implemented_list() {
        let mut c = BindingCoverage::new();
        c.register("zz", "normal", "center", BindingStatus::NotImplemented);
        assert_eq!(c.not_implemented().len(), 1);
    }

    #[test]
    fn by_mode_filters() {
        let mut c = BindingCoverage::new();
        c.register("h", "normal", "left", BindingStatus::Implemented);
        c.register("jk", "insert", "escape", BindingStatus::Implemented);
        assert_eq!(c.by_mode("normal").len(), 1);
        assert_eq!(c.by_mode("insert").len(), 1);
    }

    #[test]
    fn set_status() {
        let mut c = BindingCoverage::new();
        c.register("x", "normal", "del", BindingStatus::Stub);
        c.set_status("normal", "x", BindingStatus::Implemented);
        assert_eq!(c.by_mode("normal")[0].status, BindingStatus::Implemented);
    }

    #[test]
    fn build_normal_has_core_keys() {
        let c = build_normal_coverage();
        assert!(c.total() >= 30);
    }
}
