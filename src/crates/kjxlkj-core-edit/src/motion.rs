//! Motion definitions.

/// A motion kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionKind {
    /// Move left by characters.
    Left,
    /// Move right by characters.
    Right,
    /// Move down by lines.
    Down,
    /// Move up by lines.
    Up,
    /// Move to start of line.
    LineStart,
    /// Move to first non-blank.
    FirstNonBlank,
    /// Move to end of line.
    LineEnd,
    /// Move to next word start.
    WordStart,
    /// Move to previous word start.
    WordStartBackward,
    /// Move to word end.
    WordEnd,
    /// Move to previous word end.
    WordEndBackward,
    /// Move to file start.
    FileStart,
    /// Move to file end.
    FileEnd,
    /// Go to specific line.
    GoToLine(usize),
    /// Go to percentage of file.
    GoToPercent(usize),
    /// Move to matching bracket.
    MatchingBracket,
    /// Move to next paragraph.
    ParagraphForward,
    /// Move to previous paragraph.
    ParagraphBackward,
    /// Move to next sentence.
    SentenceForward,
    /// Move to previous sentence.
    SentenceBackward,
    /// Find character forward.
    FindChar(char),
    /// Find character backward.
    FindCharBackward(char),
    /// Till character forward.
    TillChar(char),
    /// Till character backward.
    TillCharBackward(char),
    /// Repeat last find.
    RepeatFind,
    /// Repeat last find reversed.
    RepeatFindReverse,
    /// Screen top.
    ScreenTop,
    /// Screen middle.
    ScreenMiddle,
    /// Screen bottom.
    ScreenBottom,
    /// Go to column.
    GoToColumn(usize),
    /// Go to middle of line.
    LineMiddle,
    /// Last non-blank of line.
    LastNonBlank,
    /// Next unmatched paren.
    NextUnmatchedParen(char),
    /// Previous unmatched paren.
    PrevUnmatchedParen(char),
}

/// A motion with count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Motion {
    /// The motion kind.
    pub kind: MotionKind,
    /// Repeat count (default 1).
    pub count: usize,
    /// Whether the motion is inclusive.
    pub inclusive: bool,
}

impl Motion {
    /// Create a new motion.
    pub fn new(kind: MotionKind) -> Self {
        Self {
            kind,
            count: 1,
            inclusive: false,
        }
    }

    /// Set the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }

    /// Set inclusive.
    pub fn inclusive(mut self) -> Self {
        self.inclusive = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn motion_new_left() {
        let m = Motion::new(MotionKind::Left);
        assert_eq!(m.kind, MotionKind::Left);
    }

    #[test]
    fn motion_with_count() {
        let m = Motion::new(MotionKind::Right).with_count(5);
        assert_eq!(m.count, 5);
    }

    #[test]
    fn motion_count_min_one() {
        let m = Motion::new(MotionKind::Right).with_count(0);
        assert_eq!(m.count, 1);
    }

    #[test]
    fn motion_inclusive() {
        let m = Motion::new(MotionKind::Right).inclusive();
        assert!(m.inclusive);
    }

    #[test]
    fn motion_kind_up() {
        assert_eq!(MotionKind::Up, MotionKind::Up);
    }

    #[test]
    fn motion_kind_down() {
        assert_eq!(MotionKind::Down, MotionKind::Down);
    }

    #[test]
    fn motion_kind_line_start() {
        assert_eq!(MotionKind::LineStart, MotionKind::LineStart);
    }

    #[test]
    fn motion_kind_line_end() {
        assert_eq!(MotionKind::LineEnd, MotionKind::LineEnd);
    }
}
