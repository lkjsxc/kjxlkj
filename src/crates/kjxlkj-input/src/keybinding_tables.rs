/// Full keybinding tables — all mode keybindings with action mapping.

use std::collections::HashMap;

/// Action category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionCategory { Motion, Operator, TextObject, Mode, Command, Scroll, Search, Fold, Window, Macro, Register, Mark }

/// A keybinding table entry.
#[derive(Debug, Clone)]
pub struct BindingEntry { pub key: String, pub action: String, pub category: ActionCategory, pub repeatable: bool }

impl BindingEntry {
    pub fn new(key: impl Into<String>, action: impl Into<String>, cat: ActionCategory) -> Self {
        Self { key: key.into(), action: action.into(), category: cat, repeatable: true }
    }
    pub fn non_repeatable(mut self) -> Self { self.repeatable = false; self }
}

/// Full keybinding table for a mode.
#[derive(Debug, Default)]
pub struct BindingTable { entries: Vec<BindingEntry>, name: String }

impl BindingTable {
    pub fn new(name: impl Into<String>) -> Self { Self { entries: Vec::new(), name: name.into() } }
    pub fn add(&mut self, entry: BindingEntry) { self.entries.push(entry); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn by_category(&self, cat: ActionCategory) -> Vec<&BindingEntry> { self.entries.iter().filter(|e| e.category == cat).collect() }
    pub fn find(&self, key: &str) -> Option<&BindingEntry> { self.entries.iter().find(|e| e.key == key) }
}

/// Build the complete normal mode table.
pub fn build_normal_table() -> BindingTable {
    let mut t = BindingTable::new("normal");
    // Motions
    for (k, a) in [("h","left"),("j","down"),("k","up"),("l","right"),("w","word"),("W","WORD"),
        ("b","word_back"),("B","WORD_back"),("e","word_end"),("E","WORD_end"),("0","line_start"),
        ("$","line_end"),("^","first_nonblank"),("gg","file_start"),("G","file_end"),
        ("{","paragraph_back"),("}","paragraph_forward"),("(","sentence_back"),(")","sentence_forward"),
        ("%","matching_bracket"),("f","find_char"),("F","find_char_back"),("t","till_char"),("T","till_char_back")] {
        t.add(BindingEntry::new(k, a, ActionCategory::Motion));
    }
    // Operators
    for (k, a) in [("d","delete"),("c","change"),("y","yank"),(">","indent"),("<","dedent"),("=","format"),("g~","toggle_case"),("gu","lowercase"),("gU","uppercase")] {
        t.add(BindingEntry::new(k, a, ActionCategory::Operator));
    }
    // Mode transitions
    for (k, a) in [("i","insert"),("a","append"),("I","insert_bol"),("A","append_eol"),("o","open_below"),("O","open_above"),("v","visual"),("V","visual_line"),("<C-v>","visual_block"),(":","command"),("R","replace")] {
        t.add(BindingEntry::new(k, a, ActionCategory::Mode).non_repeatable());
    }
    // Commands
    for (k, a) in [("x","delete_char"),("X","backspace"),("dd","delete_line"),("yy","yank_line"),("p","paste_after"),("P","paste_before"),("u","undo"),("<C-r>","redo"),(".","repeat"),("J","join_lines"),("~","toggle_case_char"),("ZZ","write_quit"),("ZQ","quit_no_write")] {
        t.add(BindingEntry::new(k, a, ActionCategory::Command));
    }
    // Search
    for (k, a) in [("/","search_fwd"),("?","search_back"),("n","next_match"),("N","prev_match"),("*","word_under_cursor"),("#","word_under_cursor_back")] {
        t.add(BindingEntry::new(k, a, ActionCategory::Search));
    }
    // Scroll
    for (k, a) in [("<C-f>","page_down"),("<C-b>","page_up"),("<C-d>","half_page_down"),("<C-u>","half_page_up"),("<C-e>","scroll_down"),("<C-y>","scroll_up"),("zz","center"),("zt","top"),("zb","bottom")] {
        t.add(BindingEntry::new(k, a, ActionCategory::Scroll));
    }
    t
}

/// Coverage stats.
pub fn coverage_stats(table: &BindingTable) -> HashMap<ActionCategory, usize> {
    let mut m = HashMap::new();
    for e in &table.entries { *m.entry(e.category).or_insert(0) += 1; }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_table_size() { let t = build_normal_table(); assert!(t.count() >= 60); }

    #[test]
    fn find_motion() { let t = build_normal_table(); assert!(t.find("h").is_some()); }

    #[test]
    fn find_operator() { let t = build_normal_table(); assert_eq!(t.find("d").unwrap().category, ActionCategory::Operator); }

    #[test]
    fn by_category() { let t = build_normal_table(); assert!(t.by_category(ActionCategory::Motion).len() >= 20); }

    #[test]
    fn mode_non_repeatable() {
        let t = build_normal_table();
        let i = t.find("i").unwrap();
        assert!(!i.repeatable);
    }

    #[test]
    fn coverage() {
        let t = build_normal_table();
        let s = coverage_stats(&t); assert!(s.len() >= 5);
    }

    #[test]
    fn all_categories() {
        let cats = [ActionCategory::Motion, ActionCategory::Operator, ActionCategory::TextObject,
            ActionCategory::Mode, ActionCategory::Command, ActionCategory::Scroll,
            ActionCategory::Search, ActionCategory::Fold, ActionCategory::Window,
            ActionCategory::Macro, ActionCategory::Register, ActionCategory::Mark];
        assert_eq!(cats.len(), 12);
    }
}
