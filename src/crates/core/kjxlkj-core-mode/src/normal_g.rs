//! Normal mode `g`-prefix key dispatch.
//!
//! See /docs/spec/modes/normal.md § g-prefix commands.
//! Includes g-motions (gg, g_, ge, gE) and
//! g-operators (gu, gU, g~, gq) per
//! /docs/spec/editing/operators/g-operators.md.

use kjxlkj_core_types::{Action, Key, Mode, Motion, Operator};

use crate::pending::PendingState;

/// Handle the second key after `g` was pressed.
pub(crate) fn handle_g_key(
    key: &Key,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    pending.partial = crate::pending::PartialKey::None;
    match key {
        // gg → go to first line (or line N if count).
        Key::Char('g') => {
            let a = Action::Motion(Motion::GotoFirstLine);
            pending.clear();
            (a, None)
        }
        // g_ → last non-blank.
        Key::Char('_') => {
            pending.clear();
            (Action::Motion(Motion::LastNonBlank), None)
        }
        // ge → backward to end of previous word.
        Key::Char('e') => {
            pending.clear();
            (Action::Motion(Motion::WordEndBackward), None)
        }
        // gE → backward to end of previous WORD.
        Key::Char('E') => {
            pending.clear();
            (Action::Motion(Motion::BigWordEndBackward), None)
        }
        // gu → lowercase operator.
        Key::Char('u') => {
            pending.clear();
            (
                Action::Noop,
                Some(Mode::OperatorPending(Operator::Lowercase)),
            )
        }
        // gU → uppercase operator.
        Key::Char('U') => {
            pending.clear();
            (
                Action::Noop,
                Some(Mode::OperatorPending(Operator::Uppercase)),
            )
        }
        // g~ → toggle case operator.
        Key::Char('~') => {
            pending.clear();
            (
                Action::Noop,
                Some(Mode::OperatorPending(
                    Operator::ToggleCase,
                )),
            )
        }
        // gq → format operator.
        Key::Char('q') => {
            pending.clear();
            (
                Action::Noop,
                Some(Mode::OperatorPending(Operator::Format)),
            )
        }
        // gJ → join lines without space.
        Key::Char('J') => {
            pending.clear();
            (Action::JoinLinesNoSpace, None)
        }
        _ => {
            pending.clear();
            (Action::Noop, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gg_goes_to_first_line() {
        let mut ps = PendingState::default();
        let (action, _) =
            handle_g_key(&Key::Char('g'), &mut ps);
        assert_eq!(
            action,
            Action::Motion(Motion::GotoFirstLine)
        );
    }

    #[test]
    fn g_underscore_last_nonblank() {
        let mut ps = PendingState::default();
        let (action, _) =
            handle_g_key(&Key::Char('_'), &mut ps);
        assert_eq!(
            action,
            Action::Motion(Motion::LastNonBlank)
        );
    }

    #[test]
    fn ge_word_end_backward() {
        let mut ps = PendingState::default();
        let (action, _) =
            handle_g_key(&Key::Char('e'), &mut ps);
        assert_eq!(
            action,
            Action::Motion(Motion::WordEndBackward)
        );
    }

    #[test]
    fn gu_enters_lowercase_op() {
        let mut ps = PendingState::default();
        let (_, mode) =
            handle_g_key(&Key::Char('u'), &mut ps);
        assert_eq!(
            mode,
            Some(Mode::OperatorPending(Operator::Lowercase))
        );
    }

    #[test]
    fn g_upper_u_enters_uppercase_op() {
        let mut ps = PendingState::default();
        let (_, mode) =
            handle_g_key(&Key::Char('U'), &mut ps);
        assert_eq!(
            mode,
            Some(Mode::OperatorPending(Operator::Uppercase))
        );
    }

    #[test]
    fn g_tilde_enters_toggle_case_op() {
        let mut ps = PendingState::default();
        let (_, mode) =
            handle_g_key(&Key::Char('~'), &mut ps);
        assert_eq!(
            mode,
            Some(Mode::OperatorPending(
                Operator::ToggleCase
            ))
        );
    }

    #[test]
    fn gq_enters_format_op() {
        let mut ps = PendingState::default();
        let (_, mode) =
            handle_g_key(&Key::Char('q'), &mut ps);
        assert_eq!(
            mode,
            Some(Mode::OperatorPending(Operator::Format))
        );
    }

    #[test]
    fn g_j_join_no_space() {
        let mut ps = PendingState::default();
        let (action, _) =
            handle_g_key(&Key::Char('J'), &mut ps);
        assert_eq!(action, Action::JoinLinesNoSpace);
    }
}

