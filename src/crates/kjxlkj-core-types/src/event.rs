//! Events flowing through the editor system.

use crate::{BufferId, Mode, RegisterName, Size};
use serde::{Deserialize, Serialize};

/// A key event from the terminal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyEvent {
    pub fn char(c: char) -> Self {
        Self {
            code: KeyCode::Char(c),
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    pub fn ctrl(c: char) -> Self {
        Self {
            code: KeyCode::Char(c),
            ctrl: true,
            alt: false,
            shift: false,
        }
    }

    pub fn special(code: KeyCode) -> Self {
        Self {
            code,
            ctrl: false,
            alt: false,
            shift: false,
        }
    }
}

/// Key identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char),
    Escape,
    Enter,
    Backspace,
    Tab,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    F(u8),
}

/// High-level editor event (terminal input decoded to intent).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorEvent {
    Key(KeyEvent),
    Resize(Size),
    Quit,
}

/// An intent emitted by mode interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Intent {
    /// Insert a character at cursor position.
    InsertChar(char),
    /// Insert a newline.
    InsertNewline,
    /// Delete character before cursor (backspace).
    DeleteCharBefore,
    /// Delete character at cursor.
    DeleteCharAt,
    /// Delete word before cursor (Ctrl-w in insert mode).
    DeleteWordBefore,
    /// Delete to line start (Ctrl-u in insert mode).
    DeleteToLineStart,
    /// Insert contents of a register at cursor (Ctrl-r {reg}).
    InsertFromRegister(char),
    /// Execute a motion.
    Motion(MotionKind, usize),
    /// Execute an operator over a motion range.
    Operator(OperatorKind, MotionKind, usize),
    /// Execute an operator over a text object.
    OperatorTextObject(OperatorKind, TextObjectKind, bool),
    /// Line-wise operator (dd, yy, cc, etc.).
    LineOperator(OperatorKind, usize),
    /// Enter a mode.
    EnterMode(Mode),
    /// Enter insert mode at a specific position.
    EnterInsert(InsertPosition),
    /// Execute an ex command.
    ExCommand(String),
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Repeat last change.
    RepeatLastChange,
    /// Paste from register.
    Paste(RegisterName, PastePosition),
    /// Yank current line.
    YankLine(usize),
    /// Replace single character.
    ReplaceChar(char),
    /// Replace mode character entry.
    ReplaceInsert(char),
    /// Search forward.
    SearchForward(String),
    /// Search backward.
    SearchBackward(String),
    /// Repeat search same direction.
    SearchNext,
    /// Repeat search opposite direction.
    SearchPrev,
    /// Search word under cursor forward.
    SearchWordForward,
    /// Search word under cursor backward.
    SearchWordBackward,
    /// Set mark.
    SetMark(char),
    /// Jump to mark (exact).
    JumpToMark(char),
    /// Jump to mark (line).
    JumpToMarkLine(char),
    /// Macro record start/stop.
    MacroToggleRecord(char),
    /// Macro playback.
    MacroPlay(char),
    /// Macro repeat last.
    MacroRepeatLast,
    /// Select register for next operation.
    SelectRegister(RegisterName),
    /// Join lines.
    JoinLines(bool, usize),
    /// Toggle case at cursor.
    ToggleCase,
    /// Case operator over motion.
    CaseOperator(CaseOp, MotionKind, usize),
    /// Case operator on line.
    CaseOperatorLine(CaseOp),
    /// Indent/outdent.
    Indent(bool, usize),
    /// Scroll commands.
    Scroll(ScrollKind),
    /// Visual mode swap cursor.
    VisualSwapEnd,
    /// Increment/decrement number.
    IncrementNumber(i64),
    /// Find character on line.
    FindChar(char, FindCharKind),
    /// Repeat find char.
    RepeatFindChar,
    /// Repeat find char (reverse).
    RepeatFindCharReverse,
    /// Jump list navigation.
    JumpListBack,
    JumpListForward,
    /// Change list navigation.
    ChangeListOlder,
    ChangeListNewer,
    /// Open line below/above.
    OpenLine(bool),
    /// Delete to end of line.
    DeleteToEnd,
    /// Change to end of line.
    ChangeToEnd,
    /// Substitute char.
    SubstituteChar,
    /// Substitute line.
    SubstituteLine,
    /// Window split horizontal.
    WindowSplitHorizontal,
    /// Window split vertical.
    WindowSplitVertical,
    /// Window close.
    WindowClose,
    /// Window only (close all others).
    WindowOnly,
    /// Window focus next.
    WindowFocusNext,
    /// Window focus previous.
    WindowFocusPrev,
    /// Window focus direction (h/j/k/l).
    WindowFocusDirection(MotionKind),
    /// Window equal size.
    WindowEqualSize,
    /// Window rotate.
    WindowRotate,
    /// Enter command-line mode with prefix (: / ?).
    EnterCommandLine(char),
    /// Nothing (no-op).
    Noop,
}

/// Insert position relative to cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    /// `i` — before cursor.
    BeforeCursor,
    /// `a` — after cursor.
    AfterCursor,
    /// `I` — first non-blank of line.
    FirstNonBlank,
    /// `A` — end of line.
    EndOfLine,
}

/// Operator kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
    Format,
    ToggleCase,
    Uppercase,
    Lowercase,
}

/// Motion kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionKind {
    Left,
    Right,
    Up,
    Down,
    WordForward,
    WordForwardEnd,
    WordBackward,
    WORDForward,
    WORDForwardEnd,
    WORDBackward,
    WordBackwardEnd,
    WORDBackwardEnd,
    LineStart,
    LineEnd,
    FirstNonBlank,
    LastNonBlank,
    FileStart,
    FileEnd,
    GotoLine(usize),
    GotoColumn(usize),
    GotoPercent(usize),
    ScreenTop,
    ScreenMiddle,
    ScreenBottom,
    NextParagraph,
    PrevParagraph,
    NextSentence,
    PrevSentence,
    MatchingBracket,
    FindCharForward(char),
    FindCharBackward(char),
    TillCharForward(char),
    TillCharBackward(char),
    NextNonBlankLine,
    PrevNonBlankLine,
    MiddleOfLine,
    LineStartNonBlankDown,
    LineStartNonBlankUp,
}

/// Text object kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectKind {
    Word,
    WORD,
    Sentence,
    Paragraph,
    DoubleQuote,
    SingleQuote,
    BackTick,
    Paren,
    Bracket,
    Brace,
    AngleBracket,
    Tag,
}

/// Paste position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PastePosition {
    After,
    Before,
    AfterCursorEnd,
    BeforeCursorEnd,
}

/// Case operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseOp {
    Toggle,
    Upper,
    Lower,
}

/// Scroll kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollKind {
    HalfPageDown,
    HalfPageUp,
    FullPageDown,
    FullPageUp,
    LineDown,
    LineUp,
    CursorCenter,
    CursorTop,
    CursorBottom,
    CursorCenterFirstNonBlank,
    CursorTopFirstNonBlank,
    CursorBottomFirstNonBlank,
}

/// Find char direction and inclusive/exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindCharKind {
    /// `f` inclusive forward.
    Forward,
    /// `F` inclusive backward.
    Backward,
    /// `t` exclusive forward (till).
    TillForward,
    /// `T` exclusive backward (till).
    TillBackward,
}

/// Service message envelope.
#[derive(Debug, Clone)]
pub enum ServiceMessage {
    FileChanged(std::path::PathBuf),
    DiagnosticsUpdate(BufferId, Vec<Diagnostic>),
    Notification(String),
}

/// A diagnostic entry from LSP or similar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: crate::Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
}

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}
