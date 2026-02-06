//! Motion, scroll, and find-char kind enums.

/// Motion kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionKind {
    Left, Right, Up, Down,
    WordForward, WordForwardEnd, WordBackward,
    WORDForward, WORDForwardEnd, WORDBackward,
    WordBackwardEnd, WORDBackwardEnd,
    LineStart, LineEnd, FirstNonBlank, LastNonBlank,
    FileStart, FileEnd,
    GotoLine(usize), GotoColumn(usize), GotoPercent(usize),
    ScreenTop, ScreenMiddle, ScreenBottom,
    NextParagraph, PrevParagraph, NextSentence, PrevSentence,
    MatchingBracket,
    FindCharForward(char), FindCharBackward(char),
    TillCharForward(char), TillCharBackward(char),
    NextNonBlankLine, PrevNonBlankLine,
    MiddleOfLine, LineStartNonBlankDown, LineStartNonBlankUp,
}

/// Scroll kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollKind {
    HalfPageDown, HalfPageUp, FullPageDown, FullPageUp,
    LineDown, LineUp,
    CursorCenter, CursorTop, CursorBottom,
    CursorCenterFirstNonBlank, CursorTopFirstNonBlank,
    CursorBottomFirstNonBlank,
}

/// Find char direction and inclusive/exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindCharKind {
    Forward, Backward, TillForward, TillBackward,
}
