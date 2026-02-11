//! Mode dispatch and transition logic.
//!
//! See /docs/spec/modes/transitions.md for the normative
//! transition table.

mod normal;
mod normal_g;
mod normal_motions;
mod normal_partial;
mod normal_wincmd;
mod normal_z;
mod other_modes;
mod pending;
mod resolver;

pub use normal::handle_normal_key;
pub use pending::{PartialKey, PendingState};
pub use resolver::resolve_mode_transition;

use kjxlkj_core_types::{Action, Key, KeyModifiers, Mode};

/// Process a key event in the current mode context.
///
/// Returns the action to execute and optionally a new mode.
/// `pending` holds multi-key state for normal mode.
pub fn dispatch_key(
    mode: Mode,
    key: &Key,
    mods: &KeyModifiers,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    match mode {
        Mode::Normal | Mode::InsertNormal => {
            normal::handle_normal_key(key, mods, pending)
        }
        Mode::Insert => {
            other_modes::handle_insert_key(key, mods)
        }
        Mode::Command(_) => {
            other_modes::handle_command_key(key, mods)
        }
        Mode::Visual(_) => {
            other_modes::handle_visual_key(key, mods, mode)
        }
        Mode::Replace => {
            other_modes::handle_replace_key(key, mods)
        }
        Mode::OperatorPending(_) => {
            other_modes::handle_operator_pending(
                key, mods, mode, pending,
            )
        }
        Mode::TerminalInsert => {
            other_modes::handle_terminal_insert(key, mods)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_escape_returns_normal() {
        let mut ps = PendingState::default();
        let (action, mode) = dispatch_key(
            Mode::Insert,
            &Key::Escape,
            &KeyModifiers::default(),
            &mut ps,
        );
        assert_eq!(action, Action::ExitToNormal);
        assert_eq!(mode, Some(Mode::Normal));
    }

    #[test]
    fn insert_char_stays_in_insert() {
        let mut ps = PendingState::default();
        let (action, mode) = dispatch_key(
            Mode::Insert,
            &Key::Char('x'),
            &KeyModifiers::default(),
            &mut ps,
        );
        assert_eq!(action, Action::InsertChar('x'));
        assert_eq!(mode, None);
    }

    #[test]
    fn normal_mode_a_enters_insert() {
        let mut ps = PendingState::default();
        let (_, mode) = dispatch_key(
            Mode::Normal,
            &Key::Char('a'),
            &KeyModifiers::default(),
            &mut ps,
        );
        assert_eq!(mode, Some(Mode::Insert));
    }

    #[test]
    fn insert_ctrl_o_enters_insert_normal() {
        let mut ps = PendingState::default();
        let (_, mode) = dispatch_key(
            Mode::Insert,
            &Key::Char('o'),
            &KeyModifiers {
                ctrl: true,
                ..Default::default()
            },
            &mut ps,
        );
        assert_eq!(mode, Some(Mode::InsertNormal));
    }

    #[test]
    fn dd_produces_operator_line_delete() {
        let mut ps = PendingState::default();
        let (_, mode1) = dispatch_key(
            Mode::Normal,
            &Key::Char('d'),
            &KeyModifiers::default(),
            &mut ps,
        );
        assert_eq!(
            mode1,
            Some(Mode::OperatorPending(
                kjxlkj_core_types::Operator::Delete
            ))
        );
        let (action, mode2) = dispatch_key(
            Mode::OperatorPending(
                kjxlkj_core_types::Operator::Delete,
            ),
            &Key::Char('d'),
            &KeyModifiers::default(),
            &mut ps,
        );
        assert_eq!(
            action,
            Action::OperatorLine(
                kjxlkj_core_types::Operator::Delete
            )
        );
        assert_eq!(mode2, Some(Mode::Normal));
    }
}
