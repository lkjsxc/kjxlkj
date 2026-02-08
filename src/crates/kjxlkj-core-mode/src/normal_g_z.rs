//! Normal mode: g and z prefix command handlers.

use kjxlkj_core_types::{
    Action, Key, KeyCode, Motion, Operator,
};

use crate::normal::NormalModeState;

impl NormalModeState {
    pub(crate) fn process_g_key(
        &mut self,
        key: &Key,
    ) -> Option<Action> {
        self.g_pending = false;
        let count = self.effective_count();
        let action = match &key.code {
            KeyCode::Char('g') => {
                if self.count.is_some() {
                    Action::MoveCursor(
                        Motion::GotoLine(
                            count as usize - 1,
                        ),
                        1,
                    )
                } else {
                    Action::MoveCursor(
                        Motion::GotoFirstLine,
                        1,
                    )
                }
            }
            KeyCode::Char('J') => {
                Action::JoinLinesNoSpace
            }
            KeyCode::Char('~') => {
                Action::EnterOperatorPending(
                    Operator::ToggleCase,
                )
            }
            KeyCode::Char('u') => {
                Action::EnterOperatorPending(
                    Operator::Lowercase,
                )
            }
            KeyCode::Char('U') => {
                Action::EnterOperatorPending(
                    Operator::Uppercase,
                )
            }
            KeyCode::Char('q') => {
                Action::EnterOperatorPending(
                    Operator::Format,
                )
            }
            KeyCode::Char('_') => {
                Action::MoveCursor(
                    Motion::LastNonBlank,
                    1,
                )
            }
            _ => Action::Nop,
        };
        self.reset();
        Some(action)
    }

    pub(crate) fn process_z_key(
        &mut self,
        key: &Key,
    ) -> Option<Action> {
        self.z_pending = false;
        let action = match &key.code {
            KeyCode::Char('z') => {
                Action::MoveCursor(
                    Motion::ScreenMiddle,
                    1,
                )
            }
            KeyCode::Char('t') => {
                Action::MoveCursor(Motion::ScreenTop, 1)
            }
            KeyCode::Char('b') => {
                Action::MoveCursor(
                    Motion::ScreenBottom,
                    1,
                )
            }
            _ => Action::Nop,
        };
        self.reset();
        Some(action)
    }
}
