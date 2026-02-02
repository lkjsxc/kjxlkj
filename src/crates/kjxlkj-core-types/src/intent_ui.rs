//! Mode and window intent types.

use serde::{Deserialize, Serialize};

/// Mode change intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModeIntent {
    /// Enter normal mode.
    EnterNormal,
    /// Enter insert mode at cursor.
    EnterInsert,
    /// Enter insert mode after cursor.
    EnterAppend,
    /// Enter insert mode at line start.
    EnterInsertLineStart,
    /// Enter insert mode at line end.
    EnterInsertLineEnd,
    /// Enter visual mode (charwise).
    EnterVisualChar,
    /// Enter visual mode (linewise).
    EnterVisualLine,
    /// Enter visual mode (blockwise).
    EnterVisualBlock,
    /// Enter command-line mode.
    EnterCommand,
    /// Enter replace mode.
    EnterReplace,
}

/// Window management intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowIntent {
    /// Split horizontally.
    SplitHorizontal,
    /// Split vertically.
    SplitVertical,
    /// Close current window.
    Close,
    /// Focus window in direction.
    Focus(WindowDirection),
    /// Resize window.
    Resize(WindowDirection, i32),
}

/// Window direction for navigation/resize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Command-line intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandIntent {
    /// Execute a command.
    Execute(String),
    /// Cancel command input.
    Cancel,
}

/// Search intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchIntent {
    /// Start forward search.
    Forward,
    /// Start backward search.
    Backward,
    /// Go to next match.
    Next,
    /// Go to previous match.
    Previous,
    /// Clear search highlight.
    ClearHighlight,
}
