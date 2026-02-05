//! Editor modes.

use serde::{Deserialize, Serialize};

/// Editor mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode - navigation and commands.
    #[default]
    Normal,
    /// Insert mode - text entry.
    Insert,
    /// Visual mode - character-wise selection.
    Visual,
    /// Visual line mode - line-wise selection.
    VisualLine,
    /// Visual block mode - column selection.
    VisualBlock,
    /// Replace mode - overwrite characters.
    Replace,
    /// Command mode - ex commands.
    Command,
}

impl Mode {
    /// Check if this is a visual mode.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Check if this mode uses end-inclusive cursor.
    pub fn is_end_inclusive(&self) -> bool {
        matches!(self, Mode::Insert)
    }

    /// Get the mode name for display.
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Replace => "REPLACE",
            Mode::Command => "COMMAND",
        }
    }
}
