//! Modal state machine types.

use serde::{Deserialize, Serialize};

/// The editor's current mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Replace,
    Visual(VisualKind),
    Command(CommandKind),
    OperatorPending(Operator),
    TerminalInsert,
    InsertNormal,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Replace => write!(f, "REPLACE"),
            Mode::Visual(VisualKind::Char) => write!(f, "VISUAL"),
            Mode::Visual(VisualKind::Line) => write!(f, "V-LINE"),
            Mode::Visual(VisualKind::Block) => write!(f, "V-BLOCK"),
            Mode::Command(CommandKind::Ex) => write!(f, "COMMAND"),
            Mode::Command(CommandKind::SearchForward) => write!(f, "SEARCH"),
            Mode::Command(CommandKind::SearchBackward) => write!(f, "SEARCH"),
            Mode::OperatorPending(_) => write!(f, "OP-PENDING"),
            Mode::TerminalInsert => write!(f, "TERMINAL"),
            Mode::InsertNormal => write!(f, "INS-NORMAL"),
        }
    }
}

/// Visual sub-mode.
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
    SearchForward,
    SearchBackward,
}

/// Pending operator.
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
