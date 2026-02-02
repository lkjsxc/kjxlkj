//! Mode types.

use serde::{Deserialize, Serialize};

/// Editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode: navigation and commands.
    #[default]
    Normal,
    /// Insert mode: text entry.
    Insert,
    /// Visual mode: character selection.
    Visual,
    /// Visual line mode: line selection.
    VisualLine,
    /// Visual block mode: block selection.
    VisualBlock,
    /// Command mode: ex commands.
    Command,
    /// Replace mode: overwrite text.
    Replace,
    /// Operator-pending mode: waiting for motion.
    OperatorPending,
}

impl Mode {
    /// Returns true if this is a visual mode variant.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Returns true if this is insert or replace mode.
    pub fn is_insert_like(&self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }

    /// Returns the mode name for display.
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Command => "COMMAND",
            Mode::Replace => "REPLACE",
            Mode::OperatorPending => "O-PENDING",
        }
    }
}
