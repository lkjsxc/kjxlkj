//! Typed actions emitted by the input pipeline.

use crate::{BufferId, Key, KeyModifiers, Mode, Operator};
use serde::{Deserialize, Serialize};

/// Typed editor action emitted after input decode and mapping resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    InsertChar(char), DeleteCharForward, DeleteCharBackward,
    DeleteLine, YankLine, Motion(Motion),
    EnterMode(Mode), ExitToNormal,
    ExCommand(String), Resize(u16, u16), Paste(String),
    FocusGained, FocusLost, Quit, ForceQuit, Write, WriteQuit,
    // Window management
    SplitHorizontal, SplitVertical, CloseWindow,
    FocusDirection(Direction), FocusCycle, FocusPrevious,
    WindowOnly, FocusTopLeft, FocusBottomRight,
    WindowEqualize, WindowResize(ResizeEdge, i16),
    WindowMaxHeight, WindowMaxWidth,
    OpenExplorer, CloseExplorer, OpenTerminal,
    Undo, Redo, DotRepeat, Noop,
    ForwardKey(Key, KeyModifiers),
    // Buffer management
    OpenFile(String), SwitchBuffer(BufferId),
    NextBuffer, PreviousBuffer, DeleteBuffer,
    SwitchAlternate, ListBuffers, FirstBuffer, LastBuffer,
    // Editing actions
    AppendEndOfLine, InsertFirstNonBlank,
    OpenLineBelow, OpenLineAbove, JoinLines,
    ToggleCase, ReplaceChar(char),
    PutAfter, PutBefore,
    ScrollCenter, ScrollTop, ScrollBottom,
    DeleteWordBackward, DeleteToLineStart,
    OperatorLine(Operator), OperatorMotion(Operator, Motion, usize),
    SubstituteChar, SubstituteLine, ChangeToEnd, DeleteToEnd,
    YankCurrentLine, JoinLinesNoSpace, ShowRegisters,
    // Search
    StarSearchForward, StarSearchBackward,
    ClearSearchHighlight, GStarSearchForward, GStarSearchBackward,
    IncrementNumber, DecrementNumber,
    SetOption(String, String),
    // Visual
    VisualOperator(Operator), VisualSwapAnchor,
}

/// Cursor motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Motion {
    // Character motions
    Left,
    Right,
    Up,
    Down,
    // Line motions
    LineStart,
    LineEnd,
    FirstNonBlank,
    LastNonBlank,
    // Word/WORD motions
    WordForward,
    WordBackward,
    WordEndForward,
    WordEndBackward,
    BigWordForward,
    BigWordBackward,
    BigWordEndForward,
    BigWordEndBackward,
    // Sentence/paragraph motions
    SentenceForward,
    SentenceBackward,
    ParagraphForward,
    ParagraphBackward,
    // Find/till motions
    FindForward(char),
    FindBackward(char),
    TillForward(char),
    TillBackward(char),
    RepeatFind,
    RepeatFindReverse,
    // Search repeat motions
    SearchNext,
    SearchPrev,
    // Document motions
    GotoLine(usize),
    GotoFirstLine,
    GotoLastLine,
    // Window motions
    WindowTop,
    WindowMiddle,
    WindowBottom,
    MatchParen,
    PageDown, PageUp, HalfPageDown, HalfPageUp, ScrollDown, ScrollUp,
    TextObjInner(char), TextObjAround(char),
}

/// Directional focus for window navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction { Left, Right, Up, Down }

/// Edge for window resize operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeEdge { Height, Width }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_quit_is_distinct() {
        assert_ne!(Action::Quit, Action::ForceQuit);
    }
}
