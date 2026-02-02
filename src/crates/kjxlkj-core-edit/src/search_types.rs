//! Search types.
//!
//! Types for search direction and matches.

use serde::{Deserialize, Serialize};

/// Search direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchDirection {
    /// Forward search (/).
    Forward,
    /// Backward search (?).
    Backward,
}

impl Default for SearchDirection {
    fn default() -> Self {
        Self::Forward
    }
}

/// A search match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMatch {
    /// Line number (0-indexed).
    pub line: usize,
    /// Start column (byte offset).
    pub start: usize,
    /// End column (byte offset).
    pub end: usize,
}

impl SearchMatch {
    /// Creates a new match.
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self { line, start, end }
    }

    /// Returns the length of the match.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns if the match is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
