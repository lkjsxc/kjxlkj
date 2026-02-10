//! Mode state management.

use kjxlkj_core_types::{Mode, VisualKind, CommandKind, PendingOperator};

/// Pending prefix for multi-key sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PendingPrefix {
    #[default]
    None,
    /// 'g' prefix for 'gg', 'ge', etc.
    G,
    /// '"' register prefix.
    Register,
    /// 'm' mark set prefix.
    Mark,
    /// '\'' mark jump prefix.
    MarkJump,
    /// 'Z' prefix for ZZ/ZQ.
    Z,
    /// Ctrl-w window prefix.
    Window,
}

/// Mode state with additional context.
#[derive(Debug, Clone, Default)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Accumulated count for commands.
    pub count: Option<usize>,
    /// Selected register.
    pub register: Option<char>,
    /// Pending prefix for multi-key sequences.
    pub pending_prefix: PendingPrefix,
    /// Command line content (for Command mode).
    pub cmdline: String,
    /// Command line cursor position.
    pub cmdline_cursor: usize,
    /// IME composition state.
    pub ime_composing: bool,
    /// IME preedit text.
    pub ime_preedit: String,
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
        self.pending_prefix = PendingPrefix::None;
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

    /// Start IME composition.
    pub fn ime_start(&mut self) {
        self.ime_composing = true;
        self.ime_preedit.clear();
    }

    /// Update IME preedit text.
    pub fn ime_update(&mut self, preedit: &str) {
        self.ime_preedit = preedit.to_string();
    }

    /// Commit IME composition.
    pub fn ime_commit(&mut self) -> Option<String> {
        if self.ime_composing {
            self.ime_composing = false;
            let text = std::mem::take(&mut self.ime_preedit);
            if !text.is_empty() {
                return Some(text);
            }
        }
        None
    }

    /// Cancel IME composition.
    pub fn ime_cancel(&mut self) {
        self.ime_composing = false;
        self.ime_preedit.clear();
    }

    /// Check if IME is composing.
    pub fn is_composing(&self) -> bool {
        self.ime_composing
    }
}
