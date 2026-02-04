//! Mark storage.

use kjxlkj_core_types::Position;
use std::collections::HashMap;

/// Store for marks.
#[derive(Debug, Clone, Default)]
pub struct MarkStore {
    /// Local marks (a-z).
    local: HashMap<char, Position>,
    /// Global marks (A-Z).
    global: HashMap<char, (String, Position)>,
}

impl MarkStore {
    /// Create a new mark store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a local mark.
    pub fn set_local(&mut self, mark: char, pos: Position) {
        if mark.is_ascii_lowercase() {
            self.local.insert(mark, pos);
        }
    }

    /// Get a local mark.
    pub fn get_local(&self, mark: char) -> Option<Position> {
        self.local.get(&mark).copied()
    }

    /// Set a global mark.
    pub fn set_global(&mut self, mark: char, file: String, pos: Position) {
        if mark.is_ascii_uppercase() {
            self.global.insert(mark, (file, pos));
        }
    }

    /// Get a global mark.
    pub fn get_global(&self, mark: char) -> Option<(&str, Position)> {
        self.global.get(&mark).map(|(f, p)| (f.as_str(), *p))
    }

    /// Get any mark.
    pub fn get(&self, mark: char) -> Option<Position> {
        if mark.is_ascii_lowercase() {
            self.get_local(mark)
        } else if mark.is_ascii_uppercase() {
            self.get_global(mark).map(|(_, p)| p)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_marks() {
        let mut store = MarkStore::new();
        store.set_local('a', Position::new(10, 5));
        assert_eq!(store.get_local('a'), Some(Position::new(10, 5)));
        assert_eq!(store.get_local('b'), None);
    }

    #[test]
    fn global_marks() {
        let mut store = MarkStore::new();
        store.set_global('A', "file.txt".to_string(), Position::new(1, 2));
        let result = store.get_global('A');
        assert!(result.is_some());
        let (file, pos) = result.unwrap();
        assert_eq!(file, "file.txt");
        assert_eq!(pos, Position::new(1, 2));
    }
}
