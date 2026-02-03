//! Events and actions for the editor system.

use serde::{Deserialize, Serialize};

use crate::{BufferId, Mode};

/// Motion types for operator + motion combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Motion {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
    FirstNonBlank,
    WordForward,
    WordBackward,
    WordEnd,
    FileStart,
    FileEnd,
    /// Current line (for double-operator like dd, yy).
    CurrentLine,
}

/// Operator types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
}

/// Actions that the core task can execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorAction {
    /// Move cursor left.
    CursorLeft,
    /// Move cursor right.
    CursorRight,
    /// Move cursor up.
    CursorUp,
    /// Move cursor down.
    CursorDown,
    /// Move to start of line (column 0).
    LineStart,
    /// Move to end of line.
    LineEnd,
    /// Move to first non-blank character (^).
    FirstNonBlank,
    /// Move to next word start (w).
    WordForward,
    /// Move to next WORD start (W).
    WORDForward,
    /// Move to previous word start (b).
    WordBackward,
    /// Move to previous WORD start (B).
    WORDBackward,
    /// Move to word end (e).
    WordEnd,
    /// Move to WORD end (E).
    WORDEnd,
    /// Move to file start (gg).
    FileStart,
    /// Move to file end (G).
    FileEnd,
    /// Insert character at cursor.
    InsertChar(char),
    /// Insert newline at cursor.
    InsertNewline,
    /// Delete character before cursor (backspace).
    DeleteCharBefore,
    /// Delete character at cursor (x command).
    DeleteCharAt,
    /// Delete current line (dd command).
    DeleteLine,
    /// Yank current line (yy command).
    YankLine,
    /// Paste after cursor (p command).
    PasteAfter,
    /// Operator applied to a motion (e.g., dw, cw, yw).
    OperatorMotion { operator: Operator, motion: Motion, count: Option<u32> },
    /// Undo last change.
    Undo,
    /// Redo last undone change.
    Redo,
    /// Enter insert mode.
    EnterInsertMode,
    /// Enter insert mode after cursor.
    EnterInsertModeAfter,
    /// Enter insert mode at end of line.
    EnterInsertModeEndOfLine,
    /// Open line below and enter insert mode.
    OpenLineBelow,
    /// Enter visual mode.
    EnterVisualMode,
    /// Enter visual line mode.
    EnterVisualLineMode,
    /// Enter replace mode.
    EnterReplaceMode,
    /// Enter command mode.
    EnterCommandMode,
    /// Return to normal mode.
    ReturnToNormalMode,
    /// Execute ex command.
    ExecuteCommand(String),
    /// Quit editor.
    Quit { force: bool },
    /// Write buffer to file.
    Write { path: Option<String> },
    /// Edit file.
    EditFile { path: String, force: bool },
    /// Run external command.
    RunExternal(String),
    /// No operation.
    Nop,
}

/// Events emitted by the editor for external consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorEvent {
    /// Mode changed.
    ModeChanged(Mode),
    /// Buffer content changed.
    BufferChanged(BufferId),
    /// Cursor moved.
    CursorMoved,
    /// Status message.
    StatusMessage(String),
    /// Error message.
    ErrorMessage(String),
    /// Quit requested.
    QuitRequested,
}

/// Requests to services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceRequest {
    /// Read file from filesystem.
    ReadFile { path: String },
    /// Write file to filesystem.
    WriteFile { path: String, content: String },
    /// Execute external command.
    ExecuteCommand { command: String },
}

/// Events from services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceEvent {
    /// File read completed.
    FileRead { path: String, content: String },
    /// File write completed.
    FileWritten { path: String },
    /// Command output.
    CommandOutput { output: String },
    /// Service error.
    Error { message: String },
}
