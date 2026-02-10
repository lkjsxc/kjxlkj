//! Editor mode types.

/// Visual mode sub-type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualKind {
    /// Character-wise selection.
    Char,
    /// Line-wise selection.
    Line,
    /// Block selection.
    Block,
}

/// Command mode sub-type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandKind {
    /// Ex command mode (`:` prefix).
    Ex,
    /// Forward search mode (`/` prefix).
    SearchForward,
    /// Backward search mode (`?` prefix).
    SearchBackward,
}

/// Operator waiting for motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingOperator {
    /// Delete operator.
    Delete,
    /// Change operator.
    Change,
    /// Yank operator.
    Yank,
    /// Indent right.
    IndentRight,
    /// Indent left.
    IndentLeft,
    /// Auto-format.
    Format,
    /// Toggle case.
    ToggleCase,
    /// Lowercase.
    Lowercase,
    /// Uppercase.
    Uppercase,
}

/// Editor mode state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// Normal mode for navigation and commands.
    Normal,
    /// Insert mode for text entry.
    Insert,
    /// Visual mode with selection kind.
    Visual(VisualKind),
    /// Command-line mode.
    Command(CommandKind),
    /// Replace mode for overwriting.
    Replace,
    /// Operator-pending mode awaiting motion.
    OperatorPending(PendingOperator),
    /// Insert-normal mode (single normal command via Ctrl-O).
    InsertNormal,
    /// Terminal insert mode (forwarding to PTY).
    TerminalInsert,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

impl Mode {
    /// Check if this is an insert-like mode.
    pub fn is_insert(&self) -> bool {
        matches!(self, Mode::Insert | Mode::InsertNormal)
    }

    /// Check if this is a visual mode.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual(_))
    }

    /// Check if this is command mode.
    pub fn is_command(&self) -> bool {
        matches!(self, Mode::Command(_))
    }

    /// Get the mode name for display.
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual(VisualKind::Char) => "VISUAL",
            Mode::Visual(VisualKind::Line) => "V-LINE",
            Mode::Visual(VisualKind::Block) => "V-BLOCK",
            Mode::Command(CommandKind::Ex) => "COMMAND",
            Mode::Command(CommandKind::SearchForward) => "SEARCH",
            Mode::Command(CommandKind::SearchBackward) => "SEARCH",
            Mode::Replace => "REPLACE",
            Mode::OperatorPending(_) => "OP-PENDING",
            Mode::InsertNormal => "INSERT",
            Mode::TerminalInsert => "TERMINAL",
        }
    }
}
