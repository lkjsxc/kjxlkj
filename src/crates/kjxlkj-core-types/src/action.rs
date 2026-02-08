//! Action enum — the unified vocabulary for all editor actions.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    Direction, Motion, Operator, RegisterName, ScrollDirection,
    TextObject, VisualKind,
};

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

/// The unified action type dispatched from input to core.
///
/// Per /docs/spec/architecture/input-decoding.md, this enum covers all
/// categories: movement, editing, mode, command, buffer, window, search,
/// undo, macro, and system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    // -- Movement --
    /// Execute a motion with optional count.
    MoveCursor(Motion, u32),
    /// Scroll the viewport.
    Scroll(ScrollDirection, u32),

    // -- Editing --
    /// Insert a character at cursor.
    InsertChar(char),
    /// Delete with operator + motion.
    Delete(Motion, u32),
    /// Change with operator + motion (delete + enter Insert).
    Change(Motion, u32),
    /// Yank with motion.
    Yank(Motion, u32),
    /// Put register contents. `true` = before cursor.
    Put(bool),
    /// Replace character under cursor.
    ReplaceChar(char),
    /// Execute operator on a text object.
    OperatorTextObject(Operator, TextObject, u32),
    /// Double operator (linewise: dd, yy, cc, etc.).
    DoubleOperator(Operator, u32),
    /// Substitute character: delete char, enter Insert.
    SubstituteChar,
    /// Substitute line: delete line content, enter Insert.
    SubstituteLine,
    /// Change to end of line.
    ChangeToEnd,
    /// Delete char forward (`x`).
    DeleteCharForward,
    /// Delete char backward (`X`).
    DeleteCharBackward,
    /// Join lines (`J`).
    JoinLines,
    /// Join lines without space (`gJ`).
    JoinLinesNoSpace,
    /// Toggle case under cursor (`~`).
    ToggleCaseChar,
    /// Dot repeat.
    DotRepeat,
    /// Increment number under cursor.
    Increment(i64),

    // -- Mode --
    /// Enter insert mode at position.
    EnterInsert(InsertPosition),
    /// Enter visual mode with sub-kind.
    EnterVisual(VisualKind),
    /// Enter command mode (`:`, `/`, `?`).
    EnterCommand(CommandKind),
    /// Enter replace mode.
    EnterReplace,
    /// Return to normal mode.
    ReturnToNormal,
    /// Insert-normal mode (`Ctrl-O`).
    InsertNormal,

    // -- Operator pending --
    /// Enter operator-pending mode.
    EnterOperatorPending(Operator),

    // -- Command --
    /// Execute an ex command string.
    ExecuteCommand(String),
    /// Append character to command line.
    CmdlineChar(char),
    /// Backspace in command line.
    CmdlineBackspace,
    /// Tab completion in command line.
    CmdlineComplete,
    /// Command history navigation.
    CmdlineHistory(Direction),

    // -- Buffer --
    /// Open file path in current window.
    OpenFile(PathBuf),
    /// Write current buffer.
    Write,
    /// Write and quit.
    WriteQuit,
    /// Quit current window.
    Quit,
    /// Force quit without saving.
    ForceQuit,
    /// Quit all windows.
    QuitAll,
    /// Write all modified buffers.
    WriteAll,
    /// Write all and quit.
    WriteAllQuit,
    /// Switch to buffer by ID or name.
    SwitchBuffer(String),
    /// Next buffer.
    NextBuffer,
    /// Previous buffer.
    PrevBuffer,
    /// Delete buffer.
    DeleteBuffer,
    /// Alternate file (`Ctrl-^`).
    AlternateFile,

    // -- Window --
    /// Split window horizontally.
    SplitHorizontal,
    /// Split window vertically.
    SplitVertical,
    /// Close current window.
    CloseWindow,
    /// Focus window in direction.
    FocusWindow(Direction),
    /// Cycle to next window.
    CycleWindow,
    /// Resize window.
    ResizeWindow(Direction, i32),
    /// Equalize window sizes.
    EqualizeWindows,
    /// Move window to edge.
    MoveWindow(Direction),
    /// Zoom/maximize window.
    ZoomWindow,
    /// Rotate windows.
    RotateWindows(bool),

    // -- Search --
    /// Forward search with pattern.
    SearchForward(String),
    /// Backward search with pattern.
    SearchBackward(String),
    /// Jump to next match.
    NextMatch,
    /// Jump to previous match.
    PrevMatch,

    // -- Undo --
    /// Undo last change.
    Undo,
    /// Redo last undone change.
    Redo,

    // -- Marks --
    /// Set mark at cursor.
    SetMark(char),
    /// Jump to mark (exact position).
    JumpToMark(char),
    /// Jump to mark line (first non-blank).
    JumpToMarkLine(char),

    // -- Macro --
    /// Start recording macro into register.
    RecordMacro(char),
    /// Stop recording macro.
    StopRecordMacro,
    /// Play macro from register.
    PlayMacro(char, u32),

    // -- Register --
    /// Set register for next operator.
    SetRegister(RegisterName),

    // -- System --
    /// Terminal resize.
    Resize(u16, u16),
    /// Bracketed paste text.
    Paste(String),
    /// Focus gained event.
    FocusGained,
    /// Focus lost event.
    FocusLost,
    /// Quit signal (SIGTERM etc.).
    QuitSignal,

    // -- Session --
    /// Save session.
    SessionSave,
    /// Load session.
    SessionLoad,

    // -- Terminal --
    /// Spawn terminal.
    SpawnTerminal,

    /// No operation (sentinel / default).
    Nop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_variants_exist() {
        let _ = Action::MoveCursor(Motion::Left, 1);
        let _ = Action::InsertChar('a');
        let _ = Action::Quit;
        let _ = Action::Resize(80, 24);
        let _ = Action::Nop;
    }
}
