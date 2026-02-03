//! Cursor types and semantics.

use serde::{Deserialize, Serialize};

use crate::Position;

/// Cursor state within a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// Current cursor position (line, column).
    pub position: Position,
    /// Desired column for vertical movement (sticky column).
    pub desired_col: u32,
}

impl Cursor {
    /// Create a new cursor at the given position.
    pub fn new(line: u32, col: u32) -> Self {
        Self {
            position: Position::new(line, col),
            desired_col: col,
        }
    }

    /// Move cursor to a new position, updating desired column.
    pub fn move_to(&mut self, line: u32, col: u32) {
        self.position = Position::new(line, col);
        self.desired_col = col;
    }

    /// Move cursor vertically, preserving desired column.
    pub fn move_vertical(&mut self, line: u32, max_col: u32) {
        self.position.line = line;
        self.position.col = self.desired_col.min(max_col);
    }

    /// Move cursor horizontally, updating desired column.
    pub fn move_horizontal(&mut self, col: u32) {
        self.position.col = col;
        self.desired_col = col;
    }

    /// Get the current line.
    pub fn line(&self) -> u32 {
        self.position.line
    }

    /// Get the current column.
    pub fn col(&self) -> u32 {
        self.position.col
    }
}

/// Cursor style for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CursorStyle {
    /// Block cursor (Normal mode).
    #[default]
    Block,
    /// Bar cursor (Insert mode).
    Bar,
    /// Hollow block cursor (Visual mode).
    Hollow,
    /// Underline cursor (Replace mode).
    Underline,
}
