//! Mode state management.

use kjxlkj_core_types::intent::OperatorContext;
use kjxlkj_core_types::mode::{Mode, VisualMode};

/// Mode state including pending operators.
#[derive(Debug, Clone, Default)]
pub struct ModeState {
    /// Current mode.
    mode: Mode,
    /// Pending operator context (for operator-pending mode).
    pending_operator: Option<OperatorContext>,
    /// Count prefix accumulator.
    count: Option<usize>,
}

impl ModeState {
    /// Creates a new mode state starting in normal mode.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            pending_operator: None,
            count: None,
        }
    }

    /// Returns the current mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Sets the mode.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        // Clear pending state on mode change
        self.pending_operator = None;
        self.count = None;
    }

    /// Returns the pending operator context.
    pub fn pending_operator(&self) -> Option<&OperatorContext> {
        self.pending_operator.as_ref()
    }

    /// Sets the pending operator.
    pub fn set_pending_operator(&mut self, context: OperatorContext) {
        self.pending_operator = Some(context);
        self.mode = Mode::OperatorPending;
    }

    /// Clears the pending operator.
    pub fn clear_pending_operator(&mut self) {
        self.pending_operator = None;
        if self.mode == Mode::OperatorPending {
            self.mode = Mode::Normal;
        }
    }

    /// Returns the current count prefix.
    pub fn count(&self) -> Option<usize> {
        self.count
    }

    /// Returns the count or default.
    pub fn count_or(&self, default: usize) -> usize {
        self.count.unwrap_or(default)
    }

    /// Appends a digit to the count.
    pub fn append_count_digit(&mut self, digit: u8) {
        let d = digit as usize;
        self.count = Some(self.count.unwrap_or(0) * 10 + d);
    }

    /// Clears the count.
    pub fn clear_count(&mut self) {
        self.count = None;
    }

    /// Returns true if in insert-like mode.
    pub fn is_insert_mode(&self) -> bool {
        matches!(self.mode, Mode::Insert | Mode::Replace)
    }

    /// Returns true if in visual mode.
    pub fn is_visual_mode(&self) -> bool {
        matches!(self.mode, Mode::Visual(_))
    }

    /// Returns the visual mode variant if in visual mode.
    pub fn visual_mode(&self) -> Option<VisualMode> {
        match self.mode {
            Mode::Visual(mode) => Some(mode),
            _ => None,
        }
    }
}
