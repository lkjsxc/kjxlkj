//! Intent types for kjxlkj editor operations.
//!
//! Intents represent user-level operations that the core can execute.

pub use crate::intent_ui::{
    CommandIntent, ModeIntent, SearchIntent, WindowDirection, WindowIntent,
};
use crate::{ids::BufferId, position::Position, register::Register};
use serde::{Deserialize, Serialize};

/// A high-level user intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    /// Mode-related intents.
    Mode(ModeIntent),
    /// Cursor movement intents.
    Motion(MotionIntent),
    /// Edit operation intents.
    Edit(EditIntent),
    /// Buffer management intents.
    Buffer(BufferIntent),
    /// Window management intents.
    Window(WindowIntent),
    /// Command-line intents.
    Command(CommandIntent),
    /// Search intents.
    Search(SearchIntent),
}

/// Motion intents for cursor movement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionIntent {
    /// Move left by characters.
    Left(usize),
    /// Move right by characters.
    Right(usize),
    /// Move up by lines.
    Up(usize),
    /// Move down by lines.
    Down(usize),
    /// Move to start of line.
    LineStart,
    /// Move to first non-blank of line.
    FirstNonBlank,
    /// Move to end of line.
    LineEnd,
    /// Move to start of document.
    DocumentStart,
    /// Move to end of document.
    DocumentEnd,
    /// Move to specific line.
    GotoLine(usize),
    /// Move to specific position.
    GotoPosition(Position),
    /// Move forward by words.
    WordForward(usize),
    /// Move backward by words.
    WordBackward(usize),
    /// Move to end of word.
    WordEnd(usize),
    /// Page up.
    PageUp,
    /// Page down.
    PageDown,
    /// Half page up.
    HalfPageUp,
    /// Half page down.
    HalfPageDown,
}

/// Edit operation intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditIntent {
    /// Insert a character.
    InsertChar(char),
    /// Insert a string.
    InsertString(String),
    /// Insert a newline.
    InsertNewline,
    /// Delete character before cursor.
    DeleteCharBefore,
    /// Delete character at cursor.
    DeleteCharAt,
    /// Delete text covered by motion.
    Delete,
    /// Yank (copy) text covered by motion.
    Yank,
    /// Put (paste) from register.
    Put(PutDirection),
    /// Change text covered by motion.
    Change,
    /// Undo last change.
    Undo,
    /// Redo last undone change.
    Redo,
    /// Join lines.
    JoinLines,
    /// Indent.
    Indent,
    /// Outdent.
    Outdent,
}

/// Direction for put operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PutDirection {
    /// Put before cursor (P).
    Before,
    /// Put after cursor (p).
    After,
}

/// Buffer management intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BufferIntent {
    /// Open a new buffer.
    Open(String),
    /// Write buffer to file.
    Write(Option<String>),
    /// Close current buffer.
    Close,
    /// Force close without saving.
    ForceClose,
    /// Switch to buffer by id.
    Switch(BufferId),
    /// Switch to next buffer.
    Next,
    /// Switch to previous buffer.
    Previous,
}

/// Context for pending operator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorContext {
    /// The operator (d, y, c, etc.).
    pub operator: Operator,
    /// Optional register for the operation.
    pub register: Option<Register>,
    /// Count prefix.
    pub count: Option<usize>,
}

/// Operators that can be applied to motions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Operator {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
    Format,
    Lowercase,
    Uppercase,
    SwapCase,
}
