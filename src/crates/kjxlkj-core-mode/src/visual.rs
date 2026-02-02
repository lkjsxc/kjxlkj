//! Visual mode state.

use kjxlkj_core_types::Position;
use serde::{Deserialize, Serialize};

/// Visual mode state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualState {
    /// Anchor position (selection start).
    pub anchor: Option<Position>,
    /// Current position (selection end).
    pub cursor: Option<Position>,
}

impl VisualState {
    /// Creates a new visual mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the state.
    pub fn reset(&mut self) {
        self.anchor = None;
        self.cursor = None;
    }

    /// Starts selection at a position.
    pub fn start(&mut self, pos: Position) {
        self.anchor = Some(pos);
        self.cursor = Some(pos);
    }

    /// Updates the cursor position.
    pub fn update_cursor(&mut self, pos: Position) {
        self.cursor = Some(pos);
    }

    /// Returns the selection range (normalized).
    pub fn range(&self) -> Option<kjxlkj_core_types::Range> {
        match (self.anchor, self.cursor) {
            (Some(anchor), Some(cursor)) => {
                Some(kjxlkj_core_types::Range::new(anchor, cursor).normalized())
            }
            _ => None,
        }
    }
}
