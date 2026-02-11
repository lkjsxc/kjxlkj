//! Non-Normal mode handlers: Insert, Command, Visual,
//! Replace, OperatorPending, TerminalInsert.

use kjxlkj_core_types::{
    Action, ForceModifier, Key, KeyModifiers, Mode, Motion, Operator, VisualKind,
};

use crate::normal_motions;
use crate::pending::{PartialKey, PendingState};

pub(crate) fn handle_insert_key(key: &Key, mods: &KeyModifiers) -> (Action, Option<Mode>) {
    if mods.ctrl { return match key {
        Key::Char('o') => (Action::Noop, Some(Mode::InsertNormal)),
        Key::Char('w') => (Action::DeleteWordBackward, None),
        Key::Char('u') => (Action::DeleteToLineStart, None),
        _ => (Action::Noop, None),
    }; }
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        Key::Char(c) => (Action::InsertChar(*c), None),
        Key::Enter => (Action::InsertChar('\n'), None),
        Key::Backspace => (Action::DeleteCharBackward, None),
        Key::Delete => (Action::DeleteCharForward, None),
        Key::Tab => (Action::InsertChar('\t'), None),
        _ => (Action::Noop, None),
    }
}

pub(crate) fn handle_command_key(key: &Key, _mods: &KeyModifiers) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        Key::Enter => (Action::ExCommand(String::new()), Some(Mode::Normal)),
        _ => (Action::Noop, None),
    }
}

pub(crate) fn handle_visual_key(key: &Key, mods: &KeyModifiers, mode: Mode) -> (Action, Option<Mode>) {
    let kind = match mode { Mode::Visual(k) => k, _ => VisualKind::Char };
    if !mods.ctrl {
        if let Key::Char(c) = key {
            // Sub-mode switching or exit.
            match *c {
                'v' => return if kind == VisualKind::Char {
                    (Action::ExitToNormal, Some(Mode::Normal))
                } else {
                    (Action::EnterMode(Mode::Visual(VisualKind::Char)), Some(Mode::Visual(VisualKind::Char)))
                },
                'V' => return if kind == VisualKind::Line {
                    (Action::ExitToNormal, Some(Mode::Normal))
                } else {
                    (Action::EnterMode(Mode::Visual(VisualKind::Line)), Some(Mode::Visual(VisualKind::Line)))
                },
                'o' => return (Action::VisualSwapAnchor, None),
                'd' | 'x' => return (Action::VisualOperator(Operator::Delete), Some(Mode::Normal)),
                'y' => return (Action::VisualOperator(Operator::Yank), Some(Mode::Normal)),
                'c' | 's' => return (Action::VisualOperator(Operator::Change), Some(Mode::Insert)),
                '>' => return (Action::VisualOperator(Operator::Indent), Some(Mode::Normal)),
                '<' => return (Action::VisualOperator(Operator::Dedent), Some(Mode::Normal)),
                '~' => return (Action::VisualOperator(Operator::ToggleCase), Some(Mode::Normal)),
                'u' => return (Action::VisualOperator(Operator::Lowercase), Some(Mode::Normal)),
                'U' => return (Action::VisualOperator(Operator::Uppercase), Some(Mode::Normal)),
                'J' => return (Action::JoinLines, Some(Mode::Normal)),
                'p' => return (Action::PutAfter, Some(Mode::Normal)),
                _ => {}
            }
        }
    }
    if mods.ctrl {
        if let Key::Char('v') = key {
            return if kind == VisualKind::Block {
                (Action::ExitToNormal, Some(Mode::Normal))
            } else {
                (Action::EnterMode(Mode::Visual(VisualKind::Block)), Some(Mode::Visual(VisualKind::Block)))
            };
        }
    }
    // Escape exits visual mode.
    if matches!(key, Key::Escape) { return (Action::ExitToNormal, Some(Mode::Normal)); }
    // Try motions.
    if let Some(action) = normal_motions::motion_for_key(key) {
        return (action, None);
    }
    (Action::Noop, None)
}

pub(crate) fn handle_replace_key(key: &Key, _mods: &KeyModifiers) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        Key::Char(c) => (Action::ReplaceChar(*c), None),
        _ => (Action::Noop, None),
    }
}

pub(crate) fn handle_operator_pending(
    key: &Key, mods: &KeyModifiers, mode: Mode, pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    // Resolve text object partial (i/a + char).
    if matches!(pending.partial, PartialKey::TextObjectInner | PartialKey::TextObjectAround) {
        if let (Mode::OperatorPending(op), Key::Char(c)) = (mode, key) {
            let m = if pending.partial == PartialKey::TextObjectInner {
                Motion::TextObjInner(*c)
            } else {
                Motion::TextObjAround(*c)
            };
            let cnt = pending.multiplied_count();
            let tm = if op == Operator::Change { Some(Mode::Insert) } else { Some(Mode::Normal) };
            pending.clear();
            return (Action::OperatorMotion(op, m, cnt), tm);
        }
        pending.clear();
        return (Action::Noop, Some(Mode::Normal));
    }
    if let Mode::OperatorPending(op) = mode {
        let op_char = match op {
            Operator::Delete => Some('d'), Operator::Change => Some('c'),
            Operator::Yank => Some('y'), Operator::Indent => Some('>'),
            Operator::Dedent => Some('<'), Operator::Reindent => Some('='),
            Operator::Lowercase => Some('u'), Operator::Uppercase => Some('U'),
            Operator::ToggleCase => Some('~'), Operator::Format => Some('q'),
            Operator::Filter => Some('!'),
        };
        if !mods.ctrl {
            if let Key::Char(c) = key {
                if Some(*c) == op_char {
                    pending.clear();
                    return (Action::OperatorLine(op), Some(Mode::Normal));
                }
                if *c == 'v' && pending.force.is_none() {
                    pending.force = Some(ForceModifier::Characterwise);
                    return (Action::Noop, None);
                }
                if *c == 'V' && pending.force.is_none() {
                    pending.force = Some(ForceModifier::Linewise);
                    return (Action::Noop, None);
                }
                // Text object prefixes.
                if *c == 'i' {
                    pending.partial = PartialKey::TextObjectInner;
                    return (Action::Noop, None);
                }
                if *c == 'a' {
                    pending.partial = PartialKey::TextObjectAround;
                    return (Action::Noop, None);
                }
            }
        }
        if mods.ctrl {
            if let Key::Char('v') = key {
                if pending.force.is_none() {
                    pending.force = Some(ForceModifier::Blockwise);
                    return (Action::Noop, None);
                }
            }
        }
    }
    if let Key::Char(c) = key {
        if c.is_ascii_digit() {
            let d = *c as u8 - b'0';
            if pending.push_digit(d) { return (Action::Noop, None); }
        }
    }
    match key {
        Key::Escape => { pending.clear(); (Action::ExitToNormal, Some(Mode::Normal)) }
        _ => {
            if let Some(action) = normal_motions::motion_for_key(key) {
                if let Action::Motion(m) = action {
                    if let Mode::OperatorPending(op) = mode {
                        let cnt = pending.multiplied_count();
                        pending.clear();
                        let tm = if op == Operator::Change { Some(Mode::Insert) } else { Some(Mode::Normal) };
                        return (Action::OperatorMotion(op, m, cnt), tm);
                    }
                }
            }
            pending.clear();
            (Action::Noop, Some(Mode::Normal))
        }
    }
}

pub(crate) fn handle_terminal_insert(key: &Key, mods: &KeyModifiers) -> (Action, Option<Mode>) {
    if mods.ctrl { if let Key::Char('n') = key { return (Action::ExitToNormal, Some(Mode::Normal)); } }
    (Action::ForwardKey(key.clone(), *mods), None)
}
