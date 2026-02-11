//! Normal mode `g`-prefix key dispatch.
//!
//! See /docs/spec/modes/normal.md § g-prefix commands.

use kjxlkj_core_types::{Action, Key, Mode, Motion};

use crate::pending::PendingState;

/// Handle the second key after `g` was pressed.
pub(crate) fn handle_g_key(
    key: &Key,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    pending.clear();
    match key {
        // gg → go to first line (or line N if count).
        Key::Char('g') => {
            (Action::Motion(Motion::GotoFirstLine), None)
        }
        // g_ → last non-blank.
        Key::Char('_') => {
            (Action::Motion(Motion::LastNonBlank), None)
        }
        // ge → backward to end of previous word.
        Key::Char('e') => {
            (Action::Motion(Motion::WordEndBackward), None)
        }
        // gE → backward to end of previous WORD.
        Key::Char('E') => {
            (Action::Motion(Motion::BigWordEndBackward), None)
        }
        _ => (Action::Noop, None),
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
}
