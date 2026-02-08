//! Normal mode: single-key commands and prefix handlers.
//!
//! Separated from normal.rs to keep files under 200
//! lines per policy.

use kjxlkj_core_types::{
    Action, Key, KeyCode, KeyModifiers, Motion, Operator,
};

use crate::normal::{MarkCommand, NormalModeState};

impl NormalModeState {
    pub(crate) fn dispatch_key_single(
        &mut self,
        key: &Key,
        count: u32,
    ) -> Option<Action> {
        let action = match (&key.code, key.modifiers) {
            (KeyCode::Char('x'), KeyModifiers::NONE) => {
                Action::DeleteCharForward
            }
            (KeyCode::Char('X'), KeyModifiers::NONE) => {
                Action::DeleteCharBackward
            }
            (KeyCode::Char('s'), KeyModifiers::NONE) => {
                Action::SubstituteChar
            }
            (KeyCode::Char('S'), KeyModifiers::NONE) => {
                Action::SubstituteLine
            }
            (KeyCode::Char('C'), KeyModifiers::NONE) => {
                Action::ChangeToEnd
            }
            (KeyCode::Char('J'), KeyModifiers::NONE) => {
                Action::JoinLines
            }
            (KeyCode::Char('~'), KeyModifiers::NONE) => {
                Action::ToggleCaseChar
            }
            (KeyCode::Char('.'), KeyModifiers::NONE) => {
                Action::DotRepeat
            }
            (KeyCode::Char('u'), KeyModifiers::NONE) => {
                Action::Undo
            }
            (KeyCode::Char('r'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                Action::Redo
            }
            (KeyCode::Char('p'), KeyModifiers::NONE) => {
                Action::Put(false)
            }
            (KeyCode::Char('P'), KeyModifiers::NONE) => {
                Action::Put(true)
            }
            (KeyCode::Char('n'), KeyModifiers::NONE) => {
                Action::NextMatch
            }
            (KeyCode::Char('N'), KeyModifiers::NONE) => {
                Action::PrevMatch
            }
            (KeyCode::Char('*'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::StarForward,
                    1,
                )
            }
            (KeyCode::Char('#'), KeyModifiers::NONE) => {
                Action::MoveCursor(
                    Motion::StarBackward,
                    1,
                )
            }

            // Replace char
            (KeyCode::Char('r'), KeyModifiers::NONE) => {
                self.replace_char_pending = true;
                return None;
            }
            // Replace mode
            (KeyCode::Char('R'), KeyModifiers::NONE) => {
                Action::EnterReplace
            }

            // Command mode
            (KeyCode::Char(':'), KeyModifiers::NONE) => {
                Action::EnterCommand(
                    kjxlkj_core_types::ActionCommandKind::Ex,
                )
            }
            (KeyCode::Char('/'), KeyModifiers::NONE) => {
                Action::EnterCommand(
                    kjxlkj_core_types::ActionCommandKind::SearchForward,
                )
            }
            (KeyCode::Char('?'), KeyModifiers::NONE) => {
                Action::EnterCommand(
                    kjxlkj_core_types::ActionCommandKind::SearchBackward,
                )
            }

            // Register prefix
            (KeyCode::Char('"'), KeyModifiers::NONE) => {
                self.register_pending = true;
                return None;
            }

            // g-prefix
            (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.g_pending = true;
                return None;
            }
            // z-prefix
            (KeyCode::Char('z'), KeyModifiers::NONE) => {
                self.z_pending = true;
                return None;
            }

            // Marks
            (KeyCode::Char('m'), KeyModifiers::NONE) => {
                self.mark_pending = Some(MarkCommand::Set);
                return None;
            }
            (KeyCode::Char('`'), KeyModifiers::NONE) => {
                self.mark_pending =
                    Some(MarkCommand::JumpExact);
                return None;
            }
            (KeyCode::Char('\''), KeyModifiers::NONE) => {
                self.mark_pending =
                    Some(MarkCommand::JumpLine);
                return None;
            }

            // Macros
            (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Action::RecordMacro('\0')
            }
            (KeyCode::Char('@'), KeyModifiers::NONE) => {
                Action::PlayMacro('\0', count)
            }

            // Window commands
            (KeyCode::Char('w'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                Action::CycleWindow
            }
            // Ctrl-^ alternate file
            (KeyCode::Char('^'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                Action::AlternateFile
            }

            // ZZ / ZQ — simplified: Nop
            (KeyCode::Char('Z'), KeyModifiers::NONE) => {
                Action::Nop
            }

            // Increment/decrement
            (KeyCode::Char('a'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                Action::Increment(count as i64)
            }
            (KeyCode::Char('x'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                Action::Increment(-(count as i64))
            }

            _ => Action::Nop,
        };

        self.reset();
        Some(action)
    }

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
            KeyCode::Char('J') => Action::JoinLinesNoSpace,
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
