//! Undo tree implementation.

/// An undo entry storing a text snapshot.
#[derive(Debug, Clone)]
pub struct UndoEntry {
    /// The full text content at this point.
    pub content: String,
    /// Cursor line at this point.
    pub cursor_line: usize,
    /// Cursor grapheme offset at this point.
    pub cursor_offset: usize,
}

/// Linear undo tree (can be extended to branching later).
#[derive(Debug)]
pub struct UndoTree {
    entries: Vec<UndoEntry>,
    /// Index of the current active entry, or usize::MAX if empty.
    current: usize,
    max_depth: usize,
}

impl UndoTree {
    pub fn new(max_depth: usize) -> Self {
        Self {
            entries: Vec::new(),
            current: usize::MAX,
            max_depth,
        }
    }

    /// Push a new undo checkpoint.
    pub fn push(&mut self, entry: UndoEntry) {
        // Truncate any redo history
        if self.current != usize::MAX {
            self.entries.truncate(self.current + 1);
        } else {
            self.entries.clear();
        }
        self.entries.push(entry);
        if self.entries.len() > self.max_depth {
            self.entries.remove(0);
        }
        self.current = self.entries.len() - 1;
    }

    /// Undo: return the previous state if available.
    pub fn undo(&mut self) -> Option<&UndoEntry> {
        if self.current != usize::MAX && self.current > 0 {
            self.current -= 1;
            self.entries.get(self.current)
        } else {
            None
        }
    }

    /// Redo: return the next state if available.
    pub fn redo(&mut self) -> Option<&UndoEntry> {
        if self.current != usize::MAX && self.current < self.entries.len().saturating_sub(1) {
            let entry = self.entries.get(self.current);
            self.current += 1;
            entry
        } else if self.current == usize::MAX && !self.entries.is_empty() {
            self.current = 0;
            self.entries.first()
        } else {
            None
        }
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        self.current != usize::MAX && self.current > 0
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        self.current != usize::MAX && self.current < self.entries.len().saturating_sub(1)
    }
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(s: &str) -> UndoEntry {
        UndoEntry {
            content: s.to_string(),
            cursor_line: 0,
            cursor_offset: 0,
        }
    }

    #[test]
    fn test_undo_redo() {
        let mut tree = UndoTree::new(100);
        tree.push(entry("a"));
        tree.push(entry("ab"));
        tree.push(entry("abc"));

        let e = tree.undo().unwrap();
        assert_eq!(e.content, "ab");
        let e = tree.undo().unwrap();
        assert_eq!(e.content, "a");
        assert!(tree.undo().is_none());

        let e = tree.redo().unwrap();
        assert_eq!(e.content, "a");
        let e = tree.redo().unwrap();
        assert_eq!(e.content, "ab");
    }
}
