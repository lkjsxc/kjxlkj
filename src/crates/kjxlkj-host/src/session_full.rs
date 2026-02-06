/// Full session features — session save/restore, mksession, views persistence.

use std::collections::HashMap;

/// Session data for :mksession / :source.
#[derive(Debug, Clone)]
pub struct SessionData {
    pub name: String,
    pub working_dir: String,
    pub buffers: Vec<SessionBuffer>,
    pub windows: Vec<SessionWindow>,
    pub tab_count: usize,
    pub active_tab: usize,
    pub global_marks: HashMap<char, SessionMark>,
}

/// Buffer reference within a session.
#[derive(Debug, Clone)]
pub struct SessionBuffer { pub id: u64, pub path: Option<String>, pub cursor_line: usize, pub cursor_col: usize }

/// Window layout within a session.
#[derive(Debug, Clone)]
pub struct SessionWindow { pub buf_id: u64, pub width: u16, pub height: u16, pub is_active: bool }

/// Mark position in a session.
#[derive(Debug, Clone)]
pub struct SessionMark { pub buf_id: u64, pub line: usize, pub col: usize }

impl SessionData {
    pub fn new(name: impl Into<String>, wd: impl Into<String>) -> Self {
        Self { name: name.into(), working_dir: wd.into(), buffers: Vec::new(),
            windows: Vec::new(), tab_count: 1, active_tab: 0, global_marks: HashMap::new() }
    }

    pub fn add_buffer(&mut self, id: u64, path: Option<String>, line: usize, col: usize) {
        self.buffers.push(SessionBuffer { id, path, cursor_line: line, cursor_col: col });
    }

    pub fn add_window(&mut self, buf_id: u64, w: u16, h: u16, active: bool) {
        self.windows.push(SessionWindow { buf_id, width: w, height: h, is_active: active });
    }

    pub fn set_mark(&mut self, ch: char, buf_id: u64, line: usize, col: usize) {
        self.global_marks.insert(ch, SessionMark { buf_id, line, col });
    }

    pub fn buffer_count(&self) -> usize { self.buffers.len() }
    pub fn window_count(&self) -> usize { self.windows.len() }
}

/// Serialize session to a vimscript-like save format.
pub fn serialize_session(data: &SessionData) -> String {
    let mut out = String::new();
    out.push_str(&format!("\" Session: {}\n", data.name));
    out.push_str(&format!("cd {}\n", data.working_dir));
    for buf in &data.buffers {
        if let Some(ref p) = buf.path {
            out.push_str(&format!("edit {}\n", p));
            out.push_str(&format!("call cursor({}, {})\n", buf.cursor_line + 1, buf.cursor_col + 1));
        }
    }
    out
}

/// Parse simple session lines back into buffer paths.
pub fn parse_session_buffers(script: &str) -> Vec<String> {
    script.lines().filter_map(|l| l.strip_prefix("edit ").map(|p| p.to_string())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_creation() {
        let s = SessionData::new("default", "/home/user/project");
        assert_eq!(s.buffer_count(), 0); assert_eq!(s.tab_count, 1);
    }

    #[test]
    fn add_buffers_and_windows() {
        let mut s = SessionData::new("s1", "/tmp");
        s.add_buffer(1, Some("foo.rs".into()), 10, 5);
        s.add_window(1, 80, 24, true);
        assert_eq!(s.buffer_count(), 1); assert_eq!(s.window_count(), 1);
    }

    #[test]
    fn global_marks() {
        let mut s = SessionData::new("s1", "/tmp");
        s.set_mark('A', 1, 10, 0);
        assert!(s.global_marks.contains_key(&'A'));
    }

    #[test]
    fn serialize() {
        let mut s = SessionData::new("test", "/tmp");
        s.add_buffer(1, Some("main.rs".into()), 0, 0);
        let out = serialize_session(&s);
        assert!(out.contains("edit main.rs"));
        assert!(out.contains("cd /tmp"));
    }

    #[test]
    fn parse_buffers() {
        let script = "\" Session: test\ncd /tmp\nedit foo.rs\ncall cursor(1, 1)\nedit bar.rs\n";
        let bufs = parse_session_buffers(script);
        assert_eq!(bufs, vec!["foo.rs", "bar.rs"]);
    }

    #[test]
    fn empty_session_serialize() {
        let s = SessionData::new("empty", "/");
        let out = serialize_session(&s);
        assert!(out.contains("cd /"));
        assert!(!out.contains("edit "));
    }

    #[test]
    fn session_buffer_no_path() {
        let mut s = SessionData::new("s", "/tmp");
        s.add_buffer(1, None, 0, 0);
        let out = serialize_session(&s);
        assert!(!out.contains("edit"));
    }
}
