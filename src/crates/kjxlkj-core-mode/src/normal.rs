//! Normal mode state: count prefix, register prefix,
//! and key dispatch.

use kjxlkj_core_types::{
    Action, Key, KeyCode, RegisterName,
};

/// State maintained during Normal mode key processing.
#[derive(Debug, Default)]
pub struct NormalModeState {
    /// Accumulated count prefix.
    pub(crate) count: Option<u32>,
    /// Pending register prefix.
    pub(crate) register: Option<RegisterName>,
    /// Waiting for register name character.
    pub(crate) register_pending: bool,
    /// Pending g-prefix.
    pub(crate) g_pending: bool,
    /// Pending z-prefix.
    pub(crate) z_pending: bool,
    /// Pending bracket prefix.
    pub(crate) bracket_pending: Option<char>,
    /// Pending mark command.
    pub(crate) mark_pending: Option<MarkCommand>,
    /// Pending r (replace char).
    pub(crate) replace_char_pending: bool,
    /// Pending f/F/t/T (waiting for char).
    pub(crate) find_char_pending: Option<FindCharPending>,
    /// Pending Ctrl-w window command.
    pub(crate) ctrl_w_pending: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MarkCommand {
    Set,
    JumpExact,
    JumpLine,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FindCharPending {
    Forward,
    Backward,
    TillForward,
    TillBackward,
}

impl NormalModeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset all pending state.
    pub fn reset(&mut self) {
        self.count = None;
        self.register = None;
        self.register_pending = false;
        self.g_pending = false;
        self.z_pending = false;
        self.bracket_pending = None;
        self.mark_pending = None;
        self.replace_char_pending = false;
        self.find_char_pending = None;
        self.ctrl_w_pending = false;
    }

    /// Get the effective count (default 1).
    pub fn effective_count(&self) -> u32 {
        self.count.unwrap_or(1)
    }

    /// Get the target register.
    pub fn target_register(&self) -> RegisterName {
        self.register
            .unwrap_or(RegisterName::Unnamed)
    }

    /// Process a key event in Normal mode.
    pub fn process_key(
        &mut self,
        key: &Key,
    ) -> Option<Action> {
        // Handle pending sub-states first.
        if self.replace_char_pending {
            self.replace_char_pending = false;
            if let KeyCode::Char(c) = key.code {
                let action = Action::ReplaceChar(c);
                self.reset();
                return Some(action);
            }
            self.reset();
            return Some(Action::Nop);
        }

        if let Some(mark_cmd) = self.mark_pending {
            self.mark_pending = None;
            if let KeyCode::Char(c) = key.code {
                let action = match mark_cmd {
                    MarkCommand::Set => {
                        Action::SetMark(c)
                    }
                    MarkCommand::JumpExact => {
                        Action::JumpToMark(c)
                    }
                    MarkCommand::JumpLine => {
                        Action::JumpToMarkLine(c)
                    }
                };
                self.reset();
                return Some(action);
            }
            self.reset();
            return Some(Action::Nop);
        }

        if let Some(fcp) = self.find_char_pending {
            self.find_char_pending = None;
            if let KeyCode::Char(c) = key.code {
                use kjxlkj_core_types::Motion;
                let motion = match fcp {
                    FindCharPending::Forward => {
                        Motion::FindCharForward(c)
                    }
                    FindCharPending::Backward => {
                        Motion::FindCharBackward(c)
                    }
                    FindCharPending::TillForward => {
                        Motion::TillCharForward(c)
                    }
                    FindCharPending::TillBackward => {
                        Motion::TillCharBackward(c)
                    }
                };
                let count = self.effective_count();
                self.reset();
                return Some(
                    Action::MoveCursor(motion, count),
                );
            }
            self.reset();
            return Some(Action::Nop);
        }

        if self.ctrl_w_pending {
            return self.process_ctrl_w_key(key);
        }

        if self.register_pending {
            self.register_pending = false;
            if let KeyCode::Char(c) = key.code {
                self.register =
                    RegisterName::from_char(c);
            }
            return None;
        }

        if self.g_pending {
            return self.process_g_key(key);
        }
        if self.z_pending {
            return self.process_z_key(key);
        }

        // Count prefix.
        if let Some(digit) = key.digit_value() {
            if digit > 0 || self.count.is_some() {
                let current = self.count.unwrap_or(0);
                self.count = Some(current * 10 + digit);
                return None;
            }
        }

        // Standard key dispatch (in normal_keys.rs).
        self.dispatch_key(key)
    }
}
