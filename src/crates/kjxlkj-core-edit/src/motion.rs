//! Motion definitions.

use kjxlkj_core_types::LineCol;

/// Motion kind for cursor movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionKind {
    /// Character-wise motion.
    Char,
    /// Word motion.
    Word,
    /// WORD motion (whitespace-delimited).
    BigWord,
    /// Line motion.
    Line,
    /// Paragraph motion.
    Paragraph,
    /// Search motion.
    Search,
    /// Mark motion.
    Mark,
}

/// Motion with direction and count.
#[derive(Debug, Clone)]
pub struct Motion {
    /// Motion kind.
    pub kind: MotionKind,
    /// Direction (true = forward, false = backward).
    pub forward: bool,
    /// Repeat count.
    pub count: usize,
    /// Whether this is an inclusive motion.
    pub inclusive: bool,
}

impl Motion {
    /// Create a character motion.
    pub fn char_motion(forward: bool, count: usize) -> Self {
        Self {
            kind: MotionKind::Char,
            forward,
            count,
            inclusive: false,
        }
    }

    /// Create a word motion.
    pub fn word_motion(forward: bool, count: usize) -> Self {
        Self {
            kind: MotionKind::Word,
            forward,
            count,
            inclusive: false,
        }
    }

    /// Create a line motion.
    pub fn line_motion(forward: bool, count: usize) -> Self {
        Self {
            kind: MotionKind::Line,
            forward,
            count,
            inclusive: true,
        }
    }

    /// Make this motion inclusive.
    pub fn inclusive(mut self) -> Self {
        self.inclusive = true;
        self
    }
}

/// Motion target (result of applying a motion).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MotionTarget {
    /// Start position.
    pub start: LineCol,
    /// End position.
    pub end: LineCol,
    /// Whether the range is line-wise.
    pub linewise: bool,
}

#[allow(dead_code)]
impl MotionTarget {
    /// Create a new motion target.
    pub fn new(start: LineCol, end: LineCol, linewise: bool) -> Self {
        Self {
            start,
            end,
            linewise,
        }
    }

    /// Create a character-wise target.
    pub fn charwise(start: LineCol, end: LineCol) -> Self {
        Self::new(start, end, false)
    }

    /// Create a line-wise target.
    pub fn linewise(start: LineCol, end: LineCol) -> Self {
        Self::new(start, end, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_motion_defaults() {
        let m = Motion::char_motion(true, 1);
        assert!(!m.inclusive);
        assert_eq!(m.kind, MotionKind::Char);
    }

    #[test]
    fn line_motion_is_inclusive() {
        let m = Motion::line_motion(true, 1);
        assert!(m.inclusive);
    }
}
