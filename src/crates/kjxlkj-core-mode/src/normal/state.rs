//! Normal mode state and awaiting char types.

use kjxlkj_core_types::RegisterName;

/// Types of character input we're waiting for.
#[derive(Debug, Clone, Copy)]
pub enum AwaitingChar {
    Replace,
    FindForward,
    FindBackward,
    TillForward,
    TillBackward,
    Mark,
    JumpMark,
    JumpMarkLine,
    Register,
    MacroRecord,
    MacroPlay,
}

/// Normal mode parsing state.
#[derive(Debug, Clone, Default)]
pub struct NormalModeState {
    /// Accumulated count.
    pub(super) count: Option<usize>,
    /// Pending operator.
    pub(super) pending_operator: Option<kjxlkj_core_edit::OperatorKind>,
    /// Pending register.
    pub(super) pending_register: Option<RegisterName>,
    /// Last find character motion.
    pub(super) last_find: Option<(kjxlkj_core_edit::MotionKind, char)>,
    /// Is awaiting character input (for r, f, t, etc).
    pub(super) awaiting_char: Option<AwaitingChar>,
}

impl NormalModeState {
    /// Create a new normal mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the state.
    pub fn reset(&mut self) {
        self.count = None;
        self.pending_operator = None;
        self.awaiting_char = None;
    }

    /// Get the current count (default 1).
    pub fn get_count(&self) -> usize {
        self.count.unwrap_or(1)
    }
}
