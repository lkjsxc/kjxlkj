//! Mode-specific key parsing: insert, visual, command, replace.

use kjxlkj_core_types::{
    Intent, KeyCode, KeyEvent, Mode, MotionKind, OperatorKind,
};

pub(crate) fn parse_insert(key: &KeyEvent) -> Intent {
    if key.code == KeyCode::Escape {
        return Intent::EnterMode(Mode::Normal);
    }
    if key.ctrl {
        return match &key.code {
            KeyCode::Char('h') => Intent::DeleteCharBefore,
            KeyCode::Char('w') => Intent::DeleteWordBefore,
            KeyCode::Char('u') => Intent::DeleteToLineStart,
            KeyCode::Char('j') | KeyCode::Char('m') => {
                Intent::InsertNewline
            }
            KeyCode::Char('t') => Intent::Indent(true, 1),
            KeyCode::Char('d') => Intent::Indent(false, 1),
            KeyCode::Char('o') => {
                Intent::EnterMode(Mode::InsertNormal)
            }
            _ => Intent::Noop,
        };
    }
    match &key.code {
        KeyCode::Char(c) => Intent::InsertChar(*c),
        KeyCode::Enter => Intent::InsertNewline,
        KeyCode::Backspace => Intent::DeleteCharBefore,
        KeyCode::Delete => Intent::DeleteCharAt,
        KeyCode::Tab => Intent::InsertChar('\t'),
        KeyCode::Left => Intent::Motion(MotionKind::Left, 1),
        KeyCode::Right => Intent::Motion(MotionKind::Right, 1),
        KeyCode::Up => Intent::Motion(MotionKind::Up, 1),
        KeyCode::Down => Intent::Motion(MotionKind::Down, 1),
        KeyCode::Home => Intent::Motion(MotionKind::LineStart, 1),
        KeyCode::End => Intent::Motion(MotionKind::LineEnd, 1),
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_visual(key: &KeyEvent) -> Intent {
    if key.code == KeyCode::Escape {
        return Intent::EnterMode(Mode::Normal);
    }
    if key.ctrl {
        return crate::parser_normal::parse_ctrl_key(key, 1);
    }
    match &key.code {
        KeyCode::Char('h') | KeyCode::Left => {
            Intent::Motion(MotionKind::Left, 1)
        }
        KeyCode::Char('j') | KeyCode::Down => {
            Intent::Motion(MotionKind::Down, 1)
        }
        KeyCode::Char('k') | KeyCode::Up => {
            Intent::Motion(MotionKind::Up, 1)
        }
        KeyCode::Char('l') | KeyCode::Right => {
            Intent::Motion(MotionKind::Right, 1)
        }
        KeyCode::Char('w') => {
            Intent::Motion(MotionKind::WordForward, 1)
        }
        KeyCode::Char('b') => {
            Intent::Motion(MotionKind::WordBackward, 1)
        }
        KeyCode::Char('e') => {
            Intent::Motion(MotionKind::WordForwardEnd, 1)
        }
        KeyCode::Char('0') => {
            Intent::Motion(MotionKind::LineStart, 1)
        }
        KeyCode::Char('^') => {
            Intent::Motion(MotionKind::FirstNonBlank, 1)
        }
        KeyCode::Char('$') => {
            Intent::Motion(MotionKind::LineEnd, 1)
        }
        KeyCode::Char('G') => {
            Intent::Motion(MotionKind::FileEnd, 1)
        }
        KeyCode::Char('d') | KeyCode::Char('x') => {
            Intent::Operator(
                OperatorKind::Delete,
                MotionKind::Right,
                1,
            )
        }
        KeyCode::Char('y') => Intent::Operator(
            OperatorKind::Yank,
            MotionKind::Right,
            1,
        ),
        KeyCode::Char('c') | KeyCode::Char('s') => {
            Intent::Operator(
                OperatorKind::Change,
                MotionKind::Right,
                1,
            )
        }
        KeyCode::Char('>') => Intent::Indent(true, 1),
        KeyCode::Char('<') => Intent::Indent(false, 1),
        KeyCode::Char('o') => Intent::VisualSwapEnd,
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_command(key: &KeyEvent) -> Intent {
    match &key.code {
        KeyCode::Escape => Intent::EnterMode(Mode::Normal),
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_replace(key: &KeyEvent) -> Intent {
    if key.code == KeyCode::Escape {
        return Intent::EnterMode(Mode::Normal);
    }
    match &key.code {
        KeyCode::Char(c) => Intent::ReplaceInsert(*c),
        KeyCode::Backspace => Intent::Motion(MotionKind::Left, 1),
        _ => Intent::Noop,
    }
}
