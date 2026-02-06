//! Pending parse state and shared helpers.

use kjxlkj_core_types::{CaseOp, FindCharKind, MotionKind, OperatorKind};

/// Pending parse state for multi-key sequences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PendingState {
    None, Count(usize),
    Operator(OperatorKind, usize),
    OperatorTextObject(OperatorKind, bool, usize),
    G, GOperator(CaseOp, usize), Z,
    FindChar(FindCharKind), Register,
    Mark, JumpMark, JumpMarkLine,
    MacroRecord, MacroPlay, ReplaceChar,
    InsertRegister, InsertDigraph1, InsertDigraph2(char),
    Leader, CtrlW,
}

/// Shared char-to-motion mapping (stateless).
pub(crate) fn char_to_motion(c: char) -> Option<MotionKind> {
    match c {
        'h' => Some(MotionKind::Left),
        'l' => Some(MotionKind::Right),
        'j' => Some(MotionKind::Down),
        'k' => Some(MotionKind::Up),
        'w' => Some(MotionKind::WordForward),
        'W' => Some(MotionKind::WORDForward),
        'b' => Some(MotionKind::WordBackward),
        'B' => Some(MotionKind::WORDBackward),
        'e' => Some(MotionKind::WordForwardEnd),
        'E' => Some(MotionKind::WORDForwardEnd),
        '0' => Some(MotionKind::LineStart),
        '$' => Some(MotionKind::LineEnd),
        '^' | '_' => Some(MotionKind::FirstNonBlank),
        'G' => Some(MotionKind::FileEnd),
        '{' => Some(MotionKind::PrevParagraph),
        '}' => Some(MotionKind::NextParagraph),
        '(' => Some(MotionKind::PrevSentence),
        ')' => Some(MotionKind::NextSentence),
        '%' => Some(MotionKind::MatchingBracket),
        _ => None,
    }
}
