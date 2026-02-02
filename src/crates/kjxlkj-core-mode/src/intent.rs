//! Intent types emitted by mode handlers.

use kjxlkj_core_edit::{Motion, Operator, TextObject};
use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

/// Kind of intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntentKind {
    /// No-op.
    Noop,

    /// Move cursor.
    Motion(Motion),

    /// Apply operator with motion.
    OperatorMotion { op: Operator, motion: Motion },

    /// Apply operator with text object.
    OperatorTextObject { op: Operator, text_object: TextObject },

    /// Apply operator on current line.
    OperatorLine { op: Operator },

    /// Insert text.
    InsertText { text: String },

    /// Delete character at cursor.
    DeleteChar,

    /// Delete character before cursor.
    Backspace,

    /// Insert newline.
    InsertNewline,

    /// Change mode.
    ChangeMode(Mode),

    /// Undo.
    Undo,

    /// Redo.
    Redo,

    /// Repeat last change.
    Repeat,

    /// Execute ex command.
    ExCommand { command: String },

    /// Search forward.
    SearchForward { pattern: String },

    /// Search backward.
    SearchBackward { pattern: String },

    /// Next search match.
    NextMatch,

    /// Previous search match.
    PrevMatch,

    /// Quit editor.
    Quit,

    /// Save buffer.
    Save,

    /// Save and quit.
    SaveQuit,
}

/// An intent emitted by mode processing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    /// Kind of intent.
    pub kind: IntentKind,
    /// Repeat count.
    pub count: usize,
}

impl Intent {
    /// Creates a new intent.
    pub fn new(kind: IntentKind) -> Self {
        Self { kind, count: 1 }
    }

    /// Creates a no-op intent.
    pub fn noop() -> Self {
        Self::new(IntentKind::Noop)
    }

    /// Creates a motion intent.
    pub fn motion(motion: Motion) -> Self {
        Self::new(IntentKind::Motion(motion))
    }

    /// Creates a mode change intent.
    pub fn change_mode(mode: Mode) -> Self {
        Self::new(IntentKind::ChangeMode(mode))
    }

    /// Sets the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }
}
