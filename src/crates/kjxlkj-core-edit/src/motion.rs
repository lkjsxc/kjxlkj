//! Motion types for cursor movement.

use kjxlkj_core_types::Cursor;

/// A motion describes how the cursor moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionKind {
    /// Move left by count characters.
    Left,
    /// Move right by count characters.
    Right,
    /// Move up by count lines.
    Up,
    /// Move down by count lines.
    Down,
    /// Move to start of line (column 0).
    LineStart,
    /// Move to first non-blank character.
    FirstNonBlank,
    /// Move to end of line.
    LineEnd,
    /// Move to next word start.
    WordStart,
    /// Move to next WORD start.
    WordStartBig,
    /// Move to word end.
    WordEnd,
    /// Move to WORD end.
    WordEndBig,
    /// Move to previous word start.
    WordBack,
    /// Move to previous WORD start.
    WordBackBig,
    /// Move to previous word end.
    WordEndBack,
    /// Move to previous WORD end.
    WordEndBackBig,
    /// Move to file start.
    FileStart,
    /// Move to file end.
    FileEnd,
    /// Move to specific line.
    GotoLine(usize),
    /// Move to percentage of file.
    GotoPercent(usize),
    /// Find character forward on line.
    FindChar(char),
    /// Find character backward on line.
    FindCharBack(char),
    /// Till character forward on line.
    TillChar(char),
    /// Till character backward on line.
    TillCharBack(char),
    /// Repeat last find/till.
    RepeatFind,
    /// Repeat last find/till in opposite direction.
    RepeatFindReverse,
    /// Move to matching bracket.
    MatchingBracket,
    /// Move to next sentence.
    SentenceForward,
    /// Move to previous sentence.
    SentenceBack,
    /// Move to next paragraph.
    ParagraphForward,
    /// Move to previous paragraph.
    ParagraphBack,
    /// Scroll to top of screen.
    ScreenTop,
    /// Scroll to middle of screen.
    ScreenMiddle,
    /// Scroll to bottom of screen.
    ScreenBottom,
    /// Go to column N.
    GotoColumn(usize),
    /// First non-blank of next line.
    NextLineFirstNonBlank,
    /// First non-blank of previous line.
    PrevLineFirstNonBlank,
    /// Last non-blank of line.
    LastNonBlank,
    /// Middle of line.
    LineMiddle,
}

/// A complete motion with count and kind.
#[derive(Debug, Clone, Copy)]
pub struct Motion {
    pub kind: MotionKind,
    pub count: usize,
}

impl Motion {
    /// Create a new motion with count 1.
    pub fn new(kind: MotionKind) -> Self {
        Self { kind, count: 1 }
    }

    /// Create a motion with a specific count.
    pub fn with_count(kind: MotionKind, count: usize) -> Self {
        Self { kind, count }
    }
}

/// Result of applying a motion.
#[derive(Debug, Clone, Copy)]
pub struct MotionResult {
    /// New cursor position.
    pub cursor: Cursor,
    /// Whether this is a linewise motion.
    pub linewise: bool,
    /// Whether the motion was inclusive.
    pub inclusive: bool,
}

impl MotionResult {
    /// Create a charwise, exclusive motion result.
    pub fn charwise(cursor: Cursor) -> Self {
        Self {
            cursor,
            linewise: false,
            inclusive: false,
        }
    }

    /// Create a charwise, inclusive motion result.
    pub fn inclusive(cursor: Cursor) -> Self {
        Self {
            cursor,
            linewise: false,
            inclusive: true,
        }
    }

    /// Create a linewise motion result.
    pub fn linewise(cursor: Cursor) -> Self {
        Self {
            cursor,
            linewise: true,
            inclusive: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_new() {
        let m = Motion::new(MotionKind::Left);
        assert_eq!(m.count, 1);
    }

    #[test]
    fn test_motion_with_count() {
        let m = Motion::with_count(MotionKind::Down, 5);
        assert_eq!(m.count, 5);
    }
}
