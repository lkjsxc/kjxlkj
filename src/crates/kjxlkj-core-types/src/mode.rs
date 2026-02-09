/// Editor mode state machine variants.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Replace => write!(f, "REPLACE"),
            Self::Visual(VisualKind::Char) => write!(f, "VISUAL"),
            Self::Visual(VisualKind::Line) => write!(f, "V-LINE"),
            Self::Visual(VisualKind::Block) => write!(f, "V-BLOCK"),
            Self::Command(CommandKind::Ex) => write!(f, "COMMAND"),
            Self::Command(CommandKind::Search) => write!(f, "SEARCH"),
            Self::OperatorPending(op) => write!(f, "OP: {op:?}"),
            Self::TerminalInsert => write!(f, "TERMINAL"),
            Self::InsertNormal => write!(f, "INS-NORMAL"),
        }
    }
}

/// Visual mode sub-kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualKind {
    Char,
    Line,
    Block,
}

/// Command mode sub-kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandKind {
    /// Ex command (`:`)
    Ex,
    /// Search (`/` or `?`)
    Search,
}

/// Operator awaiting a motion or text-object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Delete,
    Change,
    Yank,
    Indent,
    Dedent,
    Reindent,
    Format,
}
