/// Mode-to-keybinding mapping — ensures mode behavior matches UX tables.

/// Mode identifier matching UX spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UxMode { Normal, Insert, Visual, VisualLine, VisualBlock, Replace, Command, OperatorPending, Terminal }

/// Keybinding entry from UX tables.
#[derive(Debug, Clone)]
pub struct UxBinding { pub mode: UxMode, pub key: String, pub action: String, pub documented: bool }

impl UxBinding {
    pub fn new(mode: UxMode, key: impl Into<String>, action: impl Into<String>) -> Self {
        Self { mode, key: key.into(), action: action.into(), documented: true }
    }
}

/// Keybinding table for a mode.
#[derive(Debug, Default)]
pub struct ModeBindingTable { bindings: Vec<UxBinding> }

impl ModeBindingTable {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, binding: UxBinding) { self.bindings.push(binding); }
    pub fn count(&self) -> usize { self.bindings.len() }
    pub fn for_mode(&self, mode: UxMode) -> Vec<&UxBinding> { self.bindings.iter().filter(|b| b.mode == mode).collect() }
    pub fn find_key(&self, mode: UxMode, key: &str) -> Option<&UxBinding> {
        self.bindings.iter().find(|b| b.mode == mode && b.key == key)
    }
    pub fn undocumented(&self) -> Vec<&UxBinding> { self.bindings.iter().filter(|b| !b.documented).collect() }
}

/// Build the normal mode binding table from UX spec.
pub fn build_normal_bindings() -> ModeBindingTable {
    let mut t = ModeBindingTable::new();
    let n = UxMode::Normal;
    for (key, action) in [
        ("h", "move left"), ("j", "move down"), ("k", "move up"), ("l", "move right"),
        ("w", "word forward"), ("b", "word backward"), ("e", "end of word"),
        ("0", "line start"), ("$", "line end"), ("^", "first non-blank"),
        ("gg", "file start"), ("G", "file end"),
        ("i", "enter insert"), ("a", "append"), ("o", "open below"), ("O", "open above"),
        ("x", "delete char"), ("dd", "delete line"), ("yy", "yank line"), ("p", "paste after"),
        ("u", "undo"), ("<C-r>", "redo"), (".", "repeat"),
        ("/", "search forward"), ("?", "search backward"), ("n", "next match"), ("N", "prev match"),
        (":", "command mode"), ("v", "visual"), ("V", "visual line"),
    ] {
        t.add(UxBinding::new(n, key, action));
    }
    t
}

/// Check coverage: are all expected keys bound?
pub fn check_mode_coverage(table: &ModeBindingTable, mode: UxMode, expected_keys: &[&str]) -> Vec<String> {
    expected_keys.iter()
        .filter(|k| table.find_key(mode, k).is_none())
        .map(|k| format!("missing binding: {} in {:?}", k, mode))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_normal() {
        let t = build_normal_bindings();
        assert!(t.count() >= 25);
    }

    #[test]
    fn find_key() {
        let t = build_normal_bindings();
        assert!(t.find_key(UxMode::Normal, "h").is_some());
        assert!(t.find_key(UxMode::Normal, "nonexistent").is_none());
    }

    #[test]
    fn for_mode_filter() {
        let t = build_normal_bindings();
        assert!(t.for_mode(UxMode::Insert).is_empty());
        assert!(!t.for_mode(UxMode::Normal).is_empty());
    }

    #[test]
    fn coverage_check() {
        let t = build_normal_bindings();
        let missing = check_mode_coverage(&t, UxMode::Normal, &["h", "j", "k", "l"]);
        assert!(missing.is_empty());
    }

    #[test]
    fn coverage_missing() {
        let t = ModeBindingTable::new();
        let missing = check_mode_coverage(&t, UxMode::Normal, &["h"]);
        assert_eq!(missing.len(), 1);
    }

    #[test]
    fn undocumented() {
        let mut t = ModeBindingTable::new();
        let mut b = UxBinding::new(UxMode::Normal, "q", "record macro");
        b.documented = false; t.add(b);
        assert_eq!(t.undocumented().len(), 1);
    }

    #[test]
    fn all_modes() {
        let modes = [UxMode::Normal, UxMode::Insert, UxMode::Visual, UxMode::VisualLine,
            UxMode::VisualBlock, UxMode::Replace, UxMode::Command, UxMode::OperatorPending, UxMode::Terminal];
        assert_eq!(modes.len(), 9);
    }
}
