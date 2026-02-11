//! Normal mode key dispatch. See /docs/spec/modes/normal.md.

use kjxlkj_core_types::{
    Action, CommandKind, Key, KeyModifiers, Mode, Motion,
    Operator, VisualKind,
};

use crate::normal_motions;
use crate::normal_partial;
use crate::pending::{PartialKey, PendingState};

/// Handle a key in Normal mode.
pub fn handle_normal_key(
    key: &Key,
    mods: &KeyModifiers,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    if mods.ctrl {
        return handle_ctrl_key(key, pending);
    }
    if pending.partial != PartialKey::None {
        return normal_partial::resolve_partial(key, pending);
    }
    if let Key::Char(c) = key {
        if c.is_ascii_digit() {
            let d = *c as u8 - b'0';
            if pending.push_digit(d) {
                return (Action::Noop, None);
            }
        }
    }
    if let Some(action) = normal_motions::motion_for_key(key) {
        pending.clear();
        return (action, None);
    }
    handle_normal_command(key, pending)
}

/// Single-key commands, mode entries, and operators.
fn handle_normal_command(key: &Key, pending: &mut PendingState) -> (Action, Option<Mode>) {
    match key {
        Key::Char('i') => done(pending, Action::EnterMode(Mode::Insert), Mode::Insert),
        Key::Char('a') => done(pending, Action::EnterMode(Mode::Insert), Mode::Insert),
        Key::Char('A') => done(pending, Action::AppendEndOfLine, Mode::Insert),
        Key::Char('I') => done(pending, Action::InsertFirstNonBlank, Mode::Insert),
        Key::Char('o') => done(pending, Action::OpenLineBelow, Mode::Insert),
        Key::Char('O') => done(pending, Action::OpenLineAbove, Mode::Insert),
        Key::Char('s') => done(pending, Action::SubstituteChar, Mode::Insert),
        Key::Char('S') => done(pending, Action::SubstituteLine, Mode::Insert),
        Key::Char('C') => done(pending, Action::ChangeToEnd, Mode::Insert),
        Key::Char('D') => cleared(pending, Action::DeleteToEnd),
        Key::Char('Y') => cleared(pending, Action::YankCurrentLine),
        Key::Char('d') => op_pending(pending, Operator::Delete),
        Key::Char('c') => op_pending(pending, Operator::Change),
        Key::Char('y') => op_pending(pending, Operator::Yank),
        Key::Char('>') => op_pending(pending, Operator::Indent),
        Key::Char('<') => op_pending(pending, Operator::Dedent),
        Key::Char('=') => op_pending(pending, Operator::Reindent),
        Key::Char('!') => op_pending(pending, Operator::Filter),
        Key::Char('v') => done(pending, Action::EnterMode(Mode::Visual(VisualKind::Char)), Mode::Visual(VisualKind::Char)),
        Key::Char('V') => done(pending, Action::EnterMode(Mode::Visual(VisualKind::Line)), Mode::Visual(VisualKind::Line)),
        Key::Char(':') => done(pending, Action::EnterMode(Mode::Command(CommandKind::Ex)), Mode::Command(CommandKind::Ex)),
        Key::Char('/') => done(pending, Action::EnterMode(Mode::Command(CommandKind::SearchForward)), Mode::Command(CommandKind::SearchForward)),
        Key::Char('?') => done(pending, Action::EnterMode(Mode::Command(CommandKind::SearchBackward)), Mode::Command(CommandKind::SearchBackward)),
        Key::Char('R') => done(pending, Action::EnterMode(Mode::Replace), Mode::Replace),
        Key::Char('g') => { pending.partial = PartialKey::G; (Action::Noop, None) }
        Key::Char('z') => { pending.partial = PartialKey::Z; (Action::Noop, None) }
        Key::Char('f') => { pending.partial = PartialKey::FindForward; (Action::Noop, None) }
        Key::Char('F') => { pending.partial = PartialKey::FindBackward; (Action::Noop, None) }
        Key::Char('t') => { pending.partial = PartialKey::TillForward; (Action::Noop, None) }
        Key::Char('T') => { pending.partial = PartialKey::TillBackward; (Action::Noop, None) }
        Key::Char('r') => { pending.partial = PartialKey::ReplaceChar; (Action::Noop, None) }
        Key::Char('m') => { pending.partial = PartialKey::SetMark; (Action::Noop, None) }
        Key::Char('"') => { pending.partial = PartialKey::Register; (Action::Noop, None) }
        Key::Char('x') => cleared(pending, Action::DeleteCharForward),
        Key::Char('X') => cleared(pending, Action::DeleteCharBackward),
        Key::Char('u') => cleared(pending, Action::Undo),
        Key::Char('p') => cleared(pending, Action::PutAfter),
        Key::Char('P') => cleared(pending, Action::PutBefore),
        Key::Char('J') => cleared(pending, Action::JoinLines),
        Key::Char('~') => cleared(pending, Action::ToggleCase),
        Key::Char('.') => cleared(pending, Action::DotRepeat),
        Key::Char('n') => cleared(pending, Action::Motion(Motion::SearchNext)),
        Key::Char('N') => cleared(pending, Action::Motion(Motion::SearchPrev)),
        Key::Char('*') => cleared(pending, Action::StarSearchForward),
        Key::Char('#') => cleared(pending, Action::StarSearchBackward),
        Key::Char(';') => cleared(pending, Action::Motion(Motion::RepeatFind)),
        Key::Char(',') => cleared(pending, Action::Motion(Motion::RepeatFindReverse)),
        Key::Escape => { pending.clear(); (Action::Noop, None) }
        _ => { pending.clear(); (Action::Noop, None) }
    }
}

fn handle_ctrl_key(
    key: &Key,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    pending.clear();
    match key {
        Key::Char('r') => (Action::Redo, None),
        Key::Char('v') => (
            Action::EnterMode(Mode::Visual(VisualKind::Block)),
            Some(Mode::Visual(VisualKind::Block)),
        ),
        Key::Char('d') => (Action::Motion(Motion::HalfPageDown), None),
        Key::Char('u') => (Action::Motion(Motion::HalfPageUp), None),
        Key::Char('f') => (Action::Motion(Motion::PageDown), None),
        Key::Char('b') => (Action::Motion(Motion::PageUp), None),
        Key::Char('e') => (Action::Motion(Motion::ScrollDown), None),
        Key::Char('y') => (Action::Motion(Motion::ScrollUp), None),
        Key::Char('a') => (Action::IncrementNumber, None),
        Key::Char('x') => (Action::DecrementNumber, None),
        Key::Char('w') => (Action::Noop, None),
        _ => (Action::Noop, None),
    }
}

/// Helper: clear pending, return action with mode transition.
fn done(p: &mut PendingState, a: Action, m: Mode) -> (Action, Option<Mode>) {
    p.clear();
    (a, Some(m))
}

/// Helper: clear pending, return action, stay in same mode.
fn cleared(p: &mut PendingState, a: Action) -> (Action, Option<Mode>) {
    p.clear();
    (a, None)
}

/// Helper: enter OperatorPending, saving pre-operator count.
fn op_pending(p: &mut PendingState, op: Operator) -> (Action, Option<Mode>) {
    p.save_pre_op_count();
    (Action::Noop, Some(Mode::OperatorPending(op)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_a_triggers_append_eol() {
        let mut ps = PendingState::default();
        let m = KeyModifiers::default();
        let (action, mode) = handle_normal_key(&Key::Char('A'), &m, &mut ps);
        assert_eq!(action, Action::AppendEndOfLine);
        assert_eq!(mode, Some(Mode::Insert));
    }

    #[test]
    fn colon_enters_command_mode() {
        let mut ps = PendingState::default();
        let m = KeyModifiers::default();
        let (_, mode) = handle_normal_key(&Key::Char(':'), &m, &mut ps);
        assert_eq!(mode, Some(Mode::Command(CommandKind::Ex)));
    }

    #[test]
    fn h_moves_left() {
        let mut ps = PendingState::default();
        let m = KeyModifiers::default();
        let (action, mode) = handle_normal_key(&Key::Char('h'), &m, &mut ps);
        assert_eq!(action, Action::Motion(Motion::Left));
        assert_eq!(mode, None);
    }

    #[test]
    fn ctrl_r_redo() {
        let mut ps = PendingState::default();
        let m = KeyModifiers { ctrl: true, ..Default::default() };
        let (action, _) = handle_normal_key(&Key::Char('r'), &m, &mut ps);
        assert_eq!(action, Action::Redo);
    }

    #[test]
    fn f_then_char_produces_find_forward() {
        let mut ps = PendingState::default();
        let (a1, _) = handle_normal_key(
            &Key::Char('f'), &KeyModifiers::default(), &mut ps,
        );
        assert_eq!(a1, Action::Noop);
        assert_eq!(ps.partial, PartialKey::FindForward);
        let (a2, _) = handle_normal_key(
            &Key::Char('x'), &KeyModifiers::default(), &mut ps,
        );
        assert_eq!(a2, Action::Motion(Motion::FindForward('x')));
    }

    #[test]
    fn count_prefix_accumulates() {
        let mut ps = PendingState::default();
        handle_normal_key(
            &Key::Char('3'), &KeyModifiers::default(), &mut ps,
        );
        assert_eq!(ps.count, Some(3));
        handle_normal_key(
            &Key::Char('5'), &KeyModifiers::default(), &mut ps,
        );
        assert_eq!(ps.count, Some(35));
    }
}
