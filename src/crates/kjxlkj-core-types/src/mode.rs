//! Mode enumeration and visual sub-mode definitions.

use serde::{Deserialize, Serialize};

use crate::Operator;

/// Visual selection sub-mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualKind {
    /// Character-wise selection (`v`).
    Char,
    /// Line-wise selection (`V`).
    Line,
    /// Block-wise selection (`Ctrl-v`).
    Block,
}

/// Command-line sub-mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandKind {
    /// Ex command (`:` prefix).
    Ex,
    /// Forward search (`/` prefix).
    SearchForward,
    /// Backward search (`?` prefix).
    SearchBackward,
}

/// The modal state of the editor.
///
/// Transitions are deterministic and never block on external IO.
/// See /docs/spec/modes/transitions.md for the full transition table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    /// Navigation, operator composition, command entry.
    Normal,
    /// Text entry with completion and auto-pairs.
    Insert,
    /// Overwrite character by character.
    Replace,
    /// Selection mode with sub-kind.
    Visual(VisualKind),
    /// Ex command or search input.
    Command(CommandKind),
    /// Waiting for motion/text-object after operator key.
    OperatorPending(Operator),
    /// Forwarding input to PTY (terminal window focused).
    TerminalInsert,
    /// Single normal-mode command from insert mode (`Ctrl-O`).
    InsertNormal,
}

impl Mode {
    /// Whether this mode uses end-exclusive cursor positioning.
    pub fn is_end_exclusive(&self) -> bool {
        matches!(self, Mode::Normal | Mode::Visual(_) | Mode::Replace)
    }

    /// Whether this mode uses end-inclusive (insertion-point) cursor.
    pub fn is_end_inclusive(&self) -> bool {
        matches!(self, Mode::Insert | Mode::InsertNormal)
    }

    /// Human-readable mode indicator for the statusline.
    pub fn display_name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Replace => "REPLACE",
            Mode::Visual(VisualKind::Char) => "VISUAL",
            Mode::Visual(VisualKind::Line) => "V-LINE",
            Mode::Visual(VisualKind::Block) => "V-BLOCK",
            Mode::Command(CommandKind::Ex) => "COMMAND",
            Mode::Command(CommandKind::SearchForward) => "SEARCH",
            Mode::Command(CommandKind::SearchBackward) => "SEARCH",
            Mode::OperatorPending(_) => "OP-PENDING",
            Mode::TerminalInsert => "TERMINAL",
            Mode::InsertNormal => "INS-NORMAL",
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
    fn normal_is_end_exclusive() {
        assert!(Mode::Normal.is_end_exclusive());
    }

    #[test]
    fn insert_is_end_inclusive() {
        assert!(Mode::Insert.is_end_inclusive());
    }

    #[test]
    fn display_names() {
        assert_eq!(Mode::Normal.display_name(), "NORMAL");
        assert_eq!(Mode::Insert.display_name(), "INSERT");
        assert_eq!(Mode::Visual(VisualKind::Block).display_name(), "V-BLOCK");
    }
}
