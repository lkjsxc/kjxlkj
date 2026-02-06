//! Operator-related sequence parsing: motion targets, text objects, case operators.

use kjxlkj_core_types::{
    CaseOp, Intent, KeyCode, KeyEvent, MotionKind, OperatorKind,
    TextObjectKind,
};
use crate::pending_state::PendingState;

pub(crate) fn parse_operator_motion(
    pending: &mut PendingState, key: &KeyEvent,
    op: OperatorKind, count: usize,
    char_to_motion: &dyn Fn(char) -> Option<MotionKind>,
) -> Intent {
    *pending = PendingState::None;
    match &key.code {
        KeyCode::Escape => Intent::Noop,
        KeyCode::Char(c) => {
            let double = matches!(
                (op, c),
                (OperatorKind::Delete, 'd') | (OperatorKind::Yank, 'y')
                    | (OperatorKind::Change, 'c') | (OperatorKind::Indent, '>')
                    | (OperatorKind::Outdent, '<')
            );
            if double { return Intent::LineOperator(op, count); }
            if *c == 'i' || *c == 'a' {
                let inner = *c == 'i';
                *pending = PendingState::OperatorTextObject(op, inner, count);
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
    key: &KeyEvent, op: OperatorKind, inner: bool,
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
                Some(k) => Intent::OperatorTextObject(op, k, inner),
                None => Intent::Noop,
            }
        }
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_g_operator(
    key: &KeyEvent, case_op: CaseOp, count: usize,
    char_to_motion: &dyn Fn(char) -> Option<MotionKind>,
) -> Intent {
    match &key.code {
        KeyCode::Char(c) => {
            let double = matches!(
                (case_op, c),
                (CaseOp::Toggle, '~') | (CaseOp::Lower, 'u') | (CaseOp::Upper, 'U')
            );
            if double { return Intent::CaseOperatorLine(case_op); }
            if let Some(motion) = char_to_motion(*c) {
                return Intent::CaseOperator(case_op, motion, count);
            }
            Intent::Noop
        }
        _ => Intent::Noop,
    }
}
