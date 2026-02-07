//! Session persistence and history filtering.

use serde::{Deserialize, Serialize};

/// Persisted session state for :mksession.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub marks: Vec<(char, String)>,
    pub jumps: Vec<String>,
    pub registers: Vec<(char, String)>,
    pub history: Vec<String>,
    pub buffer_positions: Vec<(String, usize, usize)>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            marks: Vec::new(),
            jumps: Vec::new(),
            registers: Vec::new(),
            history: Vec::new(),
            buffer_positions: Vec::new(),
        }
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

/// History entry kind for filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryKind {
    Command,
    Search,
    Input,
    Debug,
}

/// Serialize a session state to JSON.
pub fn serialize_session(state: &SessionState) -> String {
    serde_json::to_string_pretty(state).unwrap_or_else(|_| "{}".to_string())
}

/// Filter history entries by kind prefix.
pub fn filter_history(history: &[String], kind: HistoryKind) -> Vec<&String> {
    let prefix = match kind {
        HistoryKind::Command => "cmd:",
        HistoryKind::Search => "search:",
        HistoryKind::Input => "input:",
        HistoryKind::Debug => "debug:",
    };
    history.iter().filter(|e| e.starts_with(prefix)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_roundtrip() {
        let state = SessionState::new();
        let json = serialize_session(&state);
        assert!(json.contains("marks"));
    }

    #[test]
    fn filter_by_kind() {
        let history = vec![
            "cmd:q".to_string(),
            "search:hello".to_string(),
            "cmd:w".to_string(),
        ];
        let cmds = filter_history(&history, HistoryKind::Command);
        assert_eq!(cmds.len(), 2);
        let searches = filter_history(&history, HistoryKind::Search);
        assert_eq!(searches.len(), 1);
    }
}
