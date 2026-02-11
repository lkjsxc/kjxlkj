//! Typed actions emitted by the input pipeline.

use crate::{BufferId, Key, KeyModifiers, Mode, Operator};
use serde::{Deserialize, Serialize};

/// Typed editor action emitted after input decode and mapping resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Insert a character at cursor position.
    InsertChar(char),
    /// Delete character under cursor (forward).
    DeleteCharForward,
    /// Delete character before cursor (backward).
    DeleteCharBackward,
    /// Delete current line.
    DeleteLine,
    /// Yank current line.
    YankLine,
    /// Move cursor by motion.
    Motion(Motion),
    /// Enter a specific mode.
    EnterMode(Mode),
    /// Return to Normal mode.
    ExitToNormal,
    ExCommand(String),
    Resize(u16, u16),
    Paste(String),
    FocusGained,
    FocusLost,
    Quit,
    /// Force quit without saving.
    ForceQuit,
    /// Write current buffer.
    Write,
    /// Write and quit.
    WriteQuit,
    SplitHorizontal,
    SplitVertical,
    CloseWindow,
    FocusDirection(Direction),
    /// Cycle focus to next window.
    FocusCycle,
    /// Only keep current window.
    WindowOnly,
    /// Open explorer.
    OpenExplorer,
    /// Open terminal.
    OpenTerminal,
    /// Navigate to previous focus.
    FocusPrevious,
    Undo,
    Redo,
    DotRepeat,
    /// No-op (unhandled key).
    Noop,
    /// Raw key forwarded (for terminal insert mode).
    ForwardKey(Key, KeyModifiers),
    /// Open a file by path.
    OpenFile(String),
    /// Switch to buffer by ID.
    SwitchBuffer(BufferId),
    /// Next buffer.
    NextBuffer,
    /// Previous buffer.
    PreviousBuffer,
    /// Delete buffer.
    DeleteBuffer,
    /// Append at end of line (A command).
    AppendEndOfLine,
    /// Insert at first non-blank (I command).
    InsertFirstNonBlank,
    /// Open line below.
    OpenLineBelow,
    /// Open line above.
    OpenLineAbove,
    /// Join lines.
    JoinLines,
    /// Toggle case of character under cursor.
    ToggleCase,
    /// Replace character under cursor.
    ReplaceChar(char),
    PutAfter,
    PutBefore,
    ScrollCenter,
    ScrollTop,
    ScrollBottom,
    /// Delete word backward (insert mode Ctrl-w).
    DeleteWordBackward,
    /// Delete to line start (insert mode Ctrl-u).
    DeleteToLineStart,
    /// Operator applied linewise (e.g. dd, yy, cc).
    OperatorLine(Operator),
    /// Operator applied with a counted motion.
    OperatorMotion(Operator, Motion, usize),
    /// Substitute char (s): delete char + enter insert.
    SubstituteChar,
    /// Substitute line (S): delete line content + enter insert.
    SubstituteLine,
    /// Change to end of line (C).
    ChangeToEnd,
    /// Delete to end of line (D).
    DeleteToEnd,
    /// Yank current line (Y).
    YankCurrentLine,
    /// Join lines without space (gJ).
    JoinLinesNoSpace,
    /// Show register contents (:registers).
    ShowRegisters,
    /// Search forward for word under cursor (*).
    StarSearchForward,
    /// Search backward for word under cursor (#).
    StarSearchBackward,
    /// Clear search highlighting (:nohlsearch).
    ClearSearchHighlight,
    /// g* forward (partial match, no word boundaries).
    GStarSearchForward,
    /// g# backward (partial match, no word boundaries).
    GStarSearchBackward,
    /// Increment number under/after cursor (<C-a>).
    IncrementNumber,
    /// Decrement number under/after cursor (<C-x>).
    DecrementNumber,
    /// Set an editor option (:set opt=val).
    SetOption(String, String),
}

/// Cursor motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Motion {
    // Character motions
    Left,
    Right,
    Up,
    Down,
    // Line motions
    LineStart,
    LineEnd,
    FirstNonBlank,
    LastNonBlank,
    // Word/WORD motions
    WordForward,
    WordBackward,
    WordEndForward,
    WordEndBackward,
    BigWordForward,
    BigWordBackward,
    BigWordEndForward,
    BigWordEndBackward,
    // Sentence/paragraph motions
    SentenceForward,
    SentenceBackward,
    ParagraphForward,
    ParagraphBackward,
    // Find/till motions
    FindForward(char),
    FindBackward(char),
    TillForward(char),
    TillBackward(char),
    RepeatFind,
    RepeatFindReverse,
    // Search repeat motions
    SearchNext,
    SearchPrev,
    // Document motions
    GotoLine(usize),
    GotoFirstLine,
    GotoLastLine,
    // Window motions
    WindowTop,
    WindowMiddle,
    WindowBottom,
    MatchParen,
    PageDown, PageUp, HalfPageDown, HalfPageUp, ScrollDown, ScrollUp,
    TextObjInner(char), TextObjAround(char),
}

/// Directional focus for window navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_quit_is_distinct() {
        assert_ne!(Action::Quit, Action::ForceQuit);
    }
}
