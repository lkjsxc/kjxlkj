//! Mark storage for `m{a-z}` / `'{a-z}` / `` `{a-z}` ``.
//!
//! See /docs/spec/editing/marks/README.md.

use std::collections::HashMap;

/// A stored mark position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkPos {
    pub line: usize,
    pub col: usize,
}

/// Per-buffer mark storage. Lowercase marks (a-z) are local to a buffer.
#[derive(Debug, Clone, Default)]
pub struct MarkStore {
    marks: HashMap<char, MarkPos>,
}

impl MarkStore {
    pub fn new() -> Self { Self { marks: HashMap::new() } }

    /// Set mark `name` at the given position.
    pub fn set(&mut self, name: char, line: usize, col: usize) {
        if name.is_ascii_lowercase() {
            self.marks.insert(name, MarkPos { line, col });
        }
    }

    /// Get mark position by name.
    pub fn get(&self, name: char) -> Option<MarkPos> {
        self.marks.get(&name).copied()
    }

    /// Remove a mark.
    pub fn remove(&mut self, name: char) {
        self.marks.remove(&name);
    }

    /// List all marks in sorted order.
    pub fn list(&self) -> Vec<(char, MarkPos)> {
        let mut v: Vec<_> = self.marks.iter().map(|(&c, &p)| (c, p)).collect();
        v.sort_by_key(|(c, _)| *c);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn set_and_get_mark() {
        let mut ms = MarkStore::new();
        ms.set('a', 5, 3);
        let p = ms.get('a').unwrap();
        assert_eq!(p, MarkPos { line: 5, col: 3 });
    }

    #[test] fn get_unset_returns_none() {
        let ms = MarkStore::new();
        assert!(ms.get('z').is_none());
    }

    #[test] fn uppercase_ignored() {
        let mut ms = MarkStore::new();
        ms.set('A', 1, 0);
        assert!(ms.get('A').is_none());
    }

    #[test] fn remove_mark() {
        let mut ms = MarkStore::new();
        ms.set('b', 2, 0);
        ms.remove('b');
        assert!(ms.get('b').is_none());
    }

    #[test] fn list_sorted() {
        let mut ms = MarkStore::new();
        ms.set('c', 3, 0);
        ms.set('a', 1, 0);
        ms.set('b', 2, 0);
        let l = ms.list();
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].0, 'a');
        assert_eq!(l[1].0, 'b');
        assert_eq!(l[2].0, 'c');
    }
}
