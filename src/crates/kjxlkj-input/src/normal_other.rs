//! Other character parsing helpers for normal mode.

use crate::command::Command;
use crate::normal_types::{CharPendingKind, MarkPendingKind};
use kjxlkj_core_types::{
    ids::RegisterId,
    motion::{Direction, Motion},
    operator::Operator,
};

/// Parse miscellaneous single characters in normal mode.
pub fn parse_other_char(
    c: char,
    count: &mut Option<usize>,
    g_prefix: &mut bool,
    z_prefix: &mut bool,
    char_pending: &mut Option<CharPendingKind>,
    mark_pending: &mut Option<MarkPendingKind>,
    register: Option<RegisterId>,
) -> Option<Command> {
    match c {
        'G' => {
            if let Some(n) = count.take() {
                Some(Command::Motion {
                    count: None,
                    motion: Motion::GoToLine(n.saturating_sub(1)),
                })
            } else {
                Some(Command::Motion {
                    count: None,
                    motion: Motion::DocumentEnd,
                })
            }
        }
        'g' => {
            *g_prefix = true;
            Some(Command::Incomplete)
        }
        'z' => {
            *z_prefix = true;
            Some(Command::Incomplete)
        }
        'u' => Some(Command::Undo),
        '.' => Some(Command::Repeat),
        'r' => {
            *char_pending = Some(CharPendingKind::Replace);
            Some(Command::Incomplete)
        }
        'R' => Some(Command::ReplaceMode),
        'm' => {
            *mark_pending = Some(MarkPendingKind::Set);
            Some(Command::Incomplete)
        }
        '\'' => {
            *mark_pending = Some(MarkPendingKind::Jump);
            Some(Command::Incomplete)
        }
        '`' => {
            *mark_pending = Some(MarkPendingKind::JumpColumn);
            Some(Command::Incomplete)
        }
        'f' => {
            *char_pending = Some(CharPendingKind::Find(Direction::Forward));
            Some(Command::Incomplete)
        }
        'F' => {
            *char_pending = Some(CharPendingKind::Find(Direction::Backward));
            Some(Command::Incomplete)
        }
        't' => {
            *char_pending = Some(CharPendingKind::Till(Direction::Forward));
            Some(Command::Incomplete)
        }
        'T' => {
            *char_pending = Some(CharPendingKind::Till(Direction::Backward));
            Some(Command::Incomplete)
        }
        'p' => Some(Command::Put {
            register: register.unwrap_or(RegisterId::Unnamed),
            before: false,
        }),
        'P' => Some(Command::Put {
            register: register.unwrap_or(RegisterId::Unnamed),
            before: true,
        }),
        'J' => Some(Command::JoinLines { spaces: true }),
        '~' => Some(Command::ToggleCaseChar),
        ':' => Some(Command::CommandMode),
        '/' => Some(Command::SearchMode(Direction::Forward)),
        '?' => Some(Command::SearchMode(Direction::Backward)),
        'x' => Some(delete_motion(
            Operator::Delete,
            Motion::Right,
            count.take(),
            register,
        )),
        'X' => Some(delete_motion(
            Operator::Delete,
            Motion::Left,
            count.take(),
            register,
        )),
        'D' => Some(delete_motion(
            Operator::Delete,
            Motion::LineEnd,
            count.take(),
            register,
        )),
        'Y' => Some(Command::OperatorLine {
            count: count.take(),
            operator: Operator::Yank,
            register: register.unwrap_or(RegisterId::Unnamed),
        }),
        _ => None,
    }
}

/// Helper to create a delete-motion command.
fn delete_motion(
    op: Operator,
    motion: Motion,
    count: Option<usize>,
    reg: Option<RegisterId>,
) -> Command {
    Command::OperatorMotion {
        count,
        operator: op,
        motion,
        register: reg.unwrap_or(RegisterId::Unnamed),
    }
}
