//! Jumplist for navigation history.

use crate::marks::Mark;

/// Maximum jumplist size.
const MAX_JUMPLIST_SIZE: usize = 100;

/// A jumplist for navigation history.
#[derive(Debug, Default)]
pub struct JumpList {
    /// List of jump locations.
    jumps: Vec<Mark>,
    /// Current position in the jumplist (-1 means at the end).
    index: isize,
}

impl JumpList {
    /// Creates a new empty jumplist.
    pub fn new() -> Self {
        Self {
            jumps: Vec::new(),
            index: -1,
        }
    }

    /// Adds a jump location.
    pub fn push(&mut self, mark: Mark) {
        // If we're not at the end, truncate
        if self.index >= 0 {
            let idx = self.index as usize;
            if idx < self.jumps.len() {
                self.jumps.truncate(idx + 1);
            }
        }

        // Don't add duplicate if same as last
        if self.jumps.last() == Some(&mark) {
            return;
        }

        self.jumps.push(mark);

        // Limit size
        if self.jumps.len() > MAX_JUMPLIST_SIZE {
            self.jumps.remove(0);
        }

        // Reset index to end
        self.index = -1;
    }

    /// Goes to the older jump (Ctrl-O).
    pub fn older(&mut self) -> Option<Mark> {
        if self.jumps.is_empty() {
            return None;
        }

        if self.index == -1 {
            self.index = self.jumps.len() as isize - 1;
        } else if self.index > 0 {
            self.index -= 1;
        } else {
            return None;
        }

        self.jumps.get(self.index as usize).copied()
    }

    /// Goes to the newer jump (Ctrl-I).
    pub fn newer(&mut self) -> Option<Mark> {
        if self.index == -1 || self.jumps.is_empty() {
            return None;
        }

        if (self.index as usize) < self.jumps.len() - 1 {
            self.index += 1;
            self.jumps.get(self.index as usize).copied()
        } else {
            self.index = -1;
            None
        }
    }

    /// Returns the current jump location.
    pub fn current(&self) -> Option<Mark> {
        if self.index >= 0 && (self.index as usize) < self.jumps.len() {
            Some(self.jumps[self.index as usize])
        } else {
            self.jumps.last().copied()
        }
    }

    /// Returns all jumps.
    pub fn all(&self) -> &[Mark] {
        &self.jumps
    }

    /// Returns the current index (0-based from end, -1 means at end).
    pub fn position(&self) -> isize {
        self.index
    }

    /// Returns the number of jumps.
    pub fn len(&self) -> usize {
        self.jumps.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.jumps.is_empty()
    }

    /// Clears the jumplist.
    pub fn clear(&mut self) {
        self.jumps.clear();
        self.index = -1;
    }
}
