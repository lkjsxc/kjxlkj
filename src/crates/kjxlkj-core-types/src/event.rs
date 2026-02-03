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
    /// Find char forward (f).
    FindCharForward(char),
    /// Find char backward (F).
    FindCharBackward(char),
    /// Till char forward (t).
    TillCharForward(char),
    /// Till char backward (T).
    TillCharBackward(char),
    /// Go to mark exact position (` + mark).
    ToMarkExact(char),
    /// Go to mark line start (' + mark).
    ToMarkLine(char),
    /// Sentence forward ()).
    SentenceForward,
    /// Sentence backward (().
    SentenceBackward,
    /// Paragraph forward (}).
    ParagraphForward,
    /// Paragraph backward ({).
    ParagraphBackward,
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

/// Text object types for operator + text-object combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextObject {
    /// Inner word (iw) - word characters only.
    InnerWord,
    /// Around word (aw) - word plus trailing whitespace.
    AroundWord,
    /// Inner WORD (iW) - non-whitespace sequence.
    InnerWORD,
    /// Around WORD (aW) - WORD plus trailing whitespace.
    AroundWORD,
    /// Inner parentheses (i().
    InnerParen,
    /// Around parentheses (a().
    AroundParen,
    /// Inner brackets (i[).
    InnerBracket,
    /// Around brackets (a[).
    AroundBracket,
    /// Inner braces (i{).
    InnerBrace,
    /// Around braces (a{).
    AroundBrace,
    /// Inner double quotes (i").
    InnerDoubleQuote,
    /// Around double quotes (a").
    AroundDoubleQuote,
    /// Inner single quotes (i').
    InnerSingleQuote,
    /// Around single quotes (a').
    AroundSingleQuote,
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
    /// Move to next sentence ()).
    SentenceForward,
    /// Move to previous sentence (().
    SentenceBackward,
    /// Move to next paragraph (}).
    ParagraphForward,
    /// Move to previous paragraph ({).
    ParagraphBackward,
    /// Find char forward (f).
    FindCharForward(char),
    /// Find char backward (F).
    FindCharBackward(char),
    /// Till char forward (t).
    TillCharForward(char),
    /// Till char backward (T).
    TillCharBackward(char),
    /// Repeat last f/t/F/T motion.
    RepeatFindChar,
    /// Repeat last f/t/F/T motion in opposite direction.
    RepeatFindCharReverse,
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
    /// Operator applied to a text object (e.g., diw, ci", ya().
    OperatorTextObject { operator: Operator, text_object: TextObject },
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
    /// Enter visual block mode.
    EnterVisualBlockMode,
    /// Enter replace mode.
    EnterReplaceMode,
    /// Enter command mode.
    EnterCommandMode,
    /// Enter search mode (forward /).
    EnterSearchForward,
    /// Enter search mode (backward ?).
    EnterSearchBackward,
    /// Execute search with pattern.
    ExecuteSearch(String),
    /// Go to next search match (n).
    SearchNext,
    /// Go to previous search match (N).
    SearchPrev,
    /// Delete visual selection.
    VisualDelete,
    /// Yank visual selection.
    VisualYank,
    /// Change visual selection (delete and enter insert mode).
    VisualChange,
    /// Repeat last change (dot command).
    RepeatLastChange,
    /// Set a mark at current cursor position.
    SetMark(char),
    /// Jump to mark exact position.
    JumpToMarkExact(char),
    /// Jump to mark line (first non-blank on that line).
    JumpToMarkLine(char),
    /// Set the pending register for the next yank/delete/paste.
    SetPendingRegister(char),
    /// Start or stop macro recording.
    ToggleMacroRecording(char),
    /// Play back a macro from a register.
    PlayMacro(char),
    /// Repeat last played macro.
    RepeatLastMacro,
    /// Jump to older position in jump list (Ctrl-o).
    JumpListOlder,
    /// Jump to newer position in jump list (Ctrl-i).
    JumpListNewer,
    /// Jump to older position in change list (g;).
    ChangeListOlder,
    /// Jump to newer position in change list (g,).
    ChangeListNewer,
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
    /// Substitute command (:s/pattern/replacement/flags).
    Substitute {
        pattern: String,
        replacement: String,
        flags: String,
    },
    /// Global command (:g/pattern/command).
    Global {
        pattern: String,
        command: String,
        invert: bool, // true for :v (vglobal)
    },
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
