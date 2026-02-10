//! Mode handler traits and result types.

use kjxlkj_core_types::KeyEvent;

/// Result of handling a key in a mode.
#[derive(Debug, Clone)]
pub enum HandleResult {
    /// Key was consumed, action(s) generated.
    Consumed(Vec<ModeAction>),
    /// Key was not handled.
    Ignored,
    /// Key is pending more input.
    Pending,
}

/// Actions that can be produced by mode handlers.
#[derive(Debug, Clone)]
pub enum ModeAction {
    /// Move cursor.
    MoveCursor(kjxlkj_core_edit::Motion, usize),
    /// Enter insert mode at position.
    EnterInsert(InsertPosition),
    /// Enter visual mode.
    EnterVisual(kjxlkj_core_types::VisualKind),
    /// Enter command mode.
    EnterCommand(kjxlkj_core_types::CommandKind),
    /// Enter replace mode.
    EnterReplace,
    /// Return to normal mode.
    ReturnNormal,
    /// Execute operator over region.
    ExecuteOperator(kjxlkj_core_edit::Operator, kjxlkj_core_edit::OperatorRegion),
    /// Insert text at cursor.
    InsertText(String),
    /// Delete at cursor.
    DeleteAtCursor(kjxlkj_core_edit::Direction),
    /// Execute command.
    ExecuteCommand(String),
    /// Search pattern.
    Search(String, kjxlkj_core_edit::Direction),
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Quit.
    Quit { force: bool },
    /// Write buffer.
    Write(Option<String>),
    /// Open file.
    OpenFile(String),
    /// Open explorer.
    OpenExplorer,
    /// Open terminal.
    OpenTerminal,
    /// Split window.
    Split(kjxlkj_core_types::SplitDirection),
    /// Close window.
    CloseWindow,
    /// Focus window.
    FocusWindow(WindowTarget),
    /// Scroll viewport.
    Scroll(ScrollAction),
    /// Set mark.
    SetMark(char),
    /// Jump to mark.
    JumpToMark(char),
    /// Record macro.
    RecordMacro(char),
    /// Stop recording macro.
    StopRecordMacro,
    /// Play macro.
    PlayMacro(char),
    /// Repeat last change.
    Repeat,
    /// Put register content.
    Put { before: bool },
    /// Yank to register.
    Yank(kjxlkj_core_edit::OperatorRegion),
}

/// Insert position relative to cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    /// Before cursor (i).
    Before,
    /// After cursor (a).
    After,
    /// At end of line (A).
    EndOfLine,
    /// At first non-blank (I).
    FirstNonBlank,
    /// New line below (o).
    NewLineBelow,
    /// New line above (O).
    NewLineAbove,
}

/// Window focus target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowTarget {
    /// Focus by direction.
    Direction(FocusDirection),
    /// Next window.
    Next,
    /// Previous window.
    Previous,
    /// Last focused window.
    Last,
    /// Top-left window.
    TopLeft,
    /// Bottom-right window.
    BottomRight,
}

/// Focus direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    /// Left.
    Left,
    /// Right.
    Right,
    /// Up.
    Up,
    /// Down.
    Down,
}

/// Scroll action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAction {
    /// Scroll up by lines.
    Up(usize),
    /// Scroll down by lines.
    Down(usize),
    /// Page up.
    PageUp,
    /// Page down.
    PageDown,
    /// Half page up.
    HalfPageUp,
    /// Half page down.
    HalfPageDown,
    /// Center cursor.
    Center,
    /// Cursor to top.
    Top,
    /// Cursor to bottom.
    Bottom,
}

/// Trait for mode handlers.
pub trait ModeHandler {
    /// Handle a key event.
    fn handle(&mut self, key: &KeyEvent) -> HandleResult;
}
