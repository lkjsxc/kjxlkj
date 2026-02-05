//! Cursor types and semantics.

use serde::{Deserialize, Serialize};

/// Cursor position in buffer coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CursorPosition {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed).
    pub column: usize,
}

impl CursorPosition {
    /// Create a new cursor position.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Create cursor at origin.
    pub fn origin() -> Self {
        Self { line: 0, column: 0 }
    }
}

/// Cursor state including preferred column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CursorState {
    /// Current position.
    pub position: CursorPosition,
    /// Preferred column for vertical movement.
    pub preferred_column: Option<usize>,
}

impl CursorState {
    /// Create a new cursor state.
    pub fn new(position: CursorPosition) -> Self {
        Self {
            position,
            preferred_column: None,
        }
    }

    /// Update position and clear preferred column.
    pub fn move_to(&mut self, position: CursorPosition) {
        self.position = position;
        self.preferred_column = None;
    }

    /// Update position preserving preferred column.
    pub fn move_vertical(&mut self, line: usize, line_length: usize) {
        let target_col = self.preferred_column.unwrap_or(self.position.column);
        self.preferred_column = Some(target_col);
        self.position.line = line;
        self.position.column = target_col.min(line_length.saturating_sub(1).max(0));
    }
}

/// Cursor shape for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CursorShape {
    /// Block cursor (Normal mode).
    #[default]
    Block,
    /// Bar cursor (Insert mode).
    Bar,
    /// Hollow block (Visual mode).
    Hollow,
    /// Underline (Replace mode).
    Underline,
}
