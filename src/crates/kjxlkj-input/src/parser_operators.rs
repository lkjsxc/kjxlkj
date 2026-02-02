//! Operator parsing utilities.

use crate::{CommandLine, Key, KeyCodeWrapper};
use kjxlkj_core_edit::{Motion, MotionKind, Operator, OperatorKind};
use kjxlkj_core_mode::{Intent, IntentKind};
use kjxlkj_core_types::Mode;

/// Parses operator key (d, y, c, etc).
pub fn parse_operator_key(key: &Key) -> Option<OperatorKind> {
    match &key.code {
        KeyCodeWrapper::Char('d') => Some(OperatorKind::Delete),
        KeyCodeWrapper::Char('y') => Some(OperatorKind::Yank),
        KeyCodeWrapper::Char('c') => Some(OperatorKind::Change),
        KeyCodeWrapper::Char('>') => Some(OperatorKind::IndentRight),
        KeyCodeWrapper::Char('<') => Some(OperatorKind::IndentLeft),
        _ => None,
    }
}

/// Parses the second key for an operator (motion or same key for line).
pub fn parse_operator_argument(key: &Key, op_kind: OperatorKind) -> Option<Intent> {
    let same_key = match (&key.code, op_kind) {
        (KeyCodeWrapper::Char('d'), OperatorKind::Delete) => true,
        (KeyCodeWrapper::Char('y'), OperatorKind::Yank) => true,
        (KeyCodeWrapper::Char('c'), OperatorKind::Change) => true,
        (KeyCodeWrapper::Char('>'), OperatorKind::IndentRight) => true,
        (KeyCodeWrapper::Char('<'), OperatorKind::IndentLeft) => true,
        _ => false,
    };

    if same_key {
        let op = Operator::new(op_kind);
        return Some(Intent::new(IntentKind::OperatorLine { op }));
    }

    if let Some(motion) = parse_motion_key(key) {
        let op = Operator::new(op_kind);
        return Some(Intent::new(IntentKind::OperatorMotion { op, motion }));
    }

    if key.is_esc() {
        return Some(Intent::noop());
    }

    None
}

/// Parses a motion key.
pub fn parse_motion_key(key: &Key) -> Option<Motion> {
    match &key.code {
        KeyCodeWrapper::Char('h') => Some(Motion::new(MotionKind::Left)),
        KeyCodeWrapper::Char('j') => Some(Motion::new(MotionKind::Down)),
        KeyCodeWrapper::Char('k') => Some(Motion::new(MotionKind::Up)),
        KeyCodeWrapper::Char('l') => Some(Motion::new(MotionKind::Right)),
        KeyCodeWrapper::Char('w') => Some(Motion::new(MotionKind::WordStart)),
        KeyCodeWrapper::Char('b') => Some(Motion::new(MotionKind::WordBack)),
        KeyCodeWrapper::Char('e') => Some(Motion::new(MotionKind::WordEnd)),
        KeyCodeWrapper::Char('0') => Some(Motion::new(MotionKind::LineStart)),
        KeyCodeWrapper::Char('$') => Some(Motion::new(MotionKind::LineEnd)),
        _ => None,
    }
}

/// Parses basic motion/command keys in normal mode.
pub fn parse_normal_key(key: &Key, cmdline: &mut CommandLine) -> Option<Intent> {
    match &key.code {
        KeyCodeWrapper::Char('h') => Some(Intent::motion(Motion::new(MotionKind::Left))),
        KeyCodeWrapper::Char('j') => Some(Intent::motion(Motion::new(MotionKind::Down))),
        KeyCodeWrapper::Char('k') => Some(Intent::motion(Motion::new(MotionKind::Up))),
        KeyCodeWrapper::Char('l') => Some(Intent::motion(Motion::new(MotionKind::Right))),
        KeyCodeWrapper::Char('w') => Some(Intent::motion(Motion::new(MotionKind::WordStart))),
        KeyCodeWrapper::Char('b') => Some(Intent::motion(Motion::new(MotionKind::WordBack))),
        KeyCodeWrapper::Char('e') => Some(Intent::motion(Motion::new(MotionKind::WordEnd))),
        KeyCodeWrapper::Char('0') => Some(Intent::motion(Motion::new(MotionKind::LineStart))),
        KeyCodeWrapper::Char('$') => Some(Intent::motion(Motion::new(MotionKind::LineEnd))),
        KeyCodeWrapper::Char('i') | KeyCodeWrapper::Char('a') => Some(Intent::change_mode(Mode::Insert)),
        KeyCodeWrapper::Char('v') => Some(Intent::change_mode(Mode::Visual)),
        KeyCodeWrapper::Char('V') => Some(Intent::change_mode(Mode::VisualLine)),
        KeyCodeWrapper::Char(':') => Some(Intent::change_mode(Mode::Command)),
        KeyCodeWrapper::Char('u') => Some(Intent::new(IntentKind::Undo)),
        KeyCodeWrapper::Char('p') => Some(Intent::new(IntentKind::PutAfter { register: None })),
        KeyCodeWrapper::Char('P') => Some(Intent::new(IntentKind::PutBefore { register: None })),
        KeyCodeWrapper::Char('n') => Some(Intent::new(IntentKind::NextMatch)),
        KeyCodeWrapper::Char('N') => Some(Intent::new(IntentKind::PrevMatch)),
        KeyCodeWrapper::Char('/') => { cmdline.open('/'); Some(Intent::change_mode(Mode::Command)) }
        KeyCodeWrapper::Char('?') => { cmdline.open('?'); Some(Intent::change_mode(Mode::Command)) }
        _ => None,
    }
}
