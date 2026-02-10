//! Motion types and execution.

use kjxlkj_core_types::CursorPosition;

/// Motion direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Forward in document.
    Forward,
    /// Backward in document.
    Backward,
}

/// Motion type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    /// Move left by characters.
    Left,
    /// Move right by characters.
    Right,
    /// Move up by lines.
    Up,
    /// Move down by lines.
    Down,
    /// Move to start of line.
    LineStart,
    /// Move to first non-blank of line.
    FirstNonBlank,
    /// Move to end of line.
    LineEnd,
    /// Move to start of word.
    WordStart(Direction),
    /// Move to end of word.
    WordEnd(Direction),
    /// Move to start of WORD.
    BigWordStart(Direction),
    /// Move to end of WORD.
    BigWordEnd(Direction),
    /// Move to start of document.
    DocumentStart,
    /// Move to end of document.
    DocumentEnd,
    /// Move to specific line (1-indexed).
    Line(usize),
    /// Move to last line.
    LastLine,
    /// Move to specific line (legacy alias).
    GoToLine(usize),
    /// Move to matching bracket.
    MatchingBracket,
    /// Move to next search match.
    SearchNext,
    /// Move to previous search match.
    SearchPrev,
    /// Move to character on line.
    FindChar { c: char, direction: Direction, till: bool },
    /// Repeat last find.
    RepeatFind,
    /// Repeat last find reversed.
    RepeatFindReverse,
    /// Move by paragraph.
    Paragraph(Direction),
    /// Move by sentence.
    Sentence(Direction),
}

/// Result of applying a motion.
#[derive(Debug, Clone, Copy)]
pub struct MotionResult {
    /// New cursor position.
    pub position: CursorPosition,
    /// Whether the motion is line-wise.
    pub linewise: bool,
    /// Whether the motion is inclusive.
    pub inclusive: bool,
}

impl MotionResult {
    /// Create a character-wise motion result.
    pub fn charwise(position: CursorPosition, inclusive: bool) -> Self {
        Self {
            position,
            linewise: false,
            inclusive,
        }
    }

    /// Create a line-wise motion result.
    pub fn linewise(position: CursorPosition) -> Self {
        Self {
            position,
            linewise: true,
            inclusive: true,
        }
    }
}

/// Check if a character is a word character.
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Check if a character is a WORD character (non-whitespace).
pub fn is_big_word_char(c: char) -> bool {
    !c.is_whitespace()
}

/// Check if a character is a bracket.
pub fn is_bracket(c: char) -> bool {
    matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>')
}

/// Get the matching bracket for a character.
pub fn matching_bracket(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        ')' => Some('('),
        '[' => Some(']'),
        ']' => Some('['),
        '{' => Some('}'),
        '}' => Some('{'),
        '<' => Some('>'),
        '>' => Some('<'),
        _ => None,
    }
}
