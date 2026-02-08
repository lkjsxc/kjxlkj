//! Action sub-types: command kinds and insert positions.

use serde::{Deserialize, Serialize};

/// Command-line sub-mode for action context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandKind {
    Ex,
    SearchForward,
    SearchBackward,
}

/// Insert-mode entry position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsertPosition {
    /// `i` — before cursor.
    BeforeCursor,
    /// `a` — after cursor.
    AfterCursor,
    /// `I` — first non-blank.
    FirstNonBlank,
    /// `A` — end of line.
    EndOfLine,
    /// `o` — new line below.
    NewLineBelow,
    /// `O` — new line above.
    NewLineAbove,
}
