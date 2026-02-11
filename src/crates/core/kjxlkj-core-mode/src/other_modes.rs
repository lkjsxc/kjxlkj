//! Non-Normal mode handlers: Insert, Command, Visual,
//! Replace, OperatorPending, TerminalInsert.

use kjxlkj_core_types::{
    Action, ForceModifier, Key, KeyModifiers, Mode, Operator,
};

use crate::normal_motions;
use crate::pending::PendingState;

pub(crate) fn handle_insert_key(
    key: &Key,
    mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    if mods.ctrl {
        return match key {
            Key::Char('o') => {
                (Action::Noop, Some(Mode::InsertNormal))
            }
            Key::Char('w') => {
                (Action::DeleteWordBackward, None)
            }
            Key::Char('u') => {
                (Action::DeleteToLineStart, None)
            }
            _ => (Action::Noop, None),
        };
    }
    match key {
        Key::Escape => {
            (Action::ExitToNormal, Some(Mode::Normal))
        }
        Key::Char(c) => (Action::InsertChar(*c), None),
        Key::Enter => (Action::InsertChar('\n'), None),
        Key::Backspace => {
            (Action::DeleteCharBackward, None)
        }
        Key::Delete => (Action::DeleteCharForward, None),
        Key::Tab => (Action::InsertChar('\t'), None),
        _ => (Action::Noop, None),
    }
}

pub(crate) fn handle_command_key(
    key: &Key,
    _mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => {
            (Action::ExitToNormal, Some(Mode::Normal))
        }
        Key::Enter => {
            (Action::ExCommand(String::new()), Some(Mode::Normal))
        }
        _ => (Action::Noop, None),
    }
}

pub(crate) fn handle_visual_key(
    key: &Key,
    _mods: &KeyModifiers,
    _mode: Mode,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => {
            (Action::ExitToNormal, Some(Mode::Normal))
        }
        _ => (Action::Noop, None),
    }
}

pub(crate) fn handle_replace_key(
    key: &Key,
    _mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => {
            (Action::ExitToNormal, Some(Mode::Normal))
        }
        Key::Char(c) => (Action::ReplaceChar(*c), None),
        _ => (Action::Noop, None),
    }
}

pub(crate) fn handle_operator_pending(
    key: &Key,
    mods: &KeyModifiers,
    mode: Mode,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    if let Mode::OperatorPending(op) = mode {
        // Double-operator detection: dd, cc, yy, >>, <<,
        // ==, guu, gUU, g~~, gqq, !!
        let op_char = match op {
            Operator::Delete => Some('d'),
            Operator::Change => Some('c'),
            Operator::Yank => Some('y'),
            Operator::Indent => Some('>'),
            Operator::Dedent => Some('<'),
            Operator::Reindent => Some('='),
            Operator::Lowercase => Some('u'),
            Operator::Uppercase => Some('U'),
            Operator::ToggleCase => Some('~'),
            Operator::Format => Some('q'),
            Operator::Filter => Some('!'),
        };
        if !mods.ctrl {
            if let Key::Char(c) = key {
                if Some(*c) == op_char {
                    pending.clear();
                    return (
                        Action::OperatorLine(op),
                        Some(Mode::Normal),
                    );
                }
                // Force modifiers: v/V between operator and motion.
                if *c == 'v' && pending.force.is_none() {
                    pending.force = Some(ForceModifier::Characterwise);
                    return (Action::Noop, None);
                }
                if *c == 'V' && pending.force.is_none() {
                    pending.force = Some(ForceModifier::Linewise);
                    return (Action::Noop, None);
                }
            }
        }
        // Ctrl-v force blockwise modifier.
        if mods.ctrl {
            if let Key::Char('v') = key {
                if pending.force.is_none() {
                    pending.force = Some(ForceModifier::Blockwise);
                    return (Action::Noop, None);
                }
            }
        }
    }
    // Count accumulation in operator-pending
    if let Key::Char(c) = key {
        if c.is_ascii_digit() {
            let d = *c as u8 - b'0';
            if pending.push_digit(d) {
                return (Action::Noop, None);
            }
        }
    }
    match key {
        Key::Escape => {
            pending.clear();
            (Action::ExitToNormal, Some(Mode::Normal))
        }
        _ => {
            if let Some(action) =
                normal_motions::motion_for_key(key)
            {
                if let Action::Motion(m) = action {
                    if let Mode::OperatorPending(op) =
                        mode
                    {
                        let cnt =
                            pending.multiplied_count();
                        pending.clear();
                        let target_mode =
                            if op == Operator::Change {
                                Some(Mode::Insert)
                            } else {
                                Some(Mode::Normal)
                            };
                        return (
                            Action::OperatorMotion(
                                op, m, cnt,
                            ),
                            target_mode,
                        );
                    }
                }
            }
            pending.clear();
            (Action::Noop, Some(Mode::Normal))
        }
    }
}

pub(crate) fn handle_terminal_insert(
    key: &Key,
    mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    if mods.ctrl {
        if let Key::Char('n') = key {
            return (
                Action::ExitToNormal,
                Some(Mode::Normal),
            );
        }
    }
    (Action::ForwardKey(key.clone(), *mods), None)
}
