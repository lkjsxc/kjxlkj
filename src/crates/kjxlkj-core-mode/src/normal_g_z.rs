//! Normal mode: g, z, and Ctrl-w prefix handlers.

use kjxlkj_core_types::{
    Action, Direction, Key, KeyCode, KeyModifiers,
    Motion, Operator,
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

    pub(crate) fn process_ctrl_w_key(
        &mut self,
        key: &Key,
    ) -> Option<Action> {
        self.ctrl_w_pending = false;
        let action = match (&key.code, key.modifiers) {
            (KeyCode::Char('h'), KeyModifiers::NONE)
            | (KeyCode::Left, _) => {
                Action::FocusWindow(Direction::Left)
            }
            (KeyCode::Char('j'), KeyModifiers::NONE)
            | (KeyCode::Down, _) => {
                Action::FocusWindow(Direction::Down)
            }
            (KeyCode::Char('k'), KeyModifiers::NONE)
            | (KeyCode::Up, _) => {
                Action::FocusWindow(Direction::Up)
            }
            (KeyCode::Char('l'), KeyModifiers::NONE)
            | (KeyCode::Right, _) => {
                Action::FocusWindow(Direction::Right)
            }
            (KeyCode::Char('w'), _) => {
                Action::CycleWindow
            }
            (KeyCode::Char('c'), KeyModifiers::NONE)
            | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Action::CloseWindow
            }
            (KeyCode::Char('s'), KeyModifiers::NONE) => {
                Action::SplitHorizontal
            }
            (KeyCode::Char('v'), KeyModifiers::NONE) => {
                Action::SplitVertical
            }
            (KeyCode::Char('='), KeyModifiers::NONE) => {
                Action::EqualizeWindows
            }
            (KeyCode::Char('H'), KeyModifiers::NONE) => {
                Action::MoveWindow(Direction::Left)
            }
            (KeyCode::Char('J'), KeyModifiers::NONE) => {
                Action::MoveWindow(Direction::Down)
            }
            (KeyCode::Char('K'), KeyModifiers::NONE) => {
                Action::MoveWindow(Direction::Up)
            }
            (KeyCode::Char('L'), KeyModifiers::NONE) => {
                Action::MoveWindow(Direction::Right)
            }
            (KeyCode::Char('r'), KeyModifiers::NONE) => {
                Action::RotateWindows(true)
            }
            (KeyCode::Char('R'), KeyModifiers::NONE) => {
                Action::RotateWindows(false)
            }
            (KeyCode::Char('o'), KeyModifiers::NONE) => {
                Action::ZoomWindow
            }
            _ => Action::Nop,
        };
        self.reset();
        Some(action)
    }
}
