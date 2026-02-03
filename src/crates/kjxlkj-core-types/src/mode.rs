//! Editor modes.

use serde::{Deserialize, Serialize};

/// The current editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode - navigation and commands.
    Normal,
    /// Insert mode - text entry.
    Insert,
    /// Visual mode - character selection.
    Visual,
    /// Visual line mode - line selection.
    VisualLine,
    /// Visual block mode - rectangular selection.
    VisualBlock,
    /// Command mode - Ex command entry.
    Command,
    /// Replace mode - overwrite text.
    Replace,
}

impl Mode {
    /// Returns true if this is an insert-like mode.
    pub fn is_insert(self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }

    /// Returns true if this is a visual mode variant.
    pub fn is_visual(self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Returns the mode name for display.
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
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_is_insert() {
        assert!(Mode::Insert.is_insert());
        assert!(Mode::Replace.is_insert());
        assert!(!Mode::Normal.is_insert());
    }

    #[test]
    fn mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }
}
