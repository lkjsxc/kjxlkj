//! Editor modes.

use serde::{Deserialize, Serialize};

/// Editor mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode (navigation and commands).
    #[default]
    Normal,
    /// Insert mode (text insertion).
    Insert,
    /// Command-line mode (Ex commands).
    Command,
    /// Visual mode (character-wise selection).
    Visual,
    /// Visual line mode (line-wise selection).
    VisualLine,
    /// Visual block mode (rectangular selection).
    VisualBlock,
    /// Replace mode (overwrite characters).
    Replace,
}

impl Mode {
    /// Check if this is a visual mode variant.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Check if this is an insert-like mode.
    pub fn is_insert(&self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }

    /// Get the display name for the mode.
    pub fn display_name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Command => "COMMAND",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Replace => "REPLACE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn test_mode_display() {
        assert_eq!(Mode::Normal.display_name(), "NORMAL");
        assert_eq!(Mode::Insert.display_name(), "INSERT");
    }
}
