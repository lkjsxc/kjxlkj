//! Normal mode command parsing.

use kjxlkj_core_types::{
    ids::RegisterId,
    motion::{Direction, Motion},
    operator::Operator,
    text_object::TextObject,
};

/// A parsed command from normal mode input.
#[derive(Debug, Clone)]
pub enum Command {
    /// Motion command.
    Motion {
        count: Option<usize>,
        motion: Motion,
    },
    /// Operator with implicit motion (doubled, e.g., dd).
    OperatorLine {
        count: Option<usize>,
        operator: Operator,
        register: RegisterId,
    },
    /// Operator pending motion.
    OperatorMotion {
        count: Option<usize>,
        operator: Operator,
        motion: Motion,
        register: RegisterId,
    },
    /// Operator on text object.
    OperatorTextObject {
        count: Option<usize>,
        operator: Operator,
        text_object: TextObject,
        register: RegisterId,
    },
    /// Enter insert mode.
    InsertMode(InsertVariant),
    /// Enter visual mode.
    VisualMode(VisualVariant),
    /// Enter command-line mode.
    CommandMode,
    /// Enter search mode.
    SearchMode(Direction),
    /// Replace single character.
    ReplaceChar(char),
    /// Enter replace mode.
    ReplaceMode,
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Repeat last command.
    Repeat,
    /// Set a mark.
    SetMark(char),
    /// Go to mark.
    GoToMark { mark: char, column: bool },
    /// Record macro.
    RecordMacro(Option<char>),
    /// Play macro.
    PlayMacro(char),
    /// Scroll command.
    Scroll(ScrollCommand),
    /// Fold command.
    Fold(FoldCommand),
    /// Join lines.
    JoinLines { spaces: bool },
    /// Put (paste).
    Put { register: RegisterId, before: bool },
    /// Toggle case at cursor.
    ToggleCaseChar,
    /// Increment number.
    Increment(i64),
    /// Decrement number.
    Decrement(i64),
    /// Open new line.
    OpenLine { below: bool },
    /// Quit.
    Quit { force: bool },
    /// Write.
    Write { path: Option<String> },
    /// Jump to older position in jumplist.
    JumpOlder,
    /// Jump to newer position in jumplist.
    JumpNewer,
    /// Show info about file/cursor.
    ShowInfo,
    /// Go to definition.
    GoToDefinition,
    /// Go to tag.
    GoToTag,
    /// Window command prefix.
    WindowPrefix,
    /// Incomplete command (needs more input).
    Incomplete,
    /// Invalid command.
    Invalid,
}

/// Variant for entering insert mode.
#[derive(Debug, Clone, Copy)]
pub enum InsertVariant {
    /// Before cursor (i).
    Before,
    /// After cursor (a).
    After,
    /// Start of line (I).
    StartOfLine,
    /// End of line (A).
    EndOfLine,
    /// Open line below (o).
    OpenBelow,
    /// Open line above (O).
    OpenAbove,
    /// Substitute character (s).
    Substitute,
    /// Substitute line (S).
    SubstituteLine,
    /// Change to end of line (C).
    ChangeToEnd,
}

/// Variant for visual mode.
#[derive(Debug, Clone, Copy)]
pub enum VisualVariant {
    /// Character-wise (v).
    Char,
    /// Line-wise (V).
    Line,
    /// Block-wise (<C-v>).
    Block,
}

/// Scroll command type.
#[derive(Debug, Clone, Copy)]
pub enum ScrollCommand {
    HalfPageDown,
    HalfPageUp,
    PageDown,
    PageUp,
    LineDown,
    LineUp,
    ToTop,
    ToMiddle,
    ToBottom,
    CenterCursor,
    HalfWidth,
}

/// Fold command type.
#[derive(Debug, Clone, Copy)]
pub enum FoldCommand {
    Create,
    Delete,
    Toggle,
    Open,
    Close,
    OpenAll,
    CloseAll,
}
