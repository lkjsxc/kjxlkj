//! Pending state handlers for normal mode parser.

use crate::command::Command;
use crate::key::Key;
use crate::normal_types::{CharPendingKind, MarkPendingKind};
use kjxlkj_core_types::{
    ids::RegisterId,
    motion::Motion,
    operator::Operator,
    text_object::{TextObject, TextObjectScope},
};

/// Handle char pending (f, F, t, T, r).
pub fn handle_char_pending(
    kind: CharPendingKind,
    key: Key,
    count: Option<usize>,
    operator: Option<Operator>,
    register: Option<RegisterId>,
) -> Command {
    if let Key::Char(c) = key {
        match kind {
            CharPendingKind::Find(dir) => {
                let motion = Motion::FindChar {
                    char: c,
                    direction: dir,
                    inclusive: true,
                };
                motion_or_operator(motion, count, operator, register)
            }
            CharPendingKind::Till(dir) => {
                let motion = Motion::TillChar {
                    char: c,
                    direction: dir,
                };
                motion_or_operator(motion, count, operator, register)
            }
            CharPendingKind::Replace => Command::ReplaceChar(c),
        }
    } else {
        Command::Invalid
    }
}

/// Handle mark pending (m, ', `)
pub fn handle_mark_pending(kind: MarkPendingKind, key: Key) -> Command {
    if let Key::Char(c) = key {
        if c.is_ascii_alphabetic() {
            return match kind {
                MarkPendingKind::Set => Command::SetMark(c),
                MarkPendingKind::Jump => Command::GoToMark {
                    mark: c,
                    column: false,
                },
                MarkPendingKind::JumpColumn => Command::GoToMark {
                    mark: c,
                    column: true,
                },
            };
        }
    }
    Command::Invalid
}

/// Handle scope pending (text object).
pub fn handle_scope_pending(
    scope: TextObjectScope,
    key: Key,
    count: Option<usize>,
    operator: Option<Operator>,
    register: Option<RegisterId>,
) -> Command {
    if let Key::Char(c) = key {
        if let Some(obj) = TextObject::from_char(c, scope) {
            if let Some(op) = operator {
                return Command::OperatorTextObject {
                    count,
                    operator: op,
                    text_object: obj,
                    register: register.unwrap_or(RegisterId::Unnamed),
                };
            }
        }
    }
    Command::Invalid
}

/// Creates motion or operator+motion command.
fn motion_or_operator(
    motion: Motion,
    count: Option<usize>,
    operator: Option<Operator>,
    register: Option<RegisterId>,
) -> Command {
    if let Some(op) = operator {
        Command::OperatorMotion {
            count,
            operator: op,
            motion,
            register: register.unwrap_or(RegisterId::Unnamed),
        }
    } else {
        Command::Motion { count, motion }
    }
}
