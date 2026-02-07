//! Motion types covering all cursor movement motions.

use serde::{Deserialize, Serialize};

use crate::types::Direction;

/// All supported cursor motions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    PrevWordEnd,
    PrevBigWordEnd,

    // Line position motions
    LineStart,
    FirstNonBlank,
    LineEnd,
    LastNonBlank,
    LineMiddle,

    // File position motions
    FileStart,
    FileEnd,
    GoToLine(usize),
    GoToColumn(usize),
    GoToPercent(usize),

    // Screen position motions
    ScreenTop,
    ScreenMiddle,
    ScreenBottom,

    // Find/till motions
    FindChar(char, Direction, bool),
    RepeatFind,
    RepeatFindReverse,

    // Matching bracket
    MatchingBracket,

    // Block motions
    ParagraphForward,
    ParagraphBackward,
    SentenceForward,
    SentenceBackward,

    // Unmatched delimiter motions
    UnmatchedParen(Direction),
    UnmatchedBrace(Direction),
}

impl Motion {
    /// Returns `true` for motions that operate across lines.
    pub fn is_linewise(&self) -> bool {
        matches!(
            self,
            Self::Up
                | Self::Down
                | Self::FileStart
                | Self::FileEnd
                | Self::GoToLine(_)
                | Self::GoToPercent(_)
                | Self::ScreenTop
                | Self::ScreenMiddle
                | Self::ScreenBottom
                | Self::ParagraphForward
                | Self::ParagraphBackward
        )
    }

    /// Returns `true` for motions that are inclusive.
    pub fn is_inclusive(&self) -> bool {
        matches!(
            self,
            Self::WordEnd
                | Self::BigWordEnd
                | Self::LineEnd
                | Self::LastNonBlank
                | Self::FindChar(_, Direction::Forward, false)
                | Self::MatchingBracket
                | Self::GoToColumn(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linewise_motions() {
        assert!(Motion::Up.is_linewise());
        assert!(!Motion::Left.is_linewise());
    }

    #[test]
    fn inclusive_motions() {
        assert!(Motion::WordEnd.is_inclusive());
        assert!(!Motion::WordForward.is_inclusive());
    }
}
