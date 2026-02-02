//! Session management.

use std::collections::HashMap;

use super::session::Session;

/// Session manager.
#[derive(Debug, Default)]
pub struct SessionManager {
    sessions: HashMap<String, Session>,
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
        self.current.as_ref().and_then(|n| self.sessions.get(n))
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

    #[test]
    fn test_session_manager_remove() {
        let mut mgr = SessionManager::new();
        mgr.save(Session::new("test"));
        assert!(mgr.remove("test"));
        assert!(!mgr.remove("test"));
    }
}
