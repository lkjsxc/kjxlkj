//! Intent types - the output of modal interpretation.

use kjxlkj_core_edit::{Motion, Operator, TextObject};
use kjxlkj_core_types::Mode;

/// Intent kind - what action the user wants to perform.
#[derive(Debug, Clone)]
pub enum IntentKind {
    // Mode transitions
    EnterInsert { after: bool },
    EnterInsertLine { below: bool },
    EnterVisual,
    EnterVisualLine,
    EnterVisualBlock,
    EnterCommand,
    EnterReplace,
    ExitToNormal,

    // Cursor movement
    Move(Motion),

    // Editing with operator
    OperatorMotion { operator: Operator, motion: Motion },
    OperatorTextObject { operator: Operator, object: TextObject },
    OperatorLine { operator: Operator },

    // Direct actions
    InsertChar(char),
    DeleteChar,
    DeleteCharBefore,
    NewlineBelow,
    NewlineAbove,
    JoinLines,

    // Undo/redo
    Undo,
    Redo,

    // Repeat
    RepeatLast,

    // Search
    SearchForward,
    SearchBackward,
    SearchNext,
    SearchPrev,

    // Scrolling
    ScrollUp(usize),
    ScrollDown(usize),
    ScrollHalfUp,
    ScrollHalfDown,

    // Command execution
    ExecuteCommand(String),

    // No-op or incomplete
    Pending,
    Noop,
}

/// Complete intent with context.
#[derive(Debug, Clone)]
pub struct Intent {
    /// The intent kind.
    pub kind: IntentKind,
    /// Count prefix (1 if not specified).
    pub count: usize,
    /// Register (if specified).
    pub register: Option<char>,
    /// Mode to return to after action.
    pub return_mode: Option<Mode>,
}

impl Intent {
    /// Create a new intent.
    pub fn new(kind: IntentKind) -> Self {
        Self {
            kind,
            count: 1,
            register: None,
            return_mode: None,
        }
    }

    /// Create a no-op intent.
    pub fn noop() -> Self {
        Self::new(IntentKind::Noop)
    }

    /// Create a pending intent.
    pub fn pending() -> Self {
        Self::new(IntentKind::Pending)
    }

    /// Set the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }

    /// Set the register.
    pub fn with_register(mut self, reg: char) -> Self {
        self.register = Some(reg);
        self
    }

    /// Check if this intent is pending more input.
    pub fn is_pending(&self) -> bool {
        matches!(self.kind, IntentKind::Pending)
    }

    /// Check if this intent changes mode.
    pub fn changes_mode(&self) -> bool {
        matches!(
            self.kind,
            IntentKind::EnterInsert { .. }
                | IntentKind::EnterInsertLine { .. }
                | IntentKind::EnterVisual
                | IntentKind::EnterVisualLine
                | IntentKind::EnterVisualBlock
                | IntentKind::EnterCommand
                | IntentKind::EnterReplace
                | IntentKind::ExitToNormal
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_intent() {
        let intent = Intent::noop();
        assert!(!intent.is_pending());
    }

    #[test]
    fn pending_intent() {
        let intent = Intent::pending();
        assert!(intent.is_pending());
    }

    #[test]
    fn mode_change_intents() {
        let intent = Intent::new(IntentKind::EnterInsert { after: false });
        assert!(intent.changes_mode());
    }
}
