//! Mark types.

use kjxlkj_core_types::{BufferId, Position};
use serde::{Deserialize, Serialize};

/// A mark's position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mark {
    /// Buffer the mark is in.
    pub buffer: BufferId,
    /// Position in the buffer.
    pub position: Position,
}

impl Mark {
    /// Creates a new mark.
    pub fn new(buffer: BufferId, position: Position) -> Self {
        Self { buffer, position }
    }
}

/// Mark type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkType {
    /// Local mark (a-z) - per buffer.
    Local,
    /// Global mark (A-Z) - across buffers.
    Global,
    /// Special mark ('.^<> etc).
    Special,
}

impl MarkType {
    /// Returns the type for a mark character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a'..='z' => Some(Self::Local),
            'A'..='Z' => Some(Self::Global),
            '.' | '^' | '\'' | '`' | '<' | '>' | '[' | ']' | '"' => Some(Self::Special),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_new() {
        let buffer = BufferId::new(1);
        let mark = Mark::new(buffer, Position::new(5, 10));
        assert_eq!(mark.buffer, buffer);
        assert_eq!(mark.position.line, 5);
        assert_eq!(mark.position.col, 10);
    }

    #[test]
    fn test_mark_type_local() {
        assert_eq!(MarkType::from_char('a'), Some(MarkType::Local));
        assert_eq!(MarkType::from_char('z'), Some(MarkType::Local));
    }

    #[test]
    fn test_mark_type_global() {
        assert_eq!(MarkType::from_char('A'), Some(MarkType::Global));
        assert_eq!(MarkType::from_char('Z'), Some(MarkType::Global));
    }

    #[test]
    fn test_mark_type_special() {
        assert_eq!(MarkType::from_char('.'), Some(MarkType::Special));
        assert_eq!(MarkType::from_char('^'), Some(MarkType::Special));
        assert_eq!(MarkType::from_char('\''), Some(MarkType::Special));
    }

    #[test]
    fn test_mark_type_invalid() {
        assert_eq!(MarkType::from_char('0'), None);
        assert_eq!(MarkType::from_char('!'), None);
    }
}
