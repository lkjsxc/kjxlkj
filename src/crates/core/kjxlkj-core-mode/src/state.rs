//! Mode state management.

use kjxlkj_core_types::{Mode, VisualKind, CommandKind, PendingOperator};

/// Mode state with additional context.
#[derive(Debug, Clone, Default)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Accumulated count for commands.
    pub count: Option<usize>,
    /// Selected register.
    pub register: Option<char>,
    /// Command line content (for Command mode).
    pub cmdline: String,
    /// Command line cursor position.
    pub cmdline_cursor: usize,
}

impl ModeState {
    /// Create a new mode state in Normal mode.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the effective count (default 1).
    pub fn effective_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Reset count and register.
    pub fn reset_prefix(&mut self) {
        self.count = None;
        self.register = None;
    }

    /// Transition to a new mode.
    pub fn transition(&mut self, new_mode: Mode) {
        self.mode = new_mode;
        self.reset_prefix();
    }

    /// Enter insert mode.
    pub fn enter_insert(&mut self) {
        self.transition(Mode::Insert);
    }

    /// Enter normal mode.
    pub fn enter_normal(&mut self) {
        self.transition(Mode::Normal);
    }

    /// Enter visual mode.
    pub fn enter_visual(&mut self, kind: VisualKind) {
        self.transition(Mode::Visual(kind));
    }

    /// Enter command mode.
    pub fn enter_command(&mut self, kind: CommandKind) {
        self.mode = Mode::Command(kind);
        self.cmdline.clear();
        self.cmdline_cursor = 0;
        self.reset_prefix();
    }

    /// Enter operator-pending mode.
    pub fn enter_operator_pending(&mut self, op: PendingOperator) {
        self.mode = Mode::OperatorPending(op);
    }

    /// Enter replace mode.
    pub fn enter_replace(&mut self) {
        self.transition(Mode::Replace);
    }

    /// Enter insert-normal mode.
    pub fn enter_insert_normal(&mut self) {
        self.mode = Mode::InsertNormal;
    }

    /// Enter terminal insert mode.
    pub fn enter_terminal_insert(&mut self) {
        self.transition(Mode::TerminalInsert);
    }
}
