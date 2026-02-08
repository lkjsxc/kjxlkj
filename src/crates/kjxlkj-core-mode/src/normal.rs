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
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MarkCommand {
    Set,
    JumpExact,
    JumpLine,
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

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{
        InsertPosition, Motion, Operator,
    };

    #[test]
    fn count_accumulation() {
        let mut s = NormalModeState::new();
        assert!(s.process_key(&Key::char('3')).is_none());
        let a = s.process_key(&Key::char('j'));
        assert!(matches!(
            a,
            Some(Action::MoveCursor(Motion::Down, 3))
        ));
    }

    #[test]
    fn insert_entry() {
        let mut s = NormalModeState::new();
        let a = s.process_key(&Key::char('i'));
        assert!(matches!(
            a,
            Some(Action::EnterInsert(
                InsertPosition::BeforeCursor
            ))
        ));
    }

    #[test]
    fn operator_pending() {
        let mut s = NormalModeState::new();
        let a = s.process_key(&Key::char('d'));
        assert!(matches!(
            a,
            Some(Action::EnterOperatorPending(
                Operator::Delete
            ))
        ));
    }

    #[test]
    fn gg_motion() {
        let mut s = NormalModeState::new();
        assert!(s.process_key(&Key::char('g')).is_none());
        let a = s.process_key(&Key::char('g'));
        assert!(matches!(
            a,
            Some(Action::MoveCursor(
                Motion::GotoFirstLine,
                1
            ))
        ));
    }
}
