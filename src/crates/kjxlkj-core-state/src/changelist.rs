//! Changelist for change position history.

use crate::marks::Mark;

/// Maximum changelist size.
const MAX_CHANGELIST_SIZE: usize = 100;

/// A changelist for change position history.
#[derive(Debug, Default)]
pub struct ChangeList {
    /// List of change locations.
    changes: Vec<Mark>,
    /// Current position in the changelist.
    index: isize,
}

impl ChangeList {
    /// Creates a new empty changelist.
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            index: -1,
        }
    }

    /// Adds a change location.
    pub fn push(&mut self, mark: Mark) {
        // Don't add duplicate if same as last
        if self.changes.last() == Some(&mark) {
            return;
        }

        self.changes.push(mark);

        // Limit size
        if self.changes.len() > MAX_CHANGELIST_SIZE {
            self.changes.remove(0);
        }

        // Reset to end
        self.index = -1;
    }

    /// Goes to older change (g;).
    pub fn older(&mut self) -> Option<Mark> {
        if self.changes.is_empty() {
            return None;
        }

        if self.index == -1 {
            self.index = self.changes.len() as isize - 1;
        } else if self.index > 0 {
            self.index -= 1;
        } else {
            return None;
        }

        self.changes.get(self.index as usize).copied()
    }

    /// Goes to newer change (g,).
    pub fn newer(&mut self) -> Option<Mark> {
        if self.index == -1 || self.changes.is_empty() {
            return None;
        }

        if (self.index as usize) < self.changes.len() - 1 {
            self.index += 1;
            self.changes.get(self.index as usize).copied()
        } else {
            self.index = -1;
            None
        }
    }

    /// Returns all changes.
    pub fn all(&self) -> &[Mark] {
        &self.changes
    }

    /// Returns the number of changes.
    pub fn len(&self) -> usize {
        self.changes.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}
