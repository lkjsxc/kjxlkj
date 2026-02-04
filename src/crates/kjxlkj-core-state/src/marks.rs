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

    #[test]
    fn get_any_mark_local() {
        let mut store = MarkStore::new();
        store.set_local('b', Position::new(5, 3));
        assert_eq!(store.get('b'), Some(Position::new(5, 3)));
    }

    #[test]
    fn get_any_mark_global() {
        let mut store = MarkStore::new();
        store.set_global('B', "f.txt".to_string(), Position::new(2, 1));
        assert_eq!(store.get('B'), Some(Position::new(2, 1)));
    }

    #[test]
    fn invalid_mark_char_ignored() {
        let mut store = MarkStore::new();
        store.set_local('1', Position::new(0, 0)); // invalid
        assert!(store.get_local('1').is_none());
    }

    #[test]
    fn mark_overwrite() {
        let mut store = MarkStore::new();
        store.set_local('x', Position::new(1, 1));
        store.set_local('x', Position::new(2, 2));
        assert_eq!(store.get_local('x'), Some(Position::new(2, 2)));
    }

    #[test]
    fn uppercase_not_local() {
        let mut store = MarkStore::new();
        store.set_local('A', Position::new(0, 0));
        assert!(store.get_local('A').is_none());
    }

    #[test]
    fn lowercase_not_global() {
        let mut store = MarkStore::new();
        store.set_global('a', "f.txt".to_string(), Position::new(0, 0));
        assert!(store.get_global('a').is_none());
    }

    #[test]
    fn get_nonexistent_mark() {
        let store = MarkStore::new();
        assert!(store.get('z').is_none());
        assert!(store.get('Z').is_none());
    }

    #[test]
    fn set_multiple_local_marks() {
        let mut store = MarkStore::new();
        store.set_local('a', Position::new(0, 0));
        store.set_local('b', Position::new(1, 1));
        store.set_local('c', Position::new(2, 2));
        assert_eq!(store.get_local('a'), Some(Position::new(0, 0)));
        assert_eq!(store.get_local('b'), Some(Position::new(1, 1)));
        assert_eq!(store.get_local('c'), Some(Position::new(2, 2)));
    }

    #[test]
    fn local_mark_returns_position() {
        let mut store = MarkStore::new();
        store.set_local('m', Position::new(5, 10));
        if let Some(p) = store.get_local('m') {
            assert_eq!(p.line, 5);
        }
    }

    #[test]
    fn global_mark_stores_filename() {
        let mut store = MarkStore::new();
        store.set_global('A', "file.txt".to_string(), Position::new(1, 2));
        let result = store.get_global('A');
        assert!(result.is_some());
    }

    #[test]
    fn get_returns_local_for_lowercase() {
        let mut store = MarkStore::new();
        store.set_local('a', Position::new(0, 0));
        assert!(store.get('a').is_some());
    }

    #[test]
    fn get_returns_global_for_uppercase() {
        let mut store = MarkStore::new();
        store.set_global('Z', "test.txt".to_string(), Position::new(0, 0));
        assert!(store.get('Z').is_some());
    }

    #[test]
    fn mark_store_empty_initially() {
        let store = MarkStore::new();
        assert!(store.get('a').is_none());
    }

    #[test]
    fn overwrite_local_mark() {
        let mut store = MarkStore::new();
        store.set_local('x', Position::new(1, 1));
        store.set_local('x', Position::new(2, 2));
        assert_eq!(store.get_local('x'), Some(Position::new(2, 2)));
    }
}
