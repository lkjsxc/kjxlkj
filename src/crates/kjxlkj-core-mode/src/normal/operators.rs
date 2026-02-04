//! Operator and motion handlers.

use crate::Intent;
use kjxlkj_core_edit::{Motion, Operator, OperatorKind};

use super::motions::key_to_motion;
use super::state::{AwaitingChar, NormalModeState};

impl NormalModeState {
    /// Handle operator+motion combinations.
    pub fn handle_operator_motion(
        &mut self,
        op: OperatorKind,
        key: char,
        count: usize,
    ) -> Intent {
        self.reset();
        // Check for line-wise operator (dd, yy, cc, etc.)
        let is_linewise = match (op, key) {
            (OperatorKind::Delete, 'd') => true,
            (OperatorKind::Yank, 'y') => true,
            (OperatorKind::Change, 'c') => true,
            (OperatorKind::Indent, '>') => true,
            (OperatorKind::Outdent, '<') => true,
            _ => false,
        };

        if is_linewise {
            return Intent::Execute(Operator::line(op, count));
        }

        // Check for text objects
        if key == 'i' || key == 'a' {
            self.pending_operator = Some(op);
            self.awaiting_char = Some(if key == 'i' {
                AwaitingChar::FindForward // Placeholder - handle text object
            } else {
                AwaitingChar::FindBackward
            });
            return Intent::None;
        }

        // Motion
        if let Some(motion_kind) = key_to_motion(key) {
            let motion = Motion::new(motion_kind).with_count(count);
            Intent::Execute(Operator::with_motion(op, motion))
        } else {
            Intent::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn dd_is_linewise() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'd', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn yy_is_linewise() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Yank, 'y', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn cc_is_linewise() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Change, 'c', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn dw_uses_motion() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'w', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn unknown_motion_none() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'z', 1);
        assert_eq!(intent, Intent::None);
    }

    #[test]
    fn indent_linewise() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Indent, '>', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn outdent_linewise() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Outdent, '<', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn delete_with_e_motion() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'e', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn yank_with_w_motion() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Yank, 'w', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn delete_with_b_motion() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'b', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn change_with_e_motion() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Change, 'e', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn yank_with_b_motion() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Yank, 'b', 1);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn delete_with_count() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'w', 3);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn yank_line_with_count() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Yank, 'y', 5);
        assert!(matches!(intent, Intent::Execute(_)));
    }

    #[test]
    fn inner_text_object_awaits() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'i', 1);
        assert_eq!(intent, Intent::None);
    }

    #[test]
    fn around_text_object_awaits() {
        let mut state = NormalModeState::new();
        let intent = state.handle_operator_motion(OperatorKind::Delete, 'a', 1);
        assert_eq!(intent, Intent::None);
    }
}
