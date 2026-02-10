//! Mode dispatcher: routes keys to the appropriate mode handler.

use kjxlkj_core_types::{Key, KeyAction, KeyCode, KeyModifiers, Mode, MotionAction, Operator};

pub use crate::key_motion::key_to_motion;

/// Result of dispatching a key in the current mode.
#[derive(Debug)]
pub enum DispatchResult {
    /// An action to execute.
    Action(KeyAction),
    /// Mode change requested.
    ModeChange(Mode),
    /// Pending: need more keys (e.g. operator-pending).
    Pending,
    /// No-op: key not recognized.
    Noop,
}

/// Stateful mode dispatcher.
pub struct ModeDispatcher {
    pub count: Option<usize>,
    pub register: Option<char>,
    pub pending_operator: Option<Operator>,
    pub pending_keys: Vec<Key>,
    pub window_pending: bool,
}

impl ModeDispatcher {
    pub fn new() -> Self {
        Self {
            count: None,
            register: None,
            pending_operator: None,
            pending_keys: Vec::new(),
            window_pending: false,
        }
    }

    /// Dispatch a key in the given mode.
    pub fn dispatch(&mut self, key: &Key, mode: &Mode) -> DispatchResult {
        // Handle Ctrl-w pending state (window commands)
        if self.window_pending {
            self.window_pending = false;
            return dispatch_window_key(key);
        }

        match mode {
            Mode::Normal => super::normal::dispatch_normal(self, key),
            Mode::Insert => super::insert::dispatch_insert(key),
            Mode::Command(_) => super::command::dispatch_command(key),
            Mode::OperatorPending(op) => {
                let op = *op;
                self.dispatch_operator_pending(key, &op)
            }
            Mode::Visual(vk) => super::visual::dispatch_visual(self, key, vk),
            Mode::Replace => dispatch_replace(key),
            Mode::TerminalInsert => dispatch_terminal_insert(key),
            Mode::InsertNormal => super::normal::dispatch_normal(self, key),
        }
    }

    fn dispatch_operator_pending(&mut self, key: &Key, op: &Operator) -> DispatchResult {
        if key.code == KeyCode::Esc {
            self.pending_operator = None;
            return DispatchResult::ModeChange(Mode::Normal);
        }
        let is_double = matches!(
            (&op, &key.code),
            (Operator::Delete, KeyCode::Char('d'))
                | (Operator::Yank, KeyCode::Char('y'))
                | (Operator::Change, KeyCode::Char('c'))
                | (Operator::Indent, KeyCode::Char('>'))
                | (Operator::Dedent, KeyCode::Char('<'))
                | (Operator::Reindent, KeyCode::Char('='))
        );
        if is_double {
            let count = self.take_count();
            self.pending_operator = None;
            return DispatchResult::Action(KeyAction::OperatorMotion {
                op: *op,
                motion: MotionAction::LineEnd,
                count,
            });
        }
        if let Some(motion) = key_to_motion(key) {
            let count = self.take_count();
            self.pending_operator = None;
            DispatchResult::Action(KeyAction::OperatorMotion {
                op: *op,
                motion,
                count,
            })
        } else {
            DispatchResult::Noop
        }
    }

    pub fn take_count(&mut self) -> usize {
        self.count.take().unwrap_or(1)
    }

    pub fn accumulate_count(&mut self, digit: u32) {
        let current = self.count.unwrap_or(0);
        self.count = Some(current * 10 + digit as usize);
    }
}

impl Default for ModeDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

fn dispatch_replace(key: &Key) -> DispatchResult {
    match &key.code {
        KeyCode::Esc => DispatchResult::ModeChange(Mode::Normal),
        KeyCode::Backspace => DispatchResult::Action(KeyAction::ReplaceBackspace),
        KeyCode::Char(c) => DispatchResult::Action(KeyAction::ReplaceChar(*c)),
        KeyCode::Enter => DispatchResult::Action(KeyAction::ReplaceChar('\n')),
        _ => DispatchResult::Noop,
    }
}

fn dispatch_terminal_insert(key: &Key) -> DispatchResult {
    if key.modifiers.contains(KeyModifiers::CTRL) {
        if let KeyCode::Char('\\') = &key.code {
            return DispatchResult::Pending;
        }
        if let KeyCode::Char('n') = &key.code {
            return DispatchResult::ModeChange(Mode::Normal);
        }
    }
    DispatchResult::Noop
}

/// Dispatch the second key after Ctrl-w window prefix.
fn dispatch_window_key(key: &Key) -> DispatchResult {
    if key.modifiers.is_empty() || key.modifiers.contains(KeyModifiers::CTRL) {
        match &key.code {
            KeyCode::Char('w') => DispatchResult::Action(KeyAction::WindowNext),
            KeyCode::Char('W') => DispatchResult::Action(KeyAction::WindowPrev),
            KeyCode::Char('s') => DispatchResult::Action(KeyAction::WindowSplitH),
            KeyCode::Char('v') => DispatchResult::Action(KeyAction::WindowSplitV),
            KeyCode::Char('c') | KeyCode::Char('q') => {
                DispatchResult::Action(KeyAction::WindowClose)
            }
            KeyCode::Char('h') | KeyCode::Left => {
                DispatchResult::Action(KeyAction::WindowFocusLeft)
            }
            KeyCode::Char('j') | KeyCode::Down => {
                DispatchResult::Action(KeyAction::WindowFocusDown)
            }
            KeyCode::Char('k') | KeyCode::Up => DispatchResult::Action(KeyAction::WindowFocusUp),
            KeyCode::Char('l') | KeyCode::Right => {
                DispatchResult::Action(KeyAction::WindowFocusRight)
            }
            KeyCode::Char('n') => DispatchResult::Action(KeyAction::WindowSplitH),
            KeyCode::Esc => DispatchResult::Noop,
            _ => DispatchResult::Noop,
        }
    } else {
        DispatchResult::Noop
    }
}
