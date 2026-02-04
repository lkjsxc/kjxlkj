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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn normal_mode_state_new() {
        let state = NormalModeState::new();
        assert!(state.count.is_none());
    }

    #[test]
    fn normal_mode_state_default() {
        let state = NormalModeState::default();
        assert!(state.pending_operator.is_none());
    }

    #[test]
    fn normal_mode_get_count_default() {
        let state = NormalModeState::new();
        assert_eq!(state.get_count(), 1);
    }

    #[test]
    fn normal_mode_reset_clears() {
        let mut state = NormalModeState::new();
        state.count = Some(5);
        state.reset();
        assert!(state.count.is_none());
    }

    #[test]
    fn awaiting_char_replace_debug() {
        let _ = format!("{:?}", AwaitingChar::Replace);
    }

    #[test]
    fn awaiting_char_find_forward() {
        let _ = format!("{:?}", AwaitingChar::FindForward);
    }

    #[test]
    fn awaiting_char_find_backward() {
        let _ = format!("{:?}", AwaitingChar::FindBackward);
    }

    #[test]
    fn awaiting_char_till_forward() {
        let _ = format!("{:?}", AwaitingChar::TillForward);
    }

    #[test]
    fn awaiting_char_mark() {
        let _ = format!("{:?}", AwaitingChar::Mark);
    }

    #[test]
    fn awaiting_char_register() {
        let _ = format!("{:?}", AwaitingChar::Register);
    }

    #[test]
    fn normal_mode_get_count_with_value() {
        let mut state = NormalModeState::new();
        state.count = Some(10);
        assert_eq!(state.get_count(), 10);
    }

    #[test]
    fn normal_mode_pending_register_none() {
        let state = NormalModeState::new();
        assert!(state.pending_register.is_none());
    }

    #[test]
    fn normal_mode_last_find_none() {
        let state = NormalModeState::new();
        assert!(state.last_find.is_none());
    }

    #[test]
    fn normal_mode_awaiting_char_none() {
        let state = NormalModeState::new();
        assert!(state.awaiting_char.is_none());
    }

    #[test]
    fn awaiting_char_till_backward() {
        let _ = format!("{:?}", AwaitingChar::TillBackward);
    }

    #[test]
    fn awaiting_char_jump_mark() {
        let _ = format!("{:?}", AwaitingChar::JumpMark);
    }

    #[test]
    fn awaiting_char_jump_mark_line() {
        let _ = format!("{:?}", AwaitingChar::JumpMarkLine);
    }

    #[test]
    fn awaiting_char_macro_record() {
        let _ = format!("{:?}", AwaitingChar::MacroRecord);
    }

    #[test]
    fn awaiting_char_macro_play() {
        let _ = format!("{:?}", AwaitingChar::MacroPlay);
    }

    #[test]
    fn awaiting_char_clone() {
        let aw = AwaitingChar::Replace;
        let aw2 = aw;
        let _ = format!("{:?}", aw2);
    }

    #[test]
    fn normal_mode_debug() {
        let state = NormalModeState::new();
        let debug = format!("{:?}", state);
        assert!(debug.contains("NormalModeState"));
    }

    #[test]
    fn normal_mode_state_clone() {
        let state = NormalModeState::new();
        let state2 = state.clone();
        assert!(state2.count.is_none());
    }

    #[test]
    fn normal_mode_reset_resets_op() {
        let mut state = NormalModeState::new();
        state.pending_operator = Some(kjxlkj_core_edit::OperatorKind::Delete);
        state.reset();
        assert!(state.pending_operator.is_none());
    }
}
