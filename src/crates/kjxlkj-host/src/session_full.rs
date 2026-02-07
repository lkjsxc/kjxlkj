//! Full session persistence: serialize/deserialize editor sessions.

use serde::{Deserialize, Serialize};

/// A buffer's state within a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBuffer {
    pub path: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

/// A window's state within a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionWindow {
    pub buffer_index: usize,
    pub width: u16,
    pub height: u16,
}

/// A named mark referencing a position in a buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMark {
    pub name: String,
    pub buffer_path: String,
    pub line: usize,
    pub col: usize,
}

/// Complete session data for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub buffers: Vec<SessionBuffer>,
    pub windows: Vec<SessionWindow>,
    pub globals: Vec<SessionMark>,
    pub cwd: String,
}

impl SessionData {
    /// Create an empty session.
    pub fn empty() -> Self {
        Self {
            buffers: Vec::new(),
            windows: Vec::new(),
            globals: Vec::new(),
            cwd: String::new(),
        }
    }
}

/// Serialize session data to a JSON string.
pub fn serialize_session(data: &SessionData) -> String {
    serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".into())
}

/// Parse buffer paths from a session file's content.
pub fn parse_session_buffers(content: &str) -> Vec<String> {
    let data: Result<SessionData, _> = serde_json::from_str(content);
    match data {
        Ok(s) => s.buffers.into_iter().map(|b| b.path).collect(),
        Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_session() -> SessionData {
        SessionData {
            buffers: vec![
                SessionBuffer {
                    path: "main.rs".into(),
                    cursor_line: 10,
                    cursor_col: 5,
                },
                SessionBuffer {
                    path: "lib.rs".into(),
                    cursor_line: 0,
                    cursor_col: 0,
                },
            ],
            windows: vec![SessionWindow {
                buffer_index: 0,
                width: 80,
                height: 24,
            }],
            globals: vec![SessionMark {
                name: "A".into(),
                buffer_path: "main.rs".into(),
                line: 10,
                col: 5,
            }],
            cwd: "/home/user/project".into(),
        }
    }

    #[test]
    fn serialize_roundtrip() {
        let data = sample_session();
        let json = serialize_session(&data);
        let paths = parse_session_buffers(&json);
        assert_eq!(paths, vec!["main.rs", "lib.rs"]);
    }

    #[test]
    fn empty_session() {
        let data = SessionData::empty();
        let json = serialize_session(&data);
        assert!(json.contains("buffers"));
    }

    #[test]
    fn parse_invalid_json() {
        let paths = parse_session_buffers("not json");
        assert!(paths.is_empty());
    }

    #[test]
    fn parse_empty_json() {
        let paths = parse_session_buffers("{}");
        // Missing fields => deserialization fails => empty vec.
        assert!(paths.is_empty());
    }

    #[test]
    fn session_mark_fields() {
        let m = SessionMark {
            name: "B".into(),
            buffer_path: "foo.rs".into(),
            line: 3,
            col: 7,
        };
        assert_eq!(m.name, "B");
        assert_eq!(m.line, 3);
    }

    #[test]
    fn session_window_fields() {
        let w = SessionWindow {
            buffer_index: 1,
            width: 120,
            height: 40,
        };
        assert_eq!(w.buffer_index, 1);
    }
}
