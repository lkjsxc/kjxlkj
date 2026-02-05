//! Cursor state.

use crate::Position;
use serde::{Deserialize, Serialize};

/// Cursor state within a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(Serialize, Deserialize)]
pub struct Cursor {
    /// Current position.
    pub position: Position,
    /// Target column for vertical movement (sticky column).
    pub target_column: Option<usize>,
}

impl Cursor {
    /// Create a cursor at origin.
    pub const fn origin() -> Self {
        Self {
            position: Position::origin(),
            target_column: None,
        }
    }

    /// Create a cursor at a specific position.
    pub const fn at(line: usize, column: usize) -> Self {
        Self {
            position: Position::new(line, column),
            target_column: None,
        }
    }

    /// Get the line index.
    pub fn line(&self) -> usize {
        self.position.line
    }

    /// Get the column index.
    pub fn column(&self) -> usize {
        self.position.column
    }
}
