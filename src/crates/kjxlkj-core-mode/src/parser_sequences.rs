//! Multi-key sequence parsing: g, z, leader, ctrl-w chords.

use kjxlkj_core_types::{
    CaseOp, Intent, KeyCode, KeyEvent, MotionKind,
    PastePosition, RegisterName, ScrollKind,
};

use crate::pending_state::PendingState;
pub(crate) use crate::parser_operators::{
    parse_operator_motion, parse_text_object_key, parse_g_operator,
};

pub(crate) fn parse_g_sequence(
    pending: &mut PendingState,
    count: usize,
    key: &KeyEvent,
) -> Intent {
    *pending = PendingState::None;
    match &key.code {
        KeyCode::Char('g') => {
            if count > 1 {
                Intent::Motion(MotionKind::GotoLine(count), 1)
            } else {
                Intent::Motion(MotionKind::FileStart, 1)
            }
        }
        KeyCode::Char('_') => {
            Intent::Motion(MotionKind::LastNonBlank, 1)
        }
        KeyCode::Char('m') => {
            Intent::Motion(MotionKind::MiddleOfLine, 1)
        }
        KeyCode::Char('e') => {
            Intent::Motion(MotionKind::WordBackwardEnd, count)
        }
        KeyCode::Char('E') => {
            Intent::Motion(MotionKind::WORDBackwardEnd, count)
        }
        KeyCode::Char('J') => Intent::JoinLines(false, count),
        KeyCode::Char('v') => Intent::ReselectVisual,
        KeyCode::Char('R') => Intent::EnterMode(kjxlkj_core_types::Mode::Replace),
        KeyCode::Char('~') => {
            *pending =
                PendingState::GOperator(CaseOp::Toggle, count);
            Intent::Noop
        }
        KeyCode::Char('u') => {
            *pending =
                PendingState::GOperator(CaseOp::Lower, count);
            Intent::Noop
        }
        KeyCode::Char('U') => {
            *pending =
                PendingState::GOperator(CaseOp::Upper, count);
            Intent::Noop
        }
        KeyCode::Char('p') => Intent::Paste(
            RegisterName::Unnamed,
            PastePosition::AfterCursorEnd,
        ),
        KeyCode::Char('P') => Intent::Paste(
            RegisterName::Unnamed,
            PastePosition::BeforeCursorEnd,
        ),
        KeyCode::Char('*') => Intent::SearchWordForward,
        KeyCode::Char('#') => Intent::SearchWordBackward,
        KeyCode::Char(';') => Intent::ChangeListOlder,
        KeyCode::Char(',') => Intent::ChangeListNewer,
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_z_sequence(
    pending: &mut PendingState,
    key: &KeyEvent,
) -> Intent {
    *pending = PendingState::None;
    match &key.code {
        KeyCode::Char('z') => {
            Intent::Scroll(ScrollKind::CursorCenter)
        }
        KeyCode::Char('t') => {
            Intent::Scroll(ScrollKind::CursorTop)
        }
        KeyCode::Char('b') => {
            Intent::Scroll(ScrollKind::CursorBottom)
        }
        KeyCode::Char('.') => {
            Intent::Scroll(ScrollKind::CursorCenterFirstNonBlank)
        }
        KeyCode::Char('-') => {
            Intent::Scroll(ScrollKind::CursorBottomFirstNonBlank)
        }
        KeyCode::Enter => {
            Intent::Scroll(ScrollKind::CursorTopFirstNonBlank)
        }
        KeyCode::Char('Z') => Intent::ExCommand(":wq".into()),
        KeyCode::Char('Q') => Intent::ExCommand(":q!".into()),
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_leader_chord(key: &KeyEvent) -> Intent {
    match &key.code {
        KeyCode::Char('e') => {
            Intent::ExCommand(":explorer".into())
        }
        KeyCode::Char('t') => {
            Intent::ExCommand(":terminal".into())
        }
        KeyCode::Char('f') => Intent::ExCommand(":find".into()),
        KeyCode::Char('g') => {
            Intent::ExCommand(":livegrep".into())
        }
        KeyCode::Char('b') => Intent::ExCommand(":ls".into()),
        KeyCode::Char('u') => {
            Intent::ExCommand(":undotree".into())
        }
        _ => Intent::Noop,
    }
}

/// Parse Ctrl-w chord for window management.
pub(crate) fn parse_ctrl_w_chord(key: &KeyEvent) -> Intent {
    match &key.code {
        KeyCode::Char('s') => Intent::WindowSplitHorizontal,
        KeyCode::Char('v') => Intent::WindowSplitVertical,
        KeyCode::Char('c') | KeyCode::Char('q') => {
            Intent::WindowClose
        }
        KeyCode::Char('o') => Intent::WindowOnly,
        KeyCode::Char('w') => Intent::WindowFocusNext,
        KeyCode::Char('W') => Intent::WindowFocusPrev,
        KeyCode::Char('h') => {
            Intent::WindowFocusDirection(MotionKind::Left)
        }
        KeyCode::Char('j') => {
            Intent::WindowFocusDirection(MotionKind::Down)
        }
        KeyCode::Char('k') => {
            Intent::WindowFocusDirection(MotionKind::Up)
        }
        KeyCode::Char('l') => {
            Intent::WindowFocusDirection(MotionKind::Right)
        }
        KeyCode::Char('=') => Intent::WindowEqualSize,
        KeyCode::Char('r') => Intent::WindowRotate,
        KeyCode::Char('n') => {
            Intent::ExCommand(":new".into())
        }
        _ => Intent::Noop,
    }
}
