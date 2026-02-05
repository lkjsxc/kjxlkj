//! Editor events and intents.

use serde::{Deserialize, Serialize};

use crate::{BufferId, Mode, Position, Range};

/// Keyboard modifier flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Modifier {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Modifier {
    pub const NONE: Modifier = Modifier {
        ctrl: false,
        alt: false,
        shift: false,
    };

    pub const CTRL: Modifier = Modifier {
        ctrl: true,
        alt: false,
        shift: false,
    };

    pub const ALT: Modifier = Modifier {
        ctrl: false,
        alt: true,
        shift: false,
    };

    pub const SHIFT: Modifier = Modifier {
        ctrl: false,
        alt: false,
        shift: true,
    };
}

/// A key event from terminal input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyEvent {
    Char(char, Modifier),
    Escape,
    Enter,
    Backspace,
    Delete,
    Tab,
    BackTab,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
}

impl KeyEvent {
    /// Create a character key with no modifiers.
    pub fn char(c: char) -> Self {
        Self::Char(c, Modifier::NONE)
    }

    /// Create a character key with Ctrl modifier.
    pub fn ctrl(c: char) -> Self {
        Self::Char(c, Modifier::CTRL)
    }
}

/// High-level editor events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditorEvent {
    Key(KeyEvent),
    Resize { width: u16, height: u16 },
    Focus(bool),
    Quit,
}

/// Typed intents emitted by mode handling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Intent {
    // Mode transitions
    EnterMode(Mode),

    // Cursor movement
    MoveUp(usize),
    MoveDown(usize),
    MoveLeft(usize),
    MoveRight(usize),
    MoveToLineStart,
    MoveToFirstNonBlank,
    MoveToLineEnd,
    MoveToDocumentStart,
    MoveToDocumentEnd,
    MoveToLine(usize),
    MoveWordForward(usize),
    MoveWordBackward(usize),
    MoveWordEnd(usize),
    MoveBigWordForward(usize),
    MoveBigWordBackward(usize),
    MoveBigWordEnd(usize),
    FindChar { char: char, forward: bool, till: bool },
    RepeatFindChar { reverse: bool },
    MatchBracket,

    // Scrolling
    ScrollUp(usize),
    ScrollDown(usize),
    ScrollHalfPageUp,
    ScrollHalfPageDown,
    ScrollPageUp,
    ScrollPageDown,
    CenterCursor,
    CursorToTop,
    CursorToBottom,

    // Text insertion
    InsertChar(char),
    InsertNewline,
    InsertNewlineAbove,
    InsertNewlineBelow,

    // Text deletion
    DeleteChar,
    DeleteCharBackward,
    DeleteLine(usize),
    DeleteToLineEnd,
    DeleteToLineStart,
    DeleteWord,
    DeleteWordBackward,
    DeleteRange(Range),

    // Operators
    Yank(Range),
    YankLine(usize),
    Paste { after: bool },
    Change(Range),
    ChangeLine(usize),
    ChangeToLineEnd,
    Indent(Range),
    Outdent(Range),

    // Undo/Redo
    Undo,
    Redo,

    // Repeat
    RepeatLastChange,

    // Visual mode
    StartVisual,
    StartVisualLine,
    StartVisualBlock,
    ExpandSelection(Range),

    // Search
    SearchForward(String),
    SearchBackward(String),
    NextSearchMatch,
    PrevSearchMatch,
    ClearSearch,

    // Ex commands
    ExecuteCommand(String),
    OpenCommandLine,
    OpenSearchForward,
    OpenSearchBackward,

    // Buffer operations
    WriteBuffer(Option<String>),
    WriteAllBuffers,
    EditFile(String),
    NextBuffer,
    PrevBuffer,
    SelectBuffer(BufferId),
    CloseBuffer(BufferId),

    // Window operations
    SplitHorizontal,
    SplitVertical,
    CloseWindow,
    OnlyWindow,
    FocusWindowUp,
    FocusWindowDown,
    FocusWindowLeft,
    FocusWindowRight,
    NextWindow,
    PrevWindow,

    // Features
    ToggleFileExplorer,
    ToggleTerminal,
    OpenFuzzyFinder,
    OpenLiveGrep,

    // Macros
    StartRecordingMacro(char),
    StopRecordingMacro,
    PlayMacro(char, usize),

    // Marks
    SetMark(char),
    GotoMark(char),
    GotoMarkLine(char),

    // Registers
    SelectRegister(char),

    // Quit
    Quit,
    QuitForce,
    QuitAll,
    WriteQuit,

    // No operation
    Noop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_event_char() {
        let k = KeyEvent::char('a');
        assert_eq!(k, KeyEvent::Char('a', Modifier::NONE));
    }

    #[test]
    fn key_event_ctrl() {
        let k = KeyEvent::ctrl('c');
        assert_eq!(k, KeyEvent::Char('c', Modifier::CTRL));
    }

    #[test]
    fn modifier_defaults() {
        let m = Modifier::default();
        assert!(!m.ctrl);
        assert!(!m.alt);
        assert!(!m.shift);
    }
}
