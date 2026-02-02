//! Session persistence.
//!
//! Saving and restoring editor state between sessions.

use std::collections::HashMap;
use std::path::PathBuf;

/// Saved window state.
#[derive(Debug, Clone)]
pub struct SavedWindow {
    /// Buffer path.
    pub buffer: Option<PathBuf>,
    /// Cursor position (line, col).
    pub cursor: (usize, usize),
    /// Window dimensions.
    pub size: (u16, u16),
}

impl SavedWindow {
    /// Creates a new saved window.
    pub fn new(buffer: Option<PathBuf>, cursor: (usize, usize)) -> Self {
        Self {
            buffer,
            cursor,
            size: (80, 24),
        }
    }
}

/// Saved tab state.
#[derive(Debug, Clone)]
pub struct SavedTab {
    /// Windows in this tab.
    pub windows: Vec<SavedWindow>,
    /// Current window index.
    pub current: usize,
}

impl SavedTab {
    /// Creates a new saved tab.
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            current: 0,
        }
    }

    /// Adds a window.
    pub fn add_window(&mut self, window: SavedWindow) {
        self.windows.push(window);
    }
}

impl Default for SavedTab {
    fn default() -> Self {
        Self::new()
    }
}

/// Session data.
#[derive(Debug, Clone)]
pub struct Session {
    /// Session name.
    pub name: String,
    /// Tabs.
    pub tabs: Vec<SavedTab>,
    /// Current tab index.
    pub current_tab: usize,
    /// Global marks.
    pub marks: HashMap<char, PathBuf>,
    /// Working directory.
    pub cwd: Option<PathBuf>,
}

impl Session {
    /// Creates a new session.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tabs: Vec::new(),
            current_tab: 0,
            marks: HashMap::new(),
            cwd: None,
        }
    }

    /// Creates a default session.
    pub fn default_session() -> Self {
        Self::new("default")
    }

    /// Adds a tab.
    pub fn add_tab(&mut self, tab: SavedTab) {
        self.tabs.push(tab);
    }

    /// Returns whether session is empty.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    /// Returns number of open files.
    pub fn file_count(&self) -> usize {
        self.tabs
            .iter()
            .flat_map(|t| t.windows.iter())
            .filter(|w| w.buffer.is_some())
            .count()
    }

    /// Sets the working directory.
    pub fn set_cwd(&mut self, path: PathBuf) {
        self.cwd = Some(path);
    }

    /// Adds a global mark.
    pub fn set_mark(&mut self, mark: char, path: PathBuf) {
        self.marks.insert(mark, path);
    }
}

/// Session manager.
#[derive(Debug, Default)]
pub struct SessionManager {
    /// Available sessions.
    sessions: HashMap<String, Session>,
    /// Current session name.
    current: Option<String>,
}

impl SessionManager {
    /// Creates a new session manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Saves a session.
    pub fn save(&mut self, session: Session) {
        let name = session.name.clone();
        self.sessions.insert(name, session);
    }

    /// Loads a session by name.
    pub fn load(&mut self, name: &str) -> Option<&Session> {
        if self.sessions.contains_key(name) {
            self.current = Some(name.to_string());
            self.sessions.get(name)
        } else {
            None
        }
    }

    /// Returns the current session.
    pub fn current(&self) -> Option<&Session> {
        self.current
            .as_ref()
            .and_then(|n| self.sessions.get(n))
    }

    /// Lists session names.
    pub fn list(&self) -> Vec<&str> {
        self.sessions.keys().map(|s| s.as_str()).collect()
    }

    /// Removes a session.
    pub fn remove(&mut self, name: &str) -> bool {
        self.sessions.remove(name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saved_window() {
        let win = SavedWindow::new(Some(PathBuf::from("/tmp/test.rs")), (10, 5));
        assert_eq!(win.cursor, (10, 5));
    }

    #[test]
    fn test_saved_tab() {
        let mut tab = SavedTab::new();
        tab.add_window(SavedWindow::new(None, (0, 0)));
        assert_eq!(tab.windows.len(), 1);
    }

    #[test]
    fn test_session_new() {
        let session = Session::new("test");
        assert_eq!(session.name, "test");
        assert!(session.is_empty());
    }

    #[test]
    fn test_session_file_count() {
        let mut session = Session::new("test");
        let mut tab = SavedTab::new();
        tab.add_window(SavedWindow::new(Some(PathBuf::from("/a.rs")), (0, 0)));
        tab.add_window(SavedWindow::new(None, (0, 0)));
        session.add_tab(tab);
        assert_eq!(session.file_count(), 1);
    }

    #[test]
    fn test_session_manager_save_load() {
        let mut mgr = SessionManager::new();
        let session = Session::new("test");
        mgr.save(session);
        assert!(mgr.load("test").is_some());
    }

    #[test]
    fn test_session_manager_list() {
        let mut mgr = SessionManager::new();
        mgr.save(Session::new("a"));
        mgr.save(Session::new("b"));
        assert_eq!(mgr.list().len(), 2);
    }
}
