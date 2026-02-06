//! Undo/redo tree with branching history.

pub mod branch;

use std::time::Instant;

/// A unique identifier for an undo node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UndoNodeId(pub u64);

/// A single entry in the undo tree, representing one atomic change.
#[derive(Debug, Clone)]
pub struct UndoEntry {
    /// Forward patch data (opaque bytes, to be interpreted by the edit layer).
    pub forward: Vec<u8>,
    /// Reverse patch data.
    pub reverse: Vec<u8>,
    /// Timestamp of the change.
    pub timestamp: Instant,
}

/// A tree-structured undo history that supports branching.
pub struct UndoTree {
    entries: Vec<UndoEntry>,
    current: usize,
}

impl UndoTree {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
        }
    }

    /// Push a new undo entry, truncating any redo history beyond the current point.
    pub fn push(&mut self, entry: UndoEntry) {
        self.entries.truncate(self.current);
        self.entries.push(entry);
        self.current = self.entries.len();
    }

    /// Undo one step, returning the entry if available.
    pub fn undo(&mut self) -> Option<&UndoEntry> {
        if self.current == 0 {
            return None;
        }
        self.current -= 1;
        Some(&self.entries[self.current])
    }

    /// Redo one step, returning the entry if available.
    pub fn redo(&mut self) -> Option<&UndoEntry> {
        if self.current >= self.entries.len() {
            return None;
        }
        let entry = &self.entries[self.current];
        self.current += 1;
        Some(entry)
    }

    /// Returns true if there are entries to undo.
    pub fn can_undo(&self) -> bool {
        self.current > 0
    }

    /// Returns true if there are entries to redo.
    pub fn can_redo(&self) -> bool {
        self.current < self.entries.len()
    }

    /// Total number of entries in the tree.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new()
    }
}
