//! Intent and related types emitted by mode interpretation.

use crate::{Mode, RegisterName};
use super::motion_kind::{FindCharKind, MotionKind, ScrollKind};

/// An intent emitted by mode interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Intent {
    InsertChar(char),
    InsertNewline,
    DeleteCharBefore,
    DeleteCharAt,
    DeleteWordBefore,
    DeleteToLineStart,
    InsertFromRegister(char),
    Motion(MotionKind, usize),
    Operator(OperatorKind, MotionKind, usize),
    OperatorTextObject(OperatorKind, TextObjectKind, bool),
    LineOperator(OperatorKind, usize),
    EnterMode(Mode),
    EnterInsert(InsertPosition),
    ExCommand(String),
    Undo,
    Redo,
    RepeatLastChange,
    Paste(RegisterName, PastePosition),
    YankLine(usize),
    ReplaceChar(char),
    ReplaceInsert(char),
    SearchForward(String),
    SearchBackward(String),
    SearchNext,
    SearchPrev,
    SearchWordForward,
    SearchWordBackward,
    SetMark(char),
    JumpToMark(char),
    JumpToMarkLine(char),
    MacroToggleRecord(char),
    MacroPlay(char),
    MacroRepeatLast,
    SelectRegister(RegisterName),
    JoinLines(bool, usize),
    ToggleCase,
    CaseOperator(CaseOp, MotionKind, usize),
    CaseOperatorLine(CaseOp),
    Indent(bool, usize),
    Scroll(ScrollKind),
    VisualSwapEnd,
    IncrementNumber(i64),
    FindChar(char, FindCharKind),
    RepeatFindChar,
    RepeatFindCharReverse,
    JumpListBack,
    JumpListForward,
    ChangeListOlder,
    ChangeListNewer,
    OpenLine(bool),
    DeleteToEnd,
    ChangeToEnd,
    SubstituteChar,
    SubstituteLine,
    WindowSplitHorizontal,
    WindowSplitVertical,
    WindowClose,
    WindowOnly,
    WindowFocusNext,
    WindowFocusPrev,
    WindowFocusDirection(MotionKind),
    WindowEqualSize,
    WindowRotate,
    EnterCommandLine(char),
    Noop,
}

/// Insert position relative to cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    BeforeCursor,
    AfterCursor,
    FirstNonBlank,
    EndOfLine,
}

/// Operator kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    Delete, Yank, Change, Indent, Outdent, Format,
    ToggleCase, Uppercase, Lowercase,
}

/// Text object kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectKind {
    Word, WORD, Sentence, Paragraph,
    DoubleQuote, SingleQuote, BackTick,
    Paren, Bracket, Brace, AngleBracket, Tag,
}

/// Paste position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PastePosition {
    After, Before, AfterCursorEnd, BeforeCursorEnd,
}

/// Case operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseOp {
    Toggle, Upper, Lower,
}
