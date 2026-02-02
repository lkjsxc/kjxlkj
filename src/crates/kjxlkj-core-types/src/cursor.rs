//! Cursor types.

use crate::Position;
use serde::{Deserialize, Serialize};

/// Cursor state within a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// Current position.
    pub position: Position,
    /// Desired column (for vertical movement).
    pub desired_col: Option<usize>,
}

impl Cursor {
    /// Creates a new cursor at the given position.
    pub fn new(position: Position) -> Self {
        Self {
            position,
            desired_col: None,
        }
    }

    /// Creates a cursor at the origin.
    pub fn origin() -> Self {
        Self::default()
    }

    /// Creates a cursor at the given line and column.
    pub fn at(line: usize, col: usize) -> Self {
        Self::new(Position::new(line, col))
    }

    /// Returns the line number.
    pub fn line(&self) -> usize {
        self.position.line
    }

    /// Returns the column number.
    pub fn col(&self) -> usize {
        self.position.col
    }

    /// Sets the desired column for vertical movement.
    pub fn with_desired_col(mut self, col: usize) -> Self {
        self.desired_col = Some(col);
        self
    }

    /// Clears the desired column.
    pub fn clear_desired_col(mut self) -> Self {
        self.desired_col = None;
        self
    }
}
