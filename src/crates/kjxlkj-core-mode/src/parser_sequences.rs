//! Multi-key sequence parsing: g, z, leader, operators, text objects.

use kjxlkj_core_types::{
    CaseOp, Intent, KeyCode, KeyEvent, MotionKind, OperatorKind,
    PastePosition, RegisterName, ScrollKind, TextObjectKind,
};

use crate::parser::PendingState;

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

pub(crate) fn parse_operator_motion(
    pending: &mut PendingState,
    key: &KeyEvent,
    op: OperatorKind,
    count: usize,
    char_to_motion: &dyn Fn(char) -> Option<MotionKind>,
) -> Intent {
    *pending = PendingState::None;
    match &key.code {
        KeyCode::Escape => Intent::Noop,
        KeyCode::Char(c) => {
            // Double-operator (dd, yy, cc, >>, <<)
            let double = matches!(
                (op, c),
                (OperatorKind::Delete, 'd')
                    | (OperatorKind::Yank, 'y')
                    | (OperatorKind::Change, 'c')
                    | (OperatorKind::Indent, '>')
                    | (OperatorKind::Outdent, '<')
            );
            if double {
                return Intent::LineOperator(op, count);
            }
            // Text object prefix (i = inner, a = around)
            if *c == 'i' || *c == 'a' {
                let inner = *c == 'i';
                *pending = PendingState::OperatorTextObject(
                    op, inner, count,
                );
                return Intent::Noop;
            }
            if let Some(motion) = char_to_motion(*c) {
                return Intent::Operator(op, motion, count);
            }
            Intent::Noop
        }
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_text_object_key(
    key: &KeyEvent,
    op: OperatorKind,
    inner: bool,
) -> Intent {
    match &key.code {
        KeyCode::Char(c) => {
            let kind = match c {
                'w' => Some(TextObjectKind::Word),
                'W' => Some(TextObjectKind::WORD),
                's' => Some(TextObjectKind::Sentence),
                'p' => Some(TextObjectKind::Paragraph),
                '"' => Some(TextObjectKind::DoubleQuote),
                '\'' => Some(TextObjectKind::SingleQuote),
                '`' => Some(TextObjectKind::BackTick),
                '(' | ')' | 'b' => Some(TextObjectKind::Paren),
                '[' | ']' => Some(TextObjectKind::Bracket),
                '{' | '}' | 'B' => Some(TextObjectKind::Brace),
                '<' | '>' => Some(TextObjectKind::AngleBracket),
                't' => Some(TextObjectKind::Tag),
                _ => None,
            };
            match kind {
                Some(k) => {
                    Intent::OperatorTextObject(op, k, inner)
                }
                None => Intent::Noop,
            }
        }
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_g_operator(
    key: &KeyEvent,
    case_op: CaseOp,
    count: usize,
    char_to_motion: &dyn Fn(char) -> Option<MotionKind>,
) -> Intent {
    match &key.code {
        KeyCode::Char(c) => {
            let double = matches!(
                (case_op, c),
                (CaseOp::Toggle, '~')
                    | (CaseOp::Lower, 'u')
                    | (CaseOp::Upper, 'U')
            );
            if double {
                return Intent::CaseOperatorLine(case_op);
            }
            if let Some(motion) = char_to_motion(*c) {
                return Intent::CaseOperator(
                    case_op, motion, count,
                );
            }
            Intent::Noop
        }
        _ => Intent::Noop,
    }
}
