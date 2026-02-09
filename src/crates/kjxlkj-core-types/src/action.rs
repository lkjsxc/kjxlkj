use crate::{BufferId, CursorPosition};

/// Actions that modify editor state, dispatched from input
/// or services to the core task.
#[derive(Debug, Clone)]
pub enum Action {
    // -- Terminal / lifecycle --
    Resize(u16, u16),
    Paste(String),
    FocusGained,
    FocusLost,
    Quit,
    ForceQuit,
    WriteQuit,

    // -- Cursor movement --
    MoveUp(usize),
    MoveDown(usize),
    MoveLeft(usize),
    MoveRight(usize),
    MoveToLineStart,
    MoveToFirstNonBlank,
    MoveToLineEnd,
    MoveWordForward(usize),
    MoveWordBackward(usize),
    MoveWordEndForward(usize),
    MoveToTop,
    MoveToBottom,
    MoveToLine(usize),
    MoveToMatchingBracket,
    FindCharForward(char),
    FindCharBackward(char),
    TillCharForward(char),
    TillCharBackward(char),
    RepeatFindChar,
    RepeatFindCharReverse,
    PageUp,
    PageDown,
    HalfPageUp,
    HalfPageDown,

    // -- Insert mode entry --
    InsertBefore,
    InsertAfter,
    InsertAtLineStart,
    InsertAtLineEnd,
    OpenBelow,
    OpenAbove,

    // -- Text editing in insert mode --
    InsertChar(char),
    InsertNewline,
    DeleteCharBackward,
    DeleteCharForward,
    DeleteWordBackward,

    // -- Normal mode operations --
    DeleteChar(usize),
    DeleteCharBack(usize),
    ReplaceChar(char),
    JoinLines,
    JoinLinesNoSpace,
    ToggleCase,
    Undo,
    Redo,
    PutAfter,
    PutBefore,
    DotRepeat,

    // -- Operators (linewise doubled) --
    DeleteLine(usize),
    YankLine(usize),
    ChangeLine,
    IndentLine(usize),
    DedentLine(usize),
    ReindentLine,

    // -- Operators with motions (after resolved) --
    DeleteRange {
        start: CursorPosition,
        end: CursorPosition,
        linewise: bool,
    },
    YankRange {
        start: CursorPosition,
        end: CursorPosition,
        linewise: bool,
    },
    ChangeRange {
        start: CursorPosition,
        end: CursorPosition,
        linewise: bool,
    },
    IndentRange {
        start_line: usize,
        end_line: usize,
    },
    DedentRange {
        start_line: usize,
        end_line: usize,
    },

    // -- Mode changes --
    EnterNormal,
    EnterInsertNormal,
    ReturnFromInsertNormal,
    EnterVisualChar,
    EnterVisualLine,
    EnterVisualBlock,
    EnterCommandEx,
    EnterSearchForward,
    EnterSearchBackward,
    EnterReplace,
    EnterTerminalInsert,
    ExitTerminalInsert,
    EnterOperatorPending(crate::Operator),

    // -- Command line --
    CmdlineInsertChar(char),
    CmdlineBackspace,
    CmdlineExecute,
    CmdlineCancel,

    // -- Visual mode --
    VisualSelectAll,
    VisualSwitchKind(crate::VisualKind),

    // -- Buffer management --
    OpenFile(String),
    WriteBuffer,
    WriteBufferAs(String),
    NextBuffer,
    PrevBuffer,
    SwitchBuffer(BufferId),
    DeleteBuffer(BufferId),

    // -- Window management --
    SplitHorizontal,
    SplitVertical,
    CloseWindow,
    FocusWindowLeft,
    FocusWindowDown,
    FocusWindowUp,
    FocusWindowRight,
    FocusNextWindow,
    FocusPrevWindow,

    // -- Viewport scrolling --
    ScrollCenterCursor,
    ScrollCursorTop,
    ScrollCursorBottom,

    // -- Search --
    SearchNext,
    SearchPrev,

    // -- Marks --
    SetMark(char),
    JumpToMark(char),
    JumpToMarkLine(char),
    ChangelistOlder,
    ChangelistNewer,
    JumpOlder,
    JumpNewer,

    // -- Macros --
    StartRecording(char),
    StopRecording,
    PlayMacro(char, usize),

    // -- Register --
    SelectRegister(char),

    // -- Noop --
    Noop,
}
