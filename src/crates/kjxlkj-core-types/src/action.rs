//! Editor action types representing all possible editor operations.

use serde::{Deserialize, Serialize};

use crate::types::{Direction, Mode, Position, Range};

/// All possible editor actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorAction {
    // Cursor movement
    CursorMove(Direction, usize),
    CursorTo(Position),

    // Text insertion
    InsertChar(char),
    InsertNewline,

    // Deletion
    DeleteChar,
    DeleteBack,
    DeleteRange(Range),

    // Yank / paste
    Yank(Range),
    Paste(Direction),

    // Mode
    ChangeMode(Mode),

    // Command execution
    ExecuteCommand(String),

    // Undo / redo
    Undo,
    Redo,

    // Session
    Quit,
    ForceQuit,
    Write(Option<String>),
    WriteQuit(Option<String>),

    // Search
    Search(String, Direction),
    SearchNext,
    SearchPrev,

    // Repeat
    Repeat,

    // Marks
    SetMark(char),
    JumpToMark(char),

    // Macros
    RecordMacro(char),
    PlayMacro(char),

    // Registers
    SelectRegister(char),

    // Scrolling
    Scroll(i64),
    ScrollPage(Direction),

    // Jump list
    JumpList(Direction),

    // Formatting / indentation
    Indent,
    Outdent,
    JoinLine(bool),

    // Case transforms
    ToggleCase,
    UpperCase,
    LowerCase,

    // Replace
    ReplaceChar(char),

    // Dot-repeat
    DotRepeat,

    // No-op
    Noop,
}

impl EditorAction {
    /// Returns `true` if the action modifies buffer content.
    pub fn is_edit(&self) -> bool {
        matches!(
            self,
            Self::InsertChar(_)
                | Self::InsertNewline
                | Self::DeleteChar
                | Self::DeleteBack
                | Self::DeleteRange(_)
                | Self::Paste(_)
                | Self::Indent
                | Self::Outdent
                | Self::JoinLine(_)
                | Self::ToggleCase
                | Self::UpperCase
                | Self::LowerCase
                | Self::ReplaceChar(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_is_not_edit() {
        assert!(!EditorAction::Noop.is_edit());
    }

    #[test]
    fn insert_char_is_edit() {
        assert!(EditorAction::InsertChar('a').is_edit());
    }
}
