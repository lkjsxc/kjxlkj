//! Typed actions emitted by the input pipeline.
//!
//! See /docs/spec/architecture/input-decoding.md for the pipeline.

use crate::{BufferId, Key, KeyModifiers, Mode, WindowId};
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
    /// Execute an ex command string.
    ExCommand(String),
    /// Resize terminal.
    Resize(u16, u16),
    /// Paste text from bracketed paste.
    Paste(String),
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
    /// Quit the editor.
    Quit,
    /// Force quit without saving.
    ForceQuit,
    /// Write current buffer.
    Write,
    /// Write and quit.
    WriteQuit,
    /// Split window horizontally.
    SplitHorizontal,
    /// Split window vertically.
    SplitVertical,
    /// Close current window.
    CloseWindow,
    /// Focus window in direction.
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
    /// Undo last edit.
    Undo,
    /// Redo last undo.
    Redo,
    /// Dot-repeat last text-changing command.
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
    /// Put register after cursor.
    PutAfter,
    /// Put register before cursor.
    PutBefore,
    /// Scroll cursor to center.
    ScrollCenter,
    /// Scroll cursor to top.
    ScrollTop,
    /// Scroll cursor to bottom.
    ScrollBottom,
}

/// Cursor motion.
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
    WordEndForward,
    BigWordForward,
    BigWordBackward,
    BigWordEndForward,
    ParagraphForward,
    ParagraphBackward,
    GotoLine(usize),
    GotoFirstLine,
    GotoLastLine,
    MatchParen,
    PageDown,
    PageUp,
    HalfPageDown,
    HalfPageUp,
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
