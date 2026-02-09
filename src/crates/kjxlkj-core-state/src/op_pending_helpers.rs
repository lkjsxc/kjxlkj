//! Helper functions for operator-pending mode dispatch.

use kjxlkj_core_edit::Motion;
use kjxlkj_core_types::{Key, KeyCode, Modifier, Operator};

/// Check whether the key doubles the pending operator (e.g. `dd`, `yy`).
pub(crate) fn is_doubled(op: Operator, key: &Key) -> bool {
    if key.modifiers != Modifier::NONE {
        return false;
    }
    matches!(
        (op, &key.code),
        (Operator::Delete, KeyCode::Char('d'))
            | (Operator::Change, KeyCode::Char('c'))
            | (Operator::Yank, KeyCode::Char('y'))
            | (Operator::Indent, KeyCode::Char('>'))
            | (Operator::Dedent, KeyCode::Char('<'))
            | (Operator::Reindent, KeyCode::Char('='))
            | (Operator::Format, KeyCode::Char('q'))
            | (Operator::FormatKeepCursor, KeyCode::Char('w'))
            | (Operator::Lowercase, KeyCode::Char('u'))
            | (Operator::Uppercase, KeyCode::Char('U'))
    )
}

/// Map a key press to a motion in operator-pending mode.
pub(crate) fn key_to_motion(key: &Key, count: usize) -> Option<Motion> {
    if key.modifiers != Modifier::NONE {
        return match &key.code {
            KeyCode::Left => Some(Motion::Left(count)),
            KeyCode::Right => Some(Motion::Right(count)),
            KeyCode::Up => Some(Motion::Up(count)),
            KeyCode::Down => Some(Motion::Down(count)),
            _ => None,
        };
    }
    match &key.code {
        KeyCode::Char('h') | KeyCode::Left => Some(Motion::Left(count)),
        KeyCode::Char('j') | KeyCode::Down => Some(Motion::Down(count)),
        KeyCode::Char('k') | KeyCode::Up => Some(Motion::Up(count)),
        KeyCode::Char('l') | KeyCode::Right => Some(Motion::Right(count)),
        KeyCode::Char('w') => Some(Motion::WordForward(count)),
        KeyCode::Char('b') => Some(Motion::WordBackward(count)),
        KeyCode::Char('e') => Some(Motion::WordEndForward(count)),
        KeyCode::Char('0') => Some(Motion::LineStart),
        KeyCode::Char('^') => Some(Motion::FirstNonBlank),
        KeyCode::Char('$') => Some(Motion::LineEnd),
        KeyCode::Char('G') => Some(Motion::LastLine),
        KeyCode::Char('f') => None,
        KeyCode::Char('F') => None,
        KeyCode::Char('t') => None,
        KeyCode::Char('T') => None,
        _ => None,
    }
}
