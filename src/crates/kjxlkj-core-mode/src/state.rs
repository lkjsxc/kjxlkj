//! Mode state management.

use kjxlkj_core_types::{Mode, Position, Range};

/// Mode state machine.
#[derive(Debug, Clone)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Pending count for operations.
    pub count: Option<u32>,
    /// Pending operator (for operator-pending mode).
    pub pending_operator: Option<PendingOperator>,
    /// Command line buffer.
    pub command_line: String,
    /// Visual selection anchor.
    pub visual_anchor: Option<Position>,
}

impl Default for ModeState {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            count: None,
            pending_operator: None,
            command_line: String::new(),
            visual_anchor: None,
        }
    }
}

impl ModeState {
    /// Create a new mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the effective count (default 1).
    pub fn effective_count(&self) -> u32 {
        self.count.unwrap_or(1)
    }

    /// Clear pending state.
    pub fn clear_pending(&mut self) {
        self.count = None;
        self.pending_operator = None;
    }

    /// Enter a new mode.
    pub fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.clear_pending();

        if mode == Mode::Command {
            self.command_line.clear();
        }

        if !mode.is_visual() {
            self.visual_anchor = None;
        }
    }

    /// Exit to normal mode.
    pub fn exit_to_normal(&mut self) {
        self.enter_mode(Mode::Normal);
    }

    /// Start visual selection at position.
    pub fn start_visual(&mut self, mode: Mode, anchor: Position) {
        self.mode = mode;
        self.visual_anchor = Some(anchor);
        self.clear_pending();
    }

    /// Get the current visual selection range.
    pub fn visual_range(&self, cursor: Position) -> Option<Range> {
        self.visual_anchor
            .map(|anchor| Range::new(anchor, cursor).normalized())
    }

    /// Accumulate a digit into the count.
    pub fn accumulate_count(&mut self, digit: u32) {
        let current = self.count.unwrap_or(0);
        self.count = Some(current.saturating_mul(10).saturating_add(digit));
    }
}

/// Pending operator for operator-pending mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingOperator {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
}
