//! Editor actions dispatched through the core event loop.

use crate::mode::Mode;
use crate::terminal::TerminalId;

/// Top-level actions sent from input/services to core.
#[derive(Debug, Clone)]
pub enum Action {
    /// Terminal was resized.
    Resize(u16, u16),
    /// Bracketed paste payload.
    Paste(String),
    /// Terminal gained focus.
    FocusGained,
    /// Terminal lost focus.
    FocusLost,
    /// Quit the editor.
    Quit,
    /// Force quit discarding changes.
    ForceQuit,
    /// A decoded key that should be dispatched in current mode.
    KeyAction(KeyAction),
    /// A service response.
    ServiceResponse(ServiceResponse),
}

/// A key-driven action resolved by the mode dispatcher.
#[derive(Debug, Clone)]
pub enum KeyAction {
    /// Insert text at cursor.
    InsertChar(char),
    /// Delete character under cursor (forward).
    DeleteCharForward,
    /// Delete character before cursor (backward).
    DeleteCharBackward,
    /// Enter a new mode.
    EnterMode(Mode),
    /// Execute an ex command string.
    ExCommand(String),
    /// Move cursor by motion.
    Motion(MotionAction),
    /// Operator + motion/textobject.
    OperatorMotion {
        op: crate::mode::Operator,
        motion: MotionAction,
        count: usize,
    },
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Open line below.
    OpenLineBelow,
    /// Open line above.
    OpenLineAbove,
    /// Join lines.
    JoinLines,
    /// Put register after cursor.
    PutAfter,
    /// Put register before cursor.
    PutBefore,
    /// Append after cursor (`a`).
    InsertAppend,
    /// Append at end of line (`A`).
    InsertAppendEol,
    /// Insert at first non-blank (`I`).
    InsertFirstNonBlank,
    /// Window navigation: next window (Ctrl-w w).
    WindowNext,
    /// Window navigation: previous window (Ctrl-w W).
    WindowPrev,
    /// Window split horizontal (Ctrl-w s).
    WindowSplitH,
    /// Window split vertical (Ctrl-w v).
    WindowSplitV,
    /// Window close (Ctrl-w c / Ctrl-w q).
    WindowClose,
    /// Viewport: center cursor vertically (zz).
    ViewportCenter,
    /// Viewport: cursor to top of window (zt).
    ViewportTop,
    /// Viewport: cursor to bottom of window (zb).
    ViewportBottom,
    /// Open terminal window (`:terminal`, `<leader>t`).
    TerminalOpen,
    /// Open terminal in horizontal split (`<leader>th`).
    TerminalSplitH,
    /// Open terminal in vertical split (`<leader>tv`).
    TerminalSplitV,
    /// Toggle explorer window (`<leader>e`).
    ExplorerToggle,
    /// Reveal current file in explorer (`<leader>E`).
    ExplorerReveal,
    /// No-op (unrecognized key).
    Noop,
    /// Replace character at cursor (Replace mode overwrite).
    ReplaceChar(char),
    /// Backspace in replace mode (restores original).
    ReplaceBackspace,
    /// Window focus direction (Ctrl-w h/j/k/l).
    WindowFocusLeft,
    WindowFocusRight,
    WindowFocusUp,
    WindowFocusDown,
    /// Delete the selected visual region.
    VisualOperator(crate::mode::Operator),
}

/// Motion actions for cursor movement.
#[derive(Debug, Clone)]
pub enum MotionAction {
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
    PageDown,
    PageUp,
    HalfPageDown,
    HalfPageUp,
    GoToLine(usize),
    GoToFirstLine,
    GoToLastLine,
    MatchingBracket,
    NextSearch,
    PrevSearch,
    FindChar(char),
    FindCharBackward(char),
    TillChar(char),
    TillCharBackward(char),
}

/// Service responses from background tasks.
#[derive(Debug, Clone)]
pub enum ServiceResponse {
    /// File read complete.
    FileRead {
        path: std::path::PathBuf,
        content: String,
    },
    /// File write complete.
    FileWritten { path: std::path::PathBuf },
    /// Terminal output from PTY.
    TerminalOutput { id: TerminalId, data: Vec<u8> },
    /// Terminal exited.
    TerminalExited { id: TerminalId },
}
