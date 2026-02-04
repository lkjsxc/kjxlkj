//! Editor mode enumeration.

use serde::{Deserialize, Serialize};

/// Editor modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode - navigation and commands.
    Normal,
    /// Insert mode - text entry.
    Insert,
    /// Visual mode - character-wise selection.
    Visual,
    /// Visual line mode - line-wise selection.
    VisualLine,
    /// Visual block mode - rectangular selection.
    VisualBlock,
    /// Command mode - Ex command entry.
    Command,
    /// Replace mode - overwrite text.
    Replace,
}

impl Mode {
    /// Check if this mode is a visual variant.
    pub fn is_visual(self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Check if this mode accepts text input.
    pub fn is_insert_like(self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
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
    fn mode_default_is_normal() {
        assert_eq!(Mode::default(), Mode::Normal);
    }

    #[test]
    fn visual_modes_detected() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn insert_like_modes() {
        assert!(Mode::Insert.is_insert_like());
        assert!(Mode::Replace.is_insert_like());
        assert!(!Mode::Normal.is_insert_like());
    }
}
