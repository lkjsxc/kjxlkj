//! Normal mode key dispatch tables.
//!
//! Separated from normal.rs to keep files under 200
//! lines per policy.

use kjxlkj_core_types::{
    Action, InsertPosition, Key, KeyCode, KeyModifiers,
    Motion, Operator, VisualKind,
};

use crate::normal::NormalModeState;

impl NormalModeState {
    pub(crate) fn dispatch_key(
        &mut self,
        key: &Key,
    ) -> Option<Action> {
        let count = self.effective_count();

        let action = match (&key.code, key.modifiers) {
            // --- Motions ---
            (KeyCode::Char('h'), KeyModifiers::NONE)
            | (KeyCode::Left, _) => {
                Action::MoveCursor(Motion::Left, count)
            }
            (KeyCode::Char('l'), KeyModifiers::NONE)
            | (KeyCode::Right, _) => {
                Action::MoveCursor(Motion::Right, count)
            }
            (KeyCode::Char('j'), KeyModifiers::NONE)
            | (KeyCode::Down, _) => {
                Action::MoveCursor(Motion::Down, count)
            }
            (KeyCode::Char('k'), KeyModifiers::NONE)
            | (KeyCode::Up, _) => {
                Action::MoveCursor(Motion::Up, count)
            }
            (KeyCode::Char('w'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::WordForward,
                    count,
                )
            }
            (KeyCode::Char('W'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::WordForwardBig,
                    count,
                )
            }
            (KeyCode::Char('b'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::WordBackward,
                    count,
                )
            }
            (KeyCode::Char('B'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::WordBackwardBig,
                    count,
                )
            }
            (KeyCode::Char('e'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::WordEndForward,
                    count,
                )
            }
            (KeyCode::Char('E'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::WordEndForwardBig,
                    count,
                )
            }
            (KeyCode::Char('0'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::LineStart, 1)
            }
            (KeyCode::Char('^'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::FirstNonBlank,
                    1,
                )
            }
            (KeyCode::Char('$'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::LineEnd, 1)
            }
            (KeyCode::Char('G'), KeyModifiers::NONE) => {
                if self.count.is_some() {
                    Action::MoveCursor(
                        Motion::GotoLine(
                            count as usize - 1,
                        ),
                        1,
                    )
                } else {
                    Action::MoveCursor(
                        Motion::GotoLastLine,
                        1,
                    )
                }
            }
            (KeyCode::Char('{'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::ParagraphBackward,
                    count,
                )
            }
            (KeyCode::Char('}'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::ParagraphForward,
                    count,
                )
            }
            (KeyCode::Char('+'), KeyModifiers::NONE)
            | (KeyCode::Enter, KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::NextLineFirstNonBlank,
                    count,
                )
            }
            (KeyCode::Char('-'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::PrevLineFirstNonBlank,
                    count,
                )
            }
            _ => {
                return self.dispatch_key_commands(
                    key, count,
                );
            }
        };

        self.reset();
        Some(action)
    }

    fn dispatch_key_commands(
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
