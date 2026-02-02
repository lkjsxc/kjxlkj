//! Motion types for cursor movement.

use serde::{Deserialize, Serialize};

/// Direction of movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Forward,
    Backward,
}

/// A motion that moves the cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Motion {
    // Character motions
    Left,
    Right,
    Up,
    Down,

    // Word motions
    WordForward,
    WordBackward,
    WordEnd,
    BigWordForward,
    BigWordBackward,
    BigWordEnd,

    // Line motions
    LineStart,
    FirstNonBlank,
    LineEnd,
    FirstColumn,
    LastColumn,

    // Paragraph motions
    ParagraphForward,
    ParagraphBackward,

    // Sentence motions
    SentenceForward,
    SentenceBackward,

    // Screen motions
    ScreenTop,
    ScreenMiddle,
    ScreenBottom,

    // Document motions
    DocumentStart,
    DocumentEnd,
    GoToLine(usize),
    GoToColumn(usize),

    // Search motions
    FindChar {
        char: char,
        direction: Direction,
        inclusive: bool,
    },
    TillChar {
        char: char,
        direction: Direction,
    },
    RepeatFindChar,
    RepeatFindCharReverse,
    NextSearchResult,
    PrevSearchResult,
    SearchWordUnderCursor {
        direction: Direction,
    },

    // Match motions
    MatchingBracket,

    // Mark motions
    GoToMark {
        mark: char,
        column: bool,
    },

    // Scroll motions
    HalfPageDown,
    HalfPageUp,
    PageDown,
    PageUp,
    ScrollLineDown,
    ScrollLineUp,
}

impl Motion {
    /// Whether this motion is linewise.
    pub fn is_linewise(&self) -> bool {
        matches!(
            self,
            Motion::Up
                | Motion::Down
                | Motion::ParagraphForward
                | Motion::ParagraphBackward
                | Motion::DocumentStart
                | Motion::DocumentEnd
                | Motion::GoToLine(_)
                | Motion::ScreenTop
                | Motion::ScreenMiddle
                | Motion::ScreenBottom
        )
    }

    /// Whether this motion is inclusive.
    pub fn is_inclusive(&self) -> bool {
        matches!(
            self,
            Motion::WordEnd
                | Motion::BigWordEnd
                | Motion::LineEnd
                | Motion::FindChar {
                    inclusive: true,
                    ..
                }
                | Motion::MatchingBracket
        )
    }

    /// Whether this motion is exclusive.
    pub fn is_exclusive(&self) -> bool {
        !self.is_inclusive() && !self.is_linewise()
    }
}

/// Result of executing a motion.
#[derive(Debug, Clone, Copy)]
pub struct MotionResult {
    /// Line number (0-based).
    pub line: usize,
    /// Column offset (0-based).
    pub column: usize,
    /// Whether the motion wrapped to a new line.
    pub wrapped: bool,
    /// Whether the motion hit a boundary.
    pub hit_boundary: bool,
}
