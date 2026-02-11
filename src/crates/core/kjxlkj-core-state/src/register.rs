//! Register storage for yank/delete/put operations.
//! See /docs/spec/editing/registers/registers.md.

use std::collections::HashMap;
use kjxlkj_core_types::RangeType;

/// A single register entry: text + scope type.
#[derive(Debug, Clone)]
pub struct RegisterEntry {
    pub text: String,
    pub scope: RangeType,
}

/// Register store: unnamed `"`, yank `0`, delete `1`-`9`, small `-`,
/// named `a`-`z`/`A`-`Z`, blackhole `_`, clipboard `+`/`*`,
/// read-only `.`/`%`/`#`/`:`/`/`.
#[derive(Debug, Clone, Default)]
pub struct RegisterStore {
    regs: HashMap<char, RegisterEntry>,
    pub selected: Option<char>,
}

impl RegisterStore {
    pub fn new() -> Self { Self::default() }
    pub fn get(&self, name: char) -> Option<&RegisterEntry> { self.regs.get(&name) }
    pub fn effective(&self) -> char { self.selected.unwrap_or('"') }
    pub fn clear_selection(&mut self) { self.selected = None; }

    pub fn write(&mut self, name: char, text: String, scope: RangeType) {
        if name.is_ascii_uppercase() {
            let lower = name.to_ascii_lowercase();
            if let Some(e) = self.regs.get_mut(&lower) {
                e.text.push_str(&text); e.scope = scope; return;
            }
            self.regs.insert(lower, RegisterEntry { text, scope });
            return;
        }
        self.regs.insert(name, RegisterEntry { text, scope });
    }

    /// Set a read-only register (system use only).
    pub fn set_readonly(&mut self, name: char, text: String) {
        self.regs.insert(name, RegisterEntry { text, scope: RangeType::Characterwise });
    }

    /// Record yank: unnamed `"` + `0`. Blackhole suppresses.
    pub fn record_yank(&mut self, text: String, scope: RangeType) {
        let reg = self.effective();
        if reg == '_' { self.clear_selection(); return; }
        self.write(reg, text.clone(), scope);
        if reg == '"' { self.write('0', text, scope); }
        self.clear_selection();
    }

    /// Record delete: unnamed `"`, rotate 1-9 if linewise, else `-`.
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
            if let Some(entry) = self.regs.remove(&src) { self.regs.insert(dst, entry); }
        }
        self.write('1', text, scope);
    }

    pub fn list_all(&self) -> Vec<(char, &RegisterEntry)> {
        let mut out: Vec<_> = self.regs.iter().map(|(&k, v)| (k, v)).collect();
        out.sort_by_key(|(k, _)| *k);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn store() -> RegisterStore { RegisterStore::new() }
    #[test]
    fn yank_writes_unnamed_and_zero() {
        let mut rs = store();
        rs.record_yank("hello".into(), RangeType::Characterwise);
        assert_eq!(rs.get('"').unwrap().text, "hello");
        assert_eq!(rs.get('0').unwrap().text, "hello");
    }
    #[test]
    fn delete_linewise_rotates() {
        let mut rs = store();
        rs.record_delete("first\n".into(), RangeType::Linewise);
        assert_eq!(rs.get('1').unwrap().text, "first\n");
        rs.record_delete("second\n".into(), RangeType::Linewise);
        assert_eq!(rs.get('1').unwrap().text, "second\n");
        assert_eq!(rs.get('2').unwrap().text, "first\n");
    }
    #[test]
    fn delete_small_writes_minus() {
        let mut rs = store();
        rs.record_delete("ab".into(), RangeType::Characterwise);
        assert_eq!(rs.get('-').unwrap().text, "ab");
    }
    #[test]
    fn append_register() {
        let mut rs = store();
        rs.write('a', "foo".into(), RangeType::Characterwise);
        rs.write('A', "bar".into(), RangeType::Characterwise);
        assert_eq!(rs.get('a').unwrap().text, "foobar");
    }
    #[test]
    fn selected_register_used() {
        let mut rs = store();
        rs.selected = Some('a');
        rs.record_yank("text".into(), RangeType::Characterwise);
        assert_eq!(rs.get('a').unwrap().text, "text");
        assert!(rs.selected.is_none());
    }
    #[test]
    fn blackhole_yank_suppresses() {
        let mut rs = store();
        rs.record_yank("before".into(), RangeType::Characterwise);
        rs.selected = Some('_');
        rs.record_yank("gone".into(), RangeType::Characterwise);
        assert_eq!(rs.get('"').unwrap().text, "before");
    }
    #[test]
    fn blackhole_delete_suppresses() {
        let mut rs = store();
        rs.record_yank("keep".into(), RangeType::Characterwise);
        rs.selected = Some('_');
        rs.record_delete("discard".into(), RangeType::Characterwise);
        assert_eq!(rs.get('"').unwrap().text, "keep");
    }
    #[test]
    fn clipboard_regs() {
        let mut rs = store();
        rs.selected = Some('+');
        rs.record_yank("clip".into(), RangeType::Characterwise);
        assert_eq!(rs.get('+').unwrap().text, "clip");
        rs.selected = Some('*');
        rs.record_yank("sel".into(), RangeType::Characterwise);
        assert_eq!(rs.get('*').unwrap().text, "sel");
    }
    #[test]
    fn readonly_registers() {
        let mut rs = store();
        rs.set_readonly('.', "inserted".into());
        rs.set_readonly(':', "write".into());
        rs.set_readonly('/', "pattern".into());
        rs.set_readonly('%', "file.txt".into());
        assert_eq!(rs.get('.').unwrap().text, "inserted");
        assert_eq!(rs.get(':').unwrap().text, "write");
        assert_eq!(rs.get('/').unwrap().text, "pattern");
        assert_eq!(rs.get('%').unwrap().text, "file.txt");
    }
    #[test]
    fn numbered_rotation_fills_nine() {
        let mut rs = store();
        for i in 1..=10 {
            rs.record_delete(format!("del{i}\n"), RangeType::Linewise);
        }
        assert_eq!(rs.get('1').unwrap().text, "del10\n");
        assert_eq!(rs.get('9').unwrap().text, "del2\n");
    }
    #[test]
    fn yank_does_not_rotate_numbered() {
        let mut rs = store();
        rs.record_delete("del\n".into(), RangeType::Linewise);
        rs.record_yank("yank".into(), RangeType::Characterwise);
        assert_eq!(rs.get('1').unwrap().text, "del\n");
        assert_eq!(rs.get('0').unwrap().text, "yank");
    }
    #[test]
    fn named_yank_skips_zero() {
        let mut rs = store();
        rs.selected = Some('b');
        rs.record_yank("named".into(), RangeType::Characterwise);
        assert_eq!(rs.get('b').unwrap().text, "named");
        assert!(rs.get('0').is_none());
    }
    #[test]
    fn list_all_returns_sorted() {
        let mut rs = store();
        rs.write('z', "z".into(), RangeType::Characterwise);
        rs.write('a', "a".into(), RangeType::Characterwise);
        let list = rs.list_all();
        assert!(list[0].0 < list[1].0);
    }
}
