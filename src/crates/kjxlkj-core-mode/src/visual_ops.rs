//! Visual mode: operator and command dispatch.

use kjxlkj_core_types::{
    Action, Key, KeyCode, KeyModifiers, Operator,
    VisualKind,
};

use crate::visual::VisualModeState;

impl VisualModeState {
    /// Process operator/command keys in visual mode.
    pub(crate) fn dispatch_visual_command(
        &mut self,
        key: &Key,
    ) -> Option<Action> {
        let action = match (&key.code, key.modifiers) {
            // Swap anchor/cursor
            (KeyCode::Char('o'), KeyModifiers::NONE) => {
                Action::Nop
            }

            // Switch visual kind
            (KeyCode::Char('v'), KeyModifiers::NONE) => {
                self.toggle_kind(VisualKind::Char);
                self.reset_count();
                return Some(Action::EnterVisual(
                    VisualKind::Char,
                ));
            }
            (KeyCode::Char('V'), KeyModifiers::NONE) => {
                self.toggle_kind(VisualKind::Line);
                self.reset_count();
                return Some(Action::EnterVisual(
                    VisualKind::Line,
                ));
            }
            (KeyCode::Char('v'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                self.toggle_kind(VisualKind::Block);
                self.reset_count();
                return Some(Action::EnterVisual(
                    VisualKind::Block,
                ));
            }

            // Operators on selection
            (KeyCode::Char('d'), KeyModifiers::NONE)
            | (KeyCode::Char('x'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Delete(
                    kjxlkj_core_types::Motion::Right,
                    1,
                ));
            }
            (KeyCode::Char('c'), KeyModifiers::NONE)
            | (KeyCode::Char('s'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Change(
                    kjxlkj_core_types::Motion::Right,
                    1,
                ));
            }
            (KeyCode::Char('y'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Yank(
                    kjxlkj_core_types::Motion::Right,
                    1,
                ));
            }
            (KeyCode::Char('>'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(
                    Operator::Indent,
                    1,
                ));
            }
            (KeyCode::Char('<'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(
                    Operator::Dedent,
                    1,
                ));
            }
            (KeyCode::Char('~'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::ToggleCaseChar);
            }
            (KeyCode::Char('U'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(
                    Operator::Uppercase,
                    1,
                ));
            }
            (KeyCode::Char('u'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(
                    Operator::Lowercase,
                    1,
                ));
            }

            // Put
            (KeyCode::Char('p'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Put(false));
            }
            (KeyCode::Char('P'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Put(true));
            }

            // Join
            (KeyCode::Char('J'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::JoinLines);
            }

            // Enter command mode with selection range
            (KeyCode::Char(':'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::EnterCommand(
                    kjxlkj_core_types::ActionCommandKind::Ex,
                ));
            }

            _ => {
                self.reset_count();
                return Some(Action::Nop);
            }
        };

        self.reset_count();
        Some(action)
    }
}
