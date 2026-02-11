//! Normal mode key dispatch.
//!
//! See /docs/spec/modes/normal.md for normative key tables.

use kjxlkj_core_types::{
    Action, CommandKind, Key, KeyModifiers, Mode, Motion, Operator,
    VisualKind, Direction,
};

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

        // Motions.
        Key::Char('h') | Key::Left => {
            (Action::Motion(Motion::Left), None)
        }
        Key::Char('l') | Key::Right => {
            (Action::Motion(Motion::Right), None)
        }
        Key::Char('k') | Key::Up => {
            (Action::Motion(Motion::Up), None)
        }
        Key::Char('j') | Key::Down => {
            (Action::Motion(Motion::Down), None)
        }
        Key::Char('0') => {
            (Action::Motion(Motion::LineStart), None)
        }
        Key::Char('$') => {
            (Action::Motion(Motion::LineEnd), None)
        }
        Key::Char('^') => {
            (Action::Motion(Motion::FirstNonBlank), None)
        }
        Key::Char('w') => {
            (Action::Motion(Motion::WordForward), None)
        }
        Key::Char('b') => {
            (Action::Motion(Motion::WordBackward), None)
        }
        Key::Char('e') => {
            (Action::Motion(Motion::WordEndForward), None)
        }
        Key::Char('W') => {
            (Action::Motion(Motion::BigWordForward), None)
        }
        Key::Char('B') => {
            (Action::Motion(Motion::BigWordBackward), None)
        }
        Key::Char('E') => {
            (Action::Motion(Motion::BigWordEndForward), None)
        }
        Key::Char('G') => {
            (Action::Motion(Motion::GotoLastLine), None)
        }
        Key::Char('{') => {
            (Action::Motion(Motion::ParagraphBackward), None)
        }
        Key::Char('}') => {
            (Action::Motion(Motion::ParagraphForward), None)
        }
        Key::Char('%') => {
            (Action::Motion(Motion::MatchParen), None)
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

        // Scrolling.
        Key::PageDown => {
            (Action::Motion(Motion::PageDown), None)
        }
        Key::PageUp => (Action::Motion(Motion::PageUp), None),

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_a_triggers_append_eol() {
        // After normalization, Shift+a == Key::Char('A').
        let (action, mode) = handle_normal_key(
            &Key::Char('A'),
            &KeyModifiers::default(),
        );
        assert_eq!(action, Action::AppendEndOfLine);
        assert_eq!(mode, Some(Mode::Insert));
    }

    #[test]
    fn physical_a_triggers_append() {
        let (action, mode) = handle_normal_key(
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
