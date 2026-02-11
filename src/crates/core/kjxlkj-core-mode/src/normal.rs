//! Normal mode key dispatch.
//!
//! See /docs/spec/modes/normal.md for normative key tables.

use kjxlkj_core_types::{
    Action, CommandKind, Key, KeyModifiers, Mode, Operator,
    VisualKind,
};

use crate::normal_motions;

/// Handle a key in Normal mode.
///
/// Returns (Action, Option<new Mode>).
/// `Shift+a` is already normalized to `Key::Char('A')` by the input
/// pipeline before this function is called.
pub fn handle_normal_key(
    key: &Key,
    mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    // Ctrl combinations first.
    if mods.ctrl {
        return handle_ctrl_key(key);
    }

    // Check motion keys (extracted to normal_motions module).
    if let Some(action) = normal_motions::motion_for_key(key) {
        return (action, None);
    }

    match key {
        // Insert entry keys.
        Key::Char('i') => {
            (Action::EnterMode(Mode::Insert), Some(Mode::Insert))
        }
        Key::Char('a') => {
            (Action::EnterMode(Mode::Insert), Some(Mode::Insert))
        }
        Key::Char('A') => {
            (Action::AppendEndOfLine, Some(Mode::Insert))
        }
        Key::Char('I') => {
            (Action::InsertFirstNonBlank, Some(Mode::Insert))
        }
        Key::Char('o') => {
            (Action::OpenLineBelow, Some(Mode::Insert))
        }
        Key::Char('O') => {
            (Action::OpenLineAbove, Some(Mode::Insert))
        }

        // Operator keys.
        Key::Char('d') => (
            Action::Noop,
            Some(Mode::OperatorPending(Operator::Delete)),
        ),
        Key::Char('c') => (
            Action::Noop,
            Some(Mode::OperatorPending(Operator::Change)),
        ),
        Key::Char('y') => (
            Action::Noop,
            Some(Mode::OperatorPending(Operator::Yank)),
        ),

        // Visual mode.
        Key::Char('v') => (
            Action::EnterMode(Mode::Visual(VisualKind::Char)),
            Some(Mode::Visual(VisualKind::Char)),
        ),
        Key::Char('V') => (
            Action::EnterMode(Mode::Visual(VisualKind::Line)),
            Some(Mode::Visual(VisualKind::Line)),
        ),

        // Command mode.
        Key::Char(':') => (
            Action::EnterMode(Mode::Command(CommandKind::Ex)),
            Some(Mode::Command(CommandKind::Ex)),
        ),
        Key::Char('/') => (
            Action::EnterMode(Mode::Command(CommandKind::Search)),
            Some(Mode::Command(CommandKind::Search)),
        ),
        Key::Char('?') => (
            Action::EnterMode(Mode::Command(CommandKind::Search)),
            Some(Mode::Command(CommandKind::Search)),
        ),

        // Replace mode.
        Key::Char('R') => {
            (Action::EnterMode(Mode::Replace), Some(Mode::Replace))
        }

        // Single-key commands.
        Key::Char('x') => (Action::DeleteCharForward, None),
        Key::Char('X') => (Action::DeleteCharBackward, None),
        Key::Char('u') => (Action::Undo, None),
        Key::Char('p') => (Action::PutAfter, None),
        Key::Char('P') => (Action::PutBefore, None),
        Key::Char('J') => (Action::JoinLines, None),
        Key::Char('~') => (Action::ToggleCase, None),
        Key::Char('.') => (Action::DotRepeat, None),

        Key::Escape => (Action::Noop, None),
        _ => (Action::Noop, None),
    }
}

fn handle_ctrl_key(key: &Key) -> (Action, Option<Mode>) {
    match key {
        Key::Char('r') => (Action::Redo, None),
        Key::Char('w') => {
            // Window command prefix; will be handled by prefix
            // resolver. For now, emit noop.
            (Action::Noop, None)
        }
        Key::Char('d') => {
            (Action::Motion(Motion::HalfPageDown), None)
        }
        Key::Char('u') => {
            (Action::Motion(Motion::HalfPageUp), None)
        }
        Key::Char('f') => {
            (Action::Motion(Motion::PageDown), None)
        }
        Key::Char('b') => {
            (Action::Motion(Motion::PageUp), None)
        }
        _ => (Action::Noop, None),
    }
}

// Re-import Motion for ctrl handler above.
use kjxlkj_core_types::Motion;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_a_triggers_append_eol() {
        let (action, mode) = handle_normal_key(
            &Key::Char('A'),
            &KeyModifiers::default(),
        );
        assert_eq!(action, Action::AppendEndOfLine);
        assert_eq!(mode, Some(Mode::Insert));
    }

    #[test]
    fn physical_a_triggers_append() {
        let (_, mode) = handle_normal_key(
            &Key::Char('a'),
            &KeyModifiers::default(),
        );
        assert_eq!(mode, Some(Mode::Insert));
    }

    #[test]
    fn colon_enters_command_mode() {
        let (_, mode) = handle_normal_key(
            &Key::Char(':'),
            &KeyModifiers::default(),
        );
        assert_eq!(mode, Some(Mode::Command(CommandKind::Ex)));
    }

    #[test]
    fn h_moves_left() {
        let (action, mode) = handle_normal_key(
            &Key::Char('h'),
            &KeyModifiers::default(),
        );
        assert_eq!(action, Action::Motion(Motion::Left));
        assert_eq!(mode, None);
    }

    #[test]
    fn ctrl_r_redo() {
        let (action, _) = handle_normal_key(
            &Key::Char('r'),
            &KeyModifiers {
                ctrl: true,
                ..Default::default()
            },
        );
        assert_eq!(action, Action::Redo);
    }
}
