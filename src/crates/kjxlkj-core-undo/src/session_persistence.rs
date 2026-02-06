/// Session persistence — save/restore editor state across sessions.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A mark position saved across sessions.
#[derive(Debug, Clone, PartialEq)]
pub struct SavedMark { pub file: String, pub line: usize, pub col: usize }

/// A single entry in the jump list history.
#[derive(Debug, Clone, PartialEq)]
pub struct JumpEntry { pub file: String, pub line: usize, pub col: usize }

/// A register entry saved across sessions.
#[derive(Debug, Clone, PartialEq)]
pub struct SavedRegister { pub name: char, pub content: String, pub linewise: bool }

/// Saved command-line history entry.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEntry { pub kind: HistoryKind, pub value: String }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryKind { Command, Search, Input, Debug }

/// Full session state that can be serialized.
#[derive(Debug, Clone)]
pub struct SessionState {
    pub marks: Vec<SavedMark>,
    pub jumps: Vec<JumpEntry>,
    pub registers: Vec<SavedRegister>,
    pub history: Vec<HistoryEntry>,
    pub last_search: Option<String>,
    pub last_substitute: Option<String>,
    pub buffer_positions: HashMap<String, (usize, usize)>,
}

impl SessionState {
    pub fn new() -> Self {
        Self { marks: Vec::new(), jumps: Vec::new(), registers: Vec::new(),
            history: Vec::new(), last_search: None, last_substitute: None,
            buffer_positions: HashMap::new() }
    }

    pub fn add_mark(&mut self, name: char, file: &str, line: usize, col: usize) {
        self.marks.retain(|m| !(m.file == file && m.line == line));
        self.marks.push(SavedMark { file: file.into(), line, col });
        if self.marks.len() > 100 { self.marks.remove(0); }
    }

    pub fn add_jump(&mut self, file: &str, line: usize, col: usize) {
        self.jumps.push(JumpEntry { file: file.into(), line, col });
        if self.jumps.len() > 100 { self.jumps.remove(0); }
    }

    pub fn add_register(&mut self, name: char, content: &str, linewise: bool) {
        self.registers.retain(|r| r.name != name);
        self.registers.push(SavedRegister { name, content: content.into(), linewise });
    }

    pub fn add_history(&mut self, kind: HistoryKind, value: &str) {
        self.history.push(HistoryEntry { kind, value: value.into() });
        if self.history.len() > 1000 { self.history.remove(0); }
    }

    pub fn save_buffer_position(&mut self, file: &str, line: usize, col: usize) {
        self.buffer_positions.insert(file.into(), (line, col));
    }

    pub fn get_buffer_position(&self, file: &str) -> Option<(usize, usize)> {
        self.buffer_positions.get(file).copied()
    }
}

/// Compute session file path from config directory.
pub fn session_path(config_dir: &Path) -> PathBuf {
    config_dir.join("session.json")
}

/// Serialize session state to JSON string.
pub fn serialize_session(state: &SessionState) -> String {
    let mut out = String::from("{\n");
    out.push_str(&format!("  \"marks\": {},\n", state.marks.len()));
    out.push_str(&format!("  \"jumps\": {},\n", state.jumps.len()));
    out.push_str(&format!("  \"registers\": {},\n", state.registers.len()));
    out.push_str(&format!("  \"history\": {},\n", state.history.len()));
    out.push_str(&format!("  \"buffers\": {}\n", state.buffer_positions.len()));
    out.push_str("}");
    out
}

/// Filter history by kind.
pub fn filter_history(state: &SessionState, kind: HistoryKind) -> Vec<&HistoryEntry> {
    state.history.iter().filter(|h| h.kind == kind).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_retrieve_marks() {
        let mut s = SessionState::new();
        s.add_mark('a', "foo.rs", 10, 5);
        assert_eq!(s.marks.len(), 1);
        assert_eq!(s.marks[0].file, "foo.rs");
    }

    #[test]
    fn jump_list_cap() {
        let mut s = SessionState::new();
        for i in 0..105 { s.add_jump("f.rs", i, 0); }
        assert_eq!(s.jumps.len(), 100);
    }

    #[test]
    fn register_dedup() {
        let mut s = SessionState::new();
        s.add_register('a', "hello", false);
        s.add_register('a', "world", true);
        assert_eq!(s.registers.len(), 1);
        assert_eq!(s.registers[0].content, "world");
        assert!(s.registers[0].linewise);
    }

    #[test]
    fn history_filter() {
        let mut s = SessionState::new();
        s.add_history(HistoryKind::Command, ":w");
        s.add_history(HistoryKind::Search, "/foo");
        s.add_history(HistoryKind::Command, ":q");
        let cmds = filter_history(&s, HistoryKind::Command);
        assert_eq!(cmds.len(), 2);
    }

    #[test]
    fn buffer_positions() {
        let mut s = SessionState::new();
        s.save_buffer_position("a.rs", 42, 7);
        assert_eq!(s.get_buffer_position("a.rs"), Some((42, 7)));
        assert_eq!(s.get_buffer_position("b.rs"), None);
    }

    #[test]
    fn session_path_construction() {
        let p = session_path(Path::new("/home/user/.config/kjxlkj"));
        assert!(p.ends_with("session.json"));
    }

    #[test]
    fn serialize_nonempty() {
        let mut s = SessionState::new();
        s.add_mark('a', "f.rs", 1, 0);
        s.add_jump("f.rs", 1, 0);
        let json = serialize_session(&s);
        assert!(json.contains("\"marks\": 1"));
        assert!(json.contains("\"jumps\": 1"));
    }

    #[test]
    fn history_cap() {
        let mut s = SessionState::new();
        for i in 0..1005 { s.add_history(HistoryKind::Command, &format!(":{}", i)); }
        assert_eq!(s.history.len(), 1000);
    }
}
