//! Editor modes.

use serde::{Deserialize, Serialize};

/// Editor mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode (default).
    #[default]
    Normal,
    /// Insert mode.
    Insert,
    /// Visual (character-wise) mode.
    Visual,
    /// Visual line mode.
    VisualLine,
    /// Visual block mode.
    VisualBlock,
    /// Command-line mode.
    Command,
    /// Search mode.
    Search,
    /// Replace mode.
    Replace,
}

impl Mode {
    /// Check if this is a visual mode.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Check if cursor should be end-inclusive.
    pub fn is_end_inclusive(&self) -> bool {
        matches!(self, Mode::Normal | Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Get the mode indicator string.
    pub fn indicator(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Command => "COMMAND",
            Mode::Search => "SEARCH",
            Mode::Replace => "REPLACE",
        }
    }
}
