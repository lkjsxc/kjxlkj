//! Intent types emitted by modes.

use kjxlkj_core_edit::{Motion, Operator};
use kjxlkj_core_types::{Mode, RegisterName};

/// An intent represents a user action to be applied by the core.
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    /// No action.
    None,
    /// Move cursor.
    Move(Motion),
    /// Execute an operator.
    Execute(Operator),
    /// Insert text at cursor.
    InsertText(String),
    /// Delete character at cursor.
    DeleteChar,
    /// Delete character before cursor.
    DeleteCharBefore,
    /// Delete to start of line.
    DeleteToLineStart,
    /// Delete word before cursor.
    DeleteWordBefore,
    /// Insert newline.
    InsertNewline,
    /// Open line below.
    OpenLineBelow,
    /// Open line above.
    OpenLineAbove,
    /// Join lines.
    JoinLines { with_space: bool },
    /// Change mode.
    ChangeMode(Mode),
    /// Enter insert mode at position.
    EnterInsert { at_line_end: bool, after_cursor: bool },
    /// Start visual selection.
    StartVisual(kjxlkj_core_types::SelectionKind),
    /// Replace character.
    ReplaceChar(char),
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Repeat last change.
    RepeatChange,
    /// Set register.
    SetRegister(RegisterName),
    /// Paste from register.
    Paste { before: bool, cursor_at_end: bool },
    /// Start macro recording.
    StartMacro(char),
    /// Stop macro recording.
    StopMacro,
    /// Play macro.
    PlayMacro(char),
    /// Set mark.
    SetMark(char),
    /// Jump to mark.
    JumpToMark { mark: char, line_start: bool },
    /// Enter command mode.
    EnterCommand,
    /// Search forward.
    SearchForward,
    /// Search backward.
    SearchBackward,
    /// Next search match.
    NextMatch,
    /// Previous search match.
    PrevMatch,
    /// Search word under cursor.
    SearchWordUnderCursor { forward: bool, whole_word: bool },
    /// Scroll viewport.
    Scroll(ScrollIntent),
    /// Center cursor on screen.
    CenterCursor(CenterKind),
    /// Indent line(s).
    IndentLines { count: usize },
    /// Outdent line(s).
    OutdentLines { count: usize },
    /// Toggle case of character.
    ToggleCaseChar,
    /// Increment number.
    IncrementNumber,
    /// Decrement number.
    DecrementNumber,
    /// Copy from line above/below.
    CopyFromAdjacentLine { above: bool },
    /// Quit.
    Quit { force: bool },
    /// Write.
    Write { path: Option<String> },
    /// Write and quit.
    WriteQuit { path: Option<String> },
    /// Execute Ex command.
    ExecuteCommand(String),
    /// Cancel current operation.
    Cancel,
    /// Jump in jump list.
    JumpList { forward: bool },
    /// Jump in change list.
    ChangeList { forward: bool },
    /// Swap visual anchor.
    SwapVisualAnchor,
    /// Enter replace mode.
    EnterReplace,
}

/// Scroll intent kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollIntent {
    /// Half page down.
    HalfPageDown,
    /// Half page up.
    HalfPageUp,
    /// Full page down.
    FullPageDown,
    /// Full page up.
    FullPageUp,
    /// One line down (keep cursor).
    LineDown,
    /// One line up (keep cursor).
    LineUp,
}

/// Center cursor positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterKind {
    /// Center cursor vertically.
    Center,
    /// Move cursor line to top.
    Top,
    /// Move cursor line to bottom.
    Bottom,
    /// Top and first non-blank.
    TopFirstNonBlank,
    /// Center and first non-blank.
    CenterFirstNonBlank,
    /// Bottom and first non-blank.
    BottomFirstNonBlank,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn center_kind_center_eq() {
        assert_eq!(CenterKind::Center, CenterKind::Center);
    }
    
    #[test]
    fn center_kind_top_eq() {
        assert_eq!(CenterKind::Top, CenterKind::Top);
    }
    
    #[test]
    fn center_kind_bottom_eq() {
        assert_eq!(CenterKind::Bottom, CenterKind::Bottom);
    }

    #[test]
    fn intent_none_eq() {
        assert_eq!(Intent::None, Intent::None);
    }

    #[test]
    fn intent_delete_char_eq() {
        assert_eq!(Intent::DeleteChar, Intent::DeleteChar);
    }
}
