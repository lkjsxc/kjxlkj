//! Motion types.

use serde::{Deserialize, Serialize};

/// Kind of motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionKind {
    // Character motions
    Left,
    Right,
    Up,
    Down,

    // Word motions
    WordStart,
    WordEnd,
    WordStartBig,
    WordEndBig,
    WordBack,
    WordBackBig,

    // Line motions
    LineStart,
    LineEnd,
    FirstNonBlank,
    LastNonBlank,

    // Paragraph motions
    ParagraphBack,
    ParagraphForward,

    // Buffer motions
    BufferStart,
    BufferEnd,
    GotoLine,

    // Search motions
    FindChar,
    FindCharBack,
    TillChar,
    TillCharBack,
    RepeatFind,
    RepeatFindReverse,

    // Match motions
    MatchingPair,
}

/// A motion with optional count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Motion {
    /// Kind of motion.
    pub kind: MotionKind,
    /// Repeat count.
    pub count: usize,
    /// Character argument (for find/till).
    pub char_arg: Option<char>,
    /// Whether the motion is inclusive.
    pub inclusive: bool,
}

impl Motion {
    /// Creates a new motion.
    pub fn new(kind: MotionKind) -> Self {
        Self {
            kind,
            count: 1,
            char_arg: None,
            inclusive: kind.is_inclusive(),
        }
    }

    /// Sets the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }

    /// Sets the character argument.
    pub fn with_char(mut self, c: char) -> Self {
        self.char_arg = Some(c);
        self
    }
}

impl MotionKind {
    /// Returns true if this motion is inclusive by default.
    pub fn is_inclusive(&self) -> bool {
        matches!(
            self,
            MotionKind::Right
                | MotionKind::WordEnd
                | MotionKind::WordEndBig
                | MotionKind::FindChar
                | MotionKind::FindCharBack
                | MotionKind::MatchingPair
        )
    }

    /// Returns true if this motion is line-wise.
    pub fn is_linewise(&self) -> bool {
        matches!(
            self,
            MotionKind::Up
                | MotionKind::Down
                | MotionKind::ParagraphBack
                | MotionKind::ParagraphForward
                | MotionKind::GotoLine
        )
    }
}
