//! Operator and motion handlers.

use crate::Intent;
use kjxlkj_core_edit::{Motion, MotionKind, Operator, OperatorKind};

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
