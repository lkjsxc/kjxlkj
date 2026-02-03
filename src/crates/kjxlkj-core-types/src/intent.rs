//! Intent types - commands that modify editor state.
//!
//! Modes emit intents; the core applies them as transactions.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{Mode, Position, Range, RegisterName, WindowId};

/// Editor intent - a typed command to modify state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    // --- Cursor movement ---
    /// Move cursor in direction.
    CursorMove(CursorDirection),
    /// Move cursor to specific position.
    CursorGoto(Position),
    /// Move cursor to line start.
    CursorLineStart,
    /// Move cursor to first non-blank.
    CursorFirstNonBlank,
    /// Move cursor to line end.
    CursorLineEnd,
    /// Move cursor to first line.
    CursorFileStart,
    /// Move cursor to last line.
    CursorFileEnd,
    /// Move cursor to specific line.
    CursorGotoLine(u32),

    // --- Mode transitions ---
    /// Enter a mode.
    EnterMode(Mode),
    /// Exit to Normal mode.
    ExitToNormal,

    // --- Text editing ---
    /// Insert text at cursor.
    InsertText(String),
    /// Insert newline and auto-indent.
    InsertNewline,
    /// Delete backward (backspace).
    DeleteBackward,
    /// Delete forward (delete key).
    DeleteForward,
    /// Delete character under cursor (x in normal).
    DeleteChar,
    /// Delete current line (dd).
    DeleteLine,
    /// Delete range.
    DeleteRange(Range),
    /// Yank (copy) current line.
    YankLine,
    /// Yank range.
    YankRange(Range),
    /// Paste after cursor.
    PasteAfter,
    /// Paste before cursor.
    PasteBefore,

    // --- Undo/Redo ---
    /// Undo last change.
    Undo,
    /// Redo last undone change.
    Redo,

    // --- Buffer operations ---
    /// Open file in new buffer.
    OpenFile(PathBuf),
    /// Write buffer to file.
    WriteBuffer { path: Option<PathBuf>, force: bool },
    /// Close buffer.
    CloseBuffer { force: bool },

    // --- Window operations ---
    /// Focus window.
    FocusWindow(WindowId),
    /// Split window horizontally.
    SplitHorizontal,
    /// Split window vertically.
    SplitVertical,
    /// Close window.
    CloseWindow,
    /// Navigate to window in direction.
    NavigateWindow(WindowDirection),

    // --- Visual mode ---
    /// Extend selection in direction.
    ExtendSelection(CursorDirection),
    /// Clear selection and return to normal.
    ClearSelection,

    // --- Command execution ---
    /// Execute ex command.
    ExecuteCommand(String),
    /// Run external command.
    RunExternalCommand(String),

    // --- Scrolling ---
    /// Scroll viewport.
    Scroll(ScrollDirection),
    /// Center cursor in viewport.
    CenterCursor,

    // --- Registers ---
    /// Set active register.
    SetRegister(RegisterName),

    // --- Misc ---
    /// No operation (used for unbound keys).
    Noop,
    /// Quit editor.
    Quit { force: bool },
}

/// Cursor movement direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorDirection {
    Left,
    Right,
    Up,
    Down,
    WordForward,
    WordBackward,
    WordEndForward,
}

/// Window navigation direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Scroll direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up(u32),
    Down(u32),
    HalfPageUp,
    HalfPageDown,
    PageUp,
    PageDown,
}
