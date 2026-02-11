//! Register storage for yank/delete/put operations.
//!
//! See /docs/spec/editing/operators/operators.md.

use std::collections::HashMap;
use kjxlkj_core_types::RangeType;

/// A single register entry: text + scope type.
#[derive(Debug, Clone)]
pub struct RegisterEntry {
    pub text: String,
    pub scope: RangeType,
}

/// Register store managing all named/numbered registers.
///
/// `"` — unnamed (default); `0` — last yank;
/// `1`–`9` — rotating delete stack (≥1 line);
/// `-` — small delete (<1 line); `a`–`z`/`A`–`Z` — named.
#[derive(Debug, Clone, Default)]
pub struct RegisterStore {
    regs: HashMap<char, RegisterEntry>,
    /// Currently selected register prefix (`"x`).
    pub selected: Option<char>,
}

impl RegisterStore {
    pub fn new() -> Self { Self::default() }

    /// Get the register entry for a given name.
    pub fn get(&self, name: char) -> Option<&RegisterEntry> {
        self.regs.get(&name)
    }

    /// Get the effective register (selected or unnamed).
    pub fn effective(&self) -> char {
        self.selected.unwrap_or('"')
    }

    /// Clear the selected register prefix.
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// Write text to a register, replacing its contents.
    pub fn write(&mut self, name: char, text: String, scope: RangeType) {
        if name.is_ascii_uppercase() {
            let lower = name.to_ascii_lowercase();
            if let Some(e) = self.regs.get_mut(&lower) {
                e.text.push_str(&text);
                e.scope = scope;
                return;
            }
            self.regs.insert(lower, RegisterEntry { text, scope });
            return;
        }
        self.regs.insert(name, RegisterEntry { text, scope });
    }

    /// Record a yank: writes to unnamed `"` and `0`.
    /// Blackhole `_` suppresses all writes.
    pub fn record_yank(&mut self, text: String, scope: RangeType) {
        let reg = self.effective();
        if reg == '_' { self.clear_selection(); return; }
        self.write(reg, text.clone(), scope);
        if reg == '"' { self.write('0', text, scope); }
        self.clear_selection();
    }

    /// Record a delete: writes to unnamed `"`, rotates
    /// 1–9 if linewise/multiline, else writes to `-`.
    /// Blackhole `_` suppresses all writes.
    pub fn record_delete(&mut self, text: String, scope: RangeType) {
        let reg = self.effective();
        if reg == '_' { self.clear_selection(); return; }
        self.write(reg, text.clone(), scope);
        if reg == '"' {
            if scope == RangeType::Linewise || text.contains('\n') {
                self.rotate_numbered(text, scope);
            } else {
                self.write('-', text, scope);
            }
        }
        self.clear_selection();
    }

    fn rotate_numbered(&mut self, text: String, scope: RangeType) {
        for i in (1..9).rev() {
            let src = char::from_digit(i, 10).unwrap();
            let dst = char::from_digit(i + 1, 10).unwrap();
            if let Some(entry) = self.regs.remove(&src) {
                self.regs.insert(dst, entry);
            }
        }
        self.write('1', text, scope);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yank_writes_unnamed_and_zero() {
        let mut rs = RegisterStore::new();
        rs.record_yank("hello".into(), RangeType::Characterwise);
        assert_eq!(rs.get('"').unwrap().text, "hello");
        assert_eq!(rs.get('0').unwrap().text, "hello");
    }

    #[test]
    fn delete_linewise_rotates() {
        let mut rs = RegisterStore::new();
        rs.record_delete("first\n".into(), RangeType::Linewise);
        assert_eq!(rs.get('1').unwrap().text, "first\n");
        rs.record_delete("second\n".into(), RangeType::Linewise);
        assert_eq!(rs.get('1').unwrap().text, "second\n");
        assert_eq!(rs.get('2').unwrap().text, "first\n");
    }

    #[test]
    fn delete_small_writes_minus() {
        let mut rs = RegisterStore::new();
        rs.record_delete("ab".into(), RangeType::Characterwise);
        assert_eq!(rs.get('-').unwrap().text, "ab");
    }

    #[test]
    fn append_register() {
        let mut rs = RegisterStore::new();
        rs.write('a', "foo".into(), RangeType::Characterwise);
        rs.write('A', "bar".into(), RangeType::Characterwise);
        assert_eq!(rs.get('a').unwrap().text, "foobar");
    }

    #[test]
    fn selected_register_used() {
        let mut rs = RegisterStore::new();
        rs.selected = Some('a');
        rs.record_yank("text".into(), RangeType::Characterwise);
        assert_eq!(rs.get('a').unwrap().text, "text");
        assert!(rs.selected.is_none());
    }

    #[test]
    fn blackhole_yank_suppresses() {
        let mut rs = RegisterStore::new();
        rs.record_yank("before".into(), RangeType::Characterwise);
        rs.selected = Some('_');
        rs.record_yank("gone".into(), RangeType::Characterwise);
        // Unnamed should still have "before", not "gone"
        assert_eq!(rs.get('"').unwrap().text, "before");
        assert!(rs.get('_').is_none());
    }

    #[test]
    fn blackhole_delete_suppresses() {
        let mut rs = RegisterStore::new();
        rs.record_yank("keep".into(), RangeType::Characterwise);
        rs.selected = Some('_');
        rs.record_delete("discard".into(), RangeType::Characterwise);
        assert_eq!(rs.get('"').unwrap().text, "keep");
        assert!(rs.get('-').is_none());
    }

    #[test]
    fn clipboard_regs_store_normally() {
        let mut rs = RegisterStore::new();
        rs.selected = Some('+');
        rs.record_yank("clip".into(), RangeType::Characterwise);
        assert_eq!(rs.get('+').unwrap().text, "clip");
        rs.selected = Some('*');
        rs.record_yank("sel".into(), RangeType::Characterwise);
        assert_eq!(rs.get('*').unwrap().text, "sel");
    }
}
