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
    /// Last non-blank character on line (g_).
    LastNonBlank,
    /// First non-blank of next line (+, Enter).
    NextLineStart,
    /// First non-blank of previous line (-).
    PrevLineStart,
    WordForward,
    WordBackward,
    WordEnd,
    /// Word end backward (ge).
    WordEndBackward,
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
    /// Match bracket (%).
    MatchBracket,
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
    /// Scroll half page down (Ctrl-d).
    ScrollHalfPageDown,
    /// Scroll half page up (Ctrl-u).
    ScrollHalfPageUp,
    /// Scroll full page down (Ctrl-f).
    ScrollPageDown,
    /// Scroll full page up (Ctrl-b).
    ScrollPageUp,
    /// Scroll one line down (Ctrl-e).
    ScrollLineDown,
    /// Scroll one line up (Ctrl-y).
    ScrollLineUp,
    /// Move cursor to top of screen (H).
    ScreenTop,
    /// Move cursor to middle of screen (M).
    ScreenMiddle,
    /// Move cursor to bottom of screen (L).
    ScreenBottom,
    /// Center cursor line on screen (zz).
    ScrollCursorCenter,
    /// Move cursor line to top of screen (zt).
    ScrollCursorTop,
    /// Move cursor line to bottom of screen (zb).
    ScrollCursorBottom,
    /// Scroll cursor to top and move to first non-blank (z<CR>).
    ScrollCursorTopFirstNonBlank,
    /// Scroll cursor to center and move to first non-blank (z.).
    ScrollCursorCenterFirstNonBlank,
    /// Scroll cursor to bottom and move to first non-blank (z-).
    ScrollCursorBottomFirstNonBlank,
    /// Move to start of line (column 0).
    LineStart,
    /// Move to end of line.
    LineEnd,
    /// Go to specific column (|).
    GoToColumn(u32),
    /// Move to middle of text on line (gm).
    LineMiddle,
    /// Move to first non-blank character (^).
    FirstNonBlank,
    /// Move to first non-blank of line with count offset (_).
    FirstNonBlankWithOffset(u32),
    /// Move to last non-blank character (g_).
    LastNonBlank,
    /// Move to first non-blank of next line (+, Enter).
    NextLineStart,
    /// Move to first non-blank of previous line (-).
    PrevLineStart,
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
    /// Move to previous word end (ge).
    WordEndBackward,
    /// Move to previous WORD end (gE).
    WORDEndBackward,
    /// Move to file start (gg).
    FileStart,
    /// Move to file end (G).
    FileEnd,
    /// Go to specific line number ({count}G, {count}gg).
    GoToLine(u32),
    /// Go to percentage of file ({count}%).
    GoToPercent(u32),
    /// Move to next sentence ()).
    SentenceForward,
    /// Move to previous sentence (().
    SentenceBackward,
    /// Move to next paragraph (}).
    ParagraphForward,
    /// Move to previous paragraph ({).
    ParagraphBackward,
    /// Match bracket (%).
    MatchBracket,
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
    /// Replace character at cursor (Replace mode - advances cursor).
    ReplaceChar(char),
    /// Replace single character at cursor (r command - no cursor advance).
    ReplaceSingleChar(char),
    /// Insert newline at cursor.
    InsertNewline,
    /// Delete character before cursor (backspace).
    DeleteCharBefore,
    /// Delete word before cursor (Ctrl-w in insert mode).
    DeleteWordBefore,
    /// Delete to start of line (Ctrl-u in insert mode).
    DeleteToLineStart,
    /// Indent current line in insert mode (Ctrl-t).
    InsertIndent,
    /// Outdent current line in insert mode (Ctrl-d).
    InsertOutdent,
    /// Copy character from line above (Ctrl-y in insert mode).
    InsertCopyAbove,
    /// Copy character from line below (Ctrl-e in insert mode).
    InsertCopyBelow,
    /// Insert register contents (Ctrl-r {reg} in insert mode).
    InsertRegister(char),
    /// Delete character at cursor (x command).
    DeleteCharAt,
    /// Delete current line (dd command).
    DeleteLine,
    /// Delete to end of line (D command).
    DeleteToEndOfLine,
    /// Yank current line (yy command).
    YankLine,
    /// Change to end of line (C command).
    ChangeToEndOfLine,
    /// Substitute character under cursor (s command).
    SubstituteChar,
    /// Substitute entire line (S command).
    SubstituteLine,
    /// Join current line with next (J command).
    JoinLines,
    /// Join current line with next without spaces (gJ command).
    JoinLinesNoSpace,
    /// Toggle case of character under cursor (~).
    ToggleCaseChar,
    /// Toggle case with motion (g~{motion}).
    ToggleCaseMotion { motion: Motion, count: Option<u32> },
    /// Toggle case of current line (g~~).
    ToggleCaseLine,
    /// Uppercase with motion (gU{motion}).
    UppercaseMotion { motion: Motion, count: Option<u32> },
    /// Uppercase current line (gUU).
    UppercaseLine,
    /// Lowercase with motion (gu{motion}).
    LowercaseMotion { motion: Motion, count: Option<u32> },
    /// Lowercase current line (guu).
    LowercaseLine,
    /// Increment number under cursor (Ctrl-A).
    IncrementNumber { amount: i32 },
    /// Decrement number under cursor (Ctrl-X).
    DecrementNumber { amount: i32 },
    /// Paste after cursor (p command).
    PasteAfter,
    /// Paste before cursor (P command).
    PasteBefore,
    /// Paste after cursor with cursor at end (gp command).
    PasteAfterCursorEnd,
    /// Paste before cursor with cursor at end (gP command).
    PasteBeforeCursorEnd,
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
    /// Enter insert mode at first non-blank of line.
    EnterInsertModeLineStart,
    /// Open line below and enter insert mode.
    OpenLineBelow,
    /// Open line above and enter insert mode.
    OpenLineAbove,
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
    /// Search forward for word under cursor (*).
    SearchWordForward,
    /// Search backward for word under cursor (#).
    SearchWordBackward,
    /// Partial search forward for word under cursor (g*).
    SearchPartialWordForward,
    /// Partial search backward for word under cursor (g#).
    SearchPartialWordBackward,
    /// Delete visual selection.
    VisualDelete,
    /// Yank visual selection.
    VisualYank,
    /// Change visual selection (delete and enter insert mode).
    VisualChange,
    /// Swap to other end of visual selection (o command in visual mode).
    VisualSwapEnd,
    /// Indent visual selection (> in visual mode).
    VisualIndent,
    /// Outdent visual selection (< in visual mode).
    VisualOutdent,
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
