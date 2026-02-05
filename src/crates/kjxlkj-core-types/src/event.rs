//! Event and intent types for the editor.

use super::{Mode, RegisterName, TextRange};
use serde::{Deserialize, Serialize};

/// Input event from terminal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputEvent {
    /// Key press.
    Key(KeyEvent),
    /// Terminal resize.
    Resize { cols: u16, rows: u16 },
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
    /// Paste from clipboard.
    Paste(String),
}

/// Key event.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEvent {
    /// Key code.
    pub code: KeyCode,
    /// Modifiers.
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    /// Create a new key event.
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    /// Create a key event with no modifiers.
    pub fn plain(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::NONE)
    }

    /// Create a key event with Ctrl modifier.
    pub fn ctrl(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::CONTROL)
    }
}

/// Key code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    /// Character key.
    Char(char),
    /// Function key.
    F(u8),
    /// Backspace.
    Backspace,
    /// Enter.
    Enter,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home.
    Home,
    /// End.
    End,
    /// Page up.
    PageUp,
    /// Page down.
    PageDown,
    /// Tab.
    Tab,
    /// Backtab (Shift+Tab).
    BackTab,
    /// Delete.
    Delete,
    /// Insert.
    Insert,
    /// Escape.
    Esc,
    /// Null (Ctrl+Space).
    Null,
}

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct KeyModifiers(u8);

impl KeyModifiers {
    /// No modifiers.
    pub const NONE: Self = Self(0);
    /// Shift modifier.
    pub const SHIFT: Self = Self(1);
    /// Control modifier.
    pub const CONTROL: Self = Self(2);
    /// Alt modifier.
    pub const ALT: Self = Self(4);

    /// Check if shift is pressed.
    pub fn shift(&self) -> bool {
        self.0 & 1 != 0
    }

    /// Check if control is pressed.
    pub fn ctrl(&self) -> bool {
        self.0 & 2 != 0
    }

    /// Check if alt is pressed.
    pub fn alt(&self) -> bool {
        self.0 & 4 != 0
    }

    /// Combine modifiers.
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Editor intent (action to be applied).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Intent {
    /// No operation.
    Nop,
    /// Insert text at cursor.
    InsertText(String),
    /// Delete text in range.
    DeleteRange(TextRange),
    /// Delete character under cursor.
    DeleteChar,
    /// Delete character before cursor.
    Backspace,
    /// Move cursor.
    MoveCursor(Motion),
    /// Change mode.
    ChangeMode(Mode),
    /// Enter insert mode at position.
    EnterInsert(InsertPosition),
    /// Execute operator with motion.
    Operator(Operator, Motion),
    /// Execute operator on selection.
    OperatorSelection(Operator),
    /// Start visual selection.
    StartVisual(VisualMode),
    /// Toggle visual selection mode.
    ToggleVisualMode(VisualMode),
    /// Set register.
    SetRegister(RegisterName),
    /// Execute ex command.
    ExCommand(String),
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Repeat last change.
    Repeat,
    /// Search forward.
    SearchForward(String),
    /// Search backward.
    SearchBackward(String),
    /// Next search match.
    NextMatch,
    /// Previous search match.
    PrevMatch,
    /// Open file.
    OpenFile(String),
    /// Save file.
    SaveFile,
    /// Save file as.
    SaveFileAs(String),
    /// Quit.
    Quit,
    /// Force quit.
    ForceQuit,
    /// Split window.
    SplitWindow(SplitDirection),
    /// Close window.
    CloseWindow,
    /// Focus window.
    FocusWindow(WindowFocus),
    /// Scroll viewport.
    Scroll(ScrollDirection, usize),
    /// Center viewport on cursor.
    CenterViewport,
    /// Record macro.
    RecordMacro(char),
    /// Stop recording macro.
    StopRecordMacro,
    /// Play macro.
    PlayMacro(char),
    /// Set mark.
    SetMark(char),
    /// Jump to mark.
    JumpToMark(char),
    /// Toggle file explorer.
    ToggleExplorer,
    /// Toggle terminal.
    ToggleTerminal,
    /// Open fuzzy finder.
    OpenFinder,
}

/// Motion types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Motion {
    /// Move left.
    Left,
    /// Move right.
    Right,
    /// Move up.
    Up,
    /// Move down.
    Down,
    /// Move to line start.
    LineStart,
    /// Move to first non-blank.
    FirstNonBlank,
    /// Move to line end.
    LineEnd,
    /// Move word forward.
    WordForward,
    /// Move word backward.
    WordBackward,
    /// Move WORD forward.
    BigWordForward,
    /// Move WORD backward.
    BigWordBackward,
    /// Move to end of word.
    WordEnd,
    /// Move to end of WORD.
    BigWordEnd,
    /// Move to next char.
    FindChar(char),
    /// Move to before next char.
    TillChar(char),
    /// Move to prev char.
    FindCharBack(char),
    /// Move to after prev char.
    TillCharBack(char),
    /// Move to matching bracket.
    MatchingBracket,
    /// Move to next paragraph.
    ParagraphForward,
    /// Move to prev paragraph.
    ParagraphBackward,
    /// Move to file start.
    FileStart,
    /// Move to file end.
    FileEnd,
    /// Move to line number.
    GoToLine(usize),
    /// Page down.
    PageDown,
    /// Page up.
    PageUp,
    /// Half page down.
    HalfPageDown,
    /// Half page up.
    HalfPageUp,
    /// To next search match.
    NextSearchMatch,
    /// To prev search match.
    PrevSearchMatch,
}

/// Operator types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    /// Delete.
    Delete,
    /// Yank (copy).
    Yank,
    /// Change (delete and enter insert).
    Change,
    /// Indent right.
    Indent,
    /// Indent left.
    Outdent,
    /// Toggle case.
    ToggleCase,
    /// To uppercase.
    Uppercase,
    /// To lowercase.
    Lowercase,
    /// Format.
    Format,
}

/// Visual mode type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualMode {
    /// Character-wise.
    Char,
    /// Line-wise.
    Line,
    /// Block.
    Block,
}

/// Insert position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsertPosition {
    /// Before cursor.
    Before,
    /// After cursor.
    After,
    /// At line start.
    LineStart,
    /// At line end.
    LineEnd,
    /// New line below.
    NewLineBelow,
    /// New line above.
    NewLineAbove,
}

/// Split direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Horizontal split.
    Horizontal,
    /// Vertical split.
    Vertical,
}

/// Window focus direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowFocus {
    /// Focus left.
    Left,
    /// Focus right.
    Right,
    /// Focus up.
    Up,
    /// Focus down.
    Down,
    /// Focus next.
    Next,
    /// Focus previous.
    Previous,
}

/// Scroll direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollDirection {
    /// Scroll up.
    Up,
    /// Scroll down.
    Down,
}
