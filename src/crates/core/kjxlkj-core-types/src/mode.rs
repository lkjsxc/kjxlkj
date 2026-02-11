//! Mode state machine definitions.
//!
//! See /docs/spec/modes/transitions.md for the normative transition table.

use serde::{Deserialize, Serialize};

/// Visual selection sub-mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualKind {
    Char,
    Line,
    Block,
}

/// Command-line sub-mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandKind {
    Ex,
    Search,
}

/// Pending operator awaiting motion/text-object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    Delete,
    Change,
    Yank,
    Indent,
    Dedent,
    Reindent,
    Format,
}

/// The editing mode state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Normal,
    Insert,
    Replace,
    Visual(VisualKind),
    Command(CommandKind),
    OperatorPending(Operator),
    TerminalInsert,
    InsertNormal,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

impl Mode {
    /// Human-readable mode name for statusline display.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Insert => "INSERT",
            Self::Replace => "REPLACE",
            Self::Visual(VisualKind::Char) => "VISUAL",
            Self::Visual(VisualKind::Line) => "V-LINE",
            Self::Visual(VisualKind::Block) => "V-BLOCK",
            Self::Command(CommandKind::Ex) => "COMMAND",
            Self::Command(CommandKind::Search) => "SEARCH",
            Self::OperatorPending(_) => "OP-PENDING",
            Self::TerminalInsert => "TERMINAL",
            Self::InsertNormal => "INS-NORMAL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mode_is_normal() {
        assert_eq!(Mode::default(), Mode::Normal);
    }

    #[test]
    fn mode_display_names() {
        assert_eq!(Mode::Normal.display_name(), "NORMAL");
        assert_eq!(Mode::Insert.display_name(), "INSERT");
        assert_eq!(
            Mode::Visual(VisualKind::Block).display_name(),
            "V-BLOCK"
        );
    }
}
