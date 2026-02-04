//! Editor modes.

use serde::{Deserialize, Serialize};

/// The current editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode for navigation and commands.
    #[default]
    Normal,
    /// Insert mode for text entry.
    Insert,
    /// Replace mode for overwriting text.
    Replace,
    /// Command-line mode for Ex commands.
    Command,
    /// Visual mode (charwise selection).
    Visual,
    /// Visual line mode (linewise selection).
    VisualLine,
    /// Visual block mode (rectangular selection).
    VisualBlock,
}

impl Mode {
    /// Returns true if this is any visual mode.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Returns true if this is an insertion mode.
    pub fn is_insert_like(&self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }

    /// Mode name for display.
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Replace => "REPLACE",
            Mode::Command => "COMMAND",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn mode_name() {
        assert_eq!(Mode::Normal.name(), "NORMAL");
        assert_eq!(Mode::Insert.name(), "INSERT");
    }
}
