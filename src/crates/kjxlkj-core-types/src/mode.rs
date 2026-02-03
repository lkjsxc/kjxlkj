//! Editor mode types.

use serde::{Deserialize, Serialize};

use crate::CursorStyle;

/// Editor modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Mode {
    /// Normal mode - navigation and operators.
    #[default]
    Normal,
    /// Insert mode - text entry.
    Insert,
    /// Visual mode - character-wise selection.
    Visual,
    /// Visual line mode - line-wise selection.
    VisualLine,
    /// Visual block mode - block selection.
    VisualBlock,
    /// Command mode - ex command entry.
    Command,
    /// Replace mode - overwrite semantics.
    Replace,
}

impl Mode {
    /// Get the cursor style for this mode.
    pub fn cursor_style(self) -> CursorStyle {
        match self {
            Mode::Normal => CursorStyle::Block,
            Mode::Insert => CursorStyle::Bar,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => CursorStyle::Hollow,
            Mode::Command => CursorStyle::Block,
            Mode::Replace => CursorStyle::Underline,
        }
    }

    /// Get the mode name for display.
    pub fn name(self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Command => "COMMAND",
            Mode::Replace => "REPLACE",
        }
    }

    /// Check if this is a visual mode variant.
    pub fn is_visual(self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Check if this mode allows text insertion.
    pub fn is_insert(self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }
}

/// Insert mode entry variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsertVariant {
    /// Insert before cursor (i).
    BeforeCursor,
    /// Insert after cursor (a).
    AfterCursor,
    /// Insert at line start (I).
    LineStart,
    /// Insert at line end (A).
    LineEnd,
    /// Open line below (o).
    OpenBelow,
    /// Open line above (O).
    OpenAbove,
}
