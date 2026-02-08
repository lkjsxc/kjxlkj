//! Normal mode: insert/visual/operator entry dispatch.

use kjxlkj_core_types::{
    Action, InsertPosition, Key, KeyCode, KeyModifiers,
    Operator, VisualKind,
};

use crate::normal::NormalModeState;

impl NormalModeState {
    pub(crate) fn dispatch_key_commands(
        &mut self,
        key: &Key,
        count: u32,
    ) -> Option<Action> {
        let action = match (&key.code, key.modifiers) {
            // --- Insert mode entry ---
            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                Action::EnterInsert(
                    InsertPosition::BeforeCursor,
                )
            }
            (KeyCode::Char('a'), KeyModifiers::NONE) => {
                Action::EnterInsert(
                    InsertPosition::AfterCursor,
                )
            }
            (KeyCode::Char('I'), KeyModifiers::NONE) => {
                Action::EnterInsert(
                    InsertPosition::FirstNonBlank,
                )
            }
            (KeyCode::Char('A'), KeyModifiers::NONE) => {
                Action::EnterInsert(
                    InsertPosition::EndOfLine,
                )
            }
            (KeyCode::Char('o'), KeyModifiers::NONE) => {
                Action::EnterInsert(
                    InsertPosition::NewLineBelow,
                )
            }
            (KeyCode::Char('O'), KeyModifiers::NONE) => {
                Action::EnterInsert(
                    InsertPosition::NewLineAbove,
                )
            }

            // --- Visual mode ---
            (KeyCode::Char('v'), KeyModifiers::NONE) => {
                Action::EnterVisual(VisualKind::Char)
            }
            (KeyCode::Char('V'), KeyModifiers::NONE) => {
                Action::EnterVisual(VisualKind::Line)
            }
            (KeyCode::Char('v'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                Action::EnterVisual(VisualKind::Block)
            }

            // --- Operators ---
            (KeyCode::Char('d'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(
                    Operator::Delete,
                )
            }
            (KeyCode::Char('c'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(
                    Operator::Change,
                )
            }
            (KeyCode::Char('y'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(
                    Operator::Yank,
                )
            }
            (KeyCode::Char('>'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(
                    Operator::Indent,
                )
            }
            (KeyCode::Char('<'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(
                    Operator::Dedent,
                )
            }
            (KeyCode::Char('='), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(
                    Operator::Reindent,
                )
            }

            _ => {
                return self.dispatch_key_single(
                    key, count,
                );
            }
        };

        self.reset();
        Some(action)
    }
}
