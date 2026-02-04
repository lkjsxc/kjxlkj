//! Mark storage.

use kjxlkj_core_types::Cursor;
use std::collections::HashMap;

/// Storage for marks.
#[derive(Debug, Default)]
pub struct MarkStore {
    /// Local marks (a-z).
    local: HashMap<char, Cursor>,
    /// Global marks (A-Z).
    global: HashMap<char, (String, Cursor)>,
    /// Special marks.
    special: HashMap<char, Cursor>,
}

impl MarkStore {
    /// Create a new mark store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a local mark.
    pub fn set_local(&mut self, name: char, cursor: Cursor) {
        if name.is_ascii_lowercase() {
            self.local.insert(name, cursor);
        }
    }

    /// Get a local mark.
    pub fn get_local(&self, name: char) -> Option<Cursor> {
        self.local.get(&name.to_ascii_lowercase()).copied()
    }

    /// Set a global mark.
    pub fn set_global(&mut self, name: char, file: String, cursor: Cursor) {
        if name.is_ascii_uppercase() {
            self.global.insert(name, (file, cursor));
        }
    }

    /// Get a global mark.
    pub fn get_global(&self, name: char) -> Option<(&str, Cursor)> {
        self.global
            .get(&name.to_ascii_uppercase())
            .map(|(f, c)| (f.as_str(), *c))
    }

    /// Set a special mark.
    pub fn set_special(&mut self, name: char, cursor: Cursor) {
        self.special.insert(name, cursor);
    }

    /// Get a special mark.
    pub fn get_special(&self, name: char) -> Option<Cursor> {
        self.special.get(&name).copied()
    }

    /// Get mark by name (local, global, or special).
    pub fn get(&self, name: char) -> Option<Cursor> {
        if name.is_ascii_lowercase() {
            self.get_local(name)
        } else if name.is_ascii_uppercase() {
            self.get_global(name).map(|(_, c)| c)
        } else {
            self.get_special(name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_marks() {
        let mut store = MarkStore::new();
        store.set_local('a', Cursor::new(5, 3));
        assert_eq!(store.get_local('a'), Some(Cursor::new(5, 3)));
        assert_eq!(store.get_local('b'), None);
    }
}
