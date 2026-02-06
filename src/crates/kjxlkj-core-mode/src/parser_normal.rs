//! Normal mode key parsing: single char and Ctrl keys.

use kjxlkj_core_types::{
    FindCharKind, InsertPosition, Intent, KeyCode, KeyEvent, Mode,
    MotionKind, OperatorKind, PastePosition, RegisterName, ScrollKind,
};

use crate::parser::PendingState;

pub(crate) fn parse_normal_key(
    pending: &mut PendingState,
    count_buf: &mut Option<usize>,
    key: &KeyEvent,
    count: usize,
) -> Intent {
    if key.ctrl {
        // Ctrl-w enters window command pending state
        if key.code == KeyCode::Char('w') {
            *pending = PendingState::CtrlW;
            return Intent::Noop;
        }
        return parse_ctrl_key(key, count);
    }
    match &key.code {
        KeyCode::Escape => Intent::EnterMode(Mode::Normal),
        KeyCode::Enter => {
            Intent::Motion(MotionKind::NextNonBlankLine, count)
        }
        KeyCode::Backspace => Intent::Motion(MotionKind::Left, count),
        KeyCode::Left => Intent::Motion(MotionKind::Left, count),
        KeyCode::Right => Intent::Motion(MotionKind::Right, count),
        KeyCode::Up => Intent::Motion(MotionKind::Up, count),
        KeyCode::Down => Intent::Motion(MotionKind::Down, count),
        KeyCode::Char(c) => {
            parse_normal_char(pending, count_buf, *c, count)
        }
        _ => Intent::Noop,
    }
}

fn parse_normal_char(
    pending: &mut PendingState,
    count_buf: &mut Option<usize>,
    c: char,
    count: usize,
) -> Intent {
    match c {
        'h' => Intent::Motion(MotionKind::Left, count),
        'j' => Intent::Motion(MotionKind::Down, count),
        'k' => Intent::Motion(MotionKind::Up, count),
        'l' => Intent::Motion(MotionKind::Right, count),
        'w' => Intent::Motion(MotionKind::WordForward, count),
        'W' => Intent::Motion(MotionKind::WORDForward, count),
        'b' => Intent::Motion(MotionKind::WordBackward, count),
        'B' => Intent::Motion(MotionKind::WORDBackward, count),
        'e' => Intent::Motion(MotionKind::WordForwardEnd, count),
        'E' => Intent::Motion(MotionKind::WORDForwardEnd, count),
        '$' => Intent::Motion(MotionKind::LineEnd, 1),
        '^' | '_' => Intent::Motion(MotionKind::FirstNonBlank, 1),
        'G' => {
            if count > 1 {
                Intent::Motion(MotionKind::GotoLine(count), 1)
            } else {
                Intent::Motion(MotionKind::FileEnd, 1)
            }
        }
        '%' => {
            if count > 1 && count <= 100 {
                Intent::Motion(MotionKind::GotoPercent(count), 1)
            } else {
                Intent::Motion(MotionKind::MatchingBracket, 1)
            }
        }
        '|' => Intent::Motion(MotionKind::GotoColumn(count), 1),
        '+' => Intent::Motion(MotionKind::NextNonBlankLine, count),
        '-' => Intent::Motion(MotionKind::PrevNonBlankLine, count),
        'H' => Intent::Motion(MotionKind::ScreenTop, 1),
        'M' => Intent::Motion(MotionKind::ScreenMiddle, 1),
        'L' => Intent::Motion(MotionKind::ScreenBottom, 1),
        '{' => Intent::Motion(MotionKind::PrevParagraph, count),
        '}' => Intent::Motion(MotionKind::NextParagraph, count),
        '(' => Intent::Motion(MotionKind::PrevSentence, count),
        ')' => Intent::Motion(MotionKind::NextSentence, count),
        'i' => Intent::EnterInsert(InsertPosition::BeforeCursor),
        'I' => Intent::EnterInsert(InsertPosition::FirstNonBlank),
        'a' => Intent::EnterInsert(InsertPosition::AfterCursor),
        'A' => Intent::EnterInsert(InsertPosition::EndOfLine),
        'o' => Intent::OpenLine(true),
        'O' => Intent::OpenLine(false),
        'v' => Intent::EnterMode(Mode::Visual),
        'V' => Intent::EnterMode(Mode::VisualLine),
        'R' => Intent::EnterMode(Mode::Replace),
        ':' | '/' | '?' => Intent::EnterMode(Mode::Command),
        'x' => Intent::DeleteCharAt,
        'X' => Intent::DeleteCharBefore,
        'D' => Intent::DeleteToEnd,
        'C' => Intent::ChangeToEnd,
        's' => Intent::SubstituteChar,
        'S' => Intent::SubstituteLine,
        'Y' => Intent::YankLine(count),
        'p' => Intent::Paste(RegisterName::Unnamed, PastePosition::After),
        'P' => Intent::Paste(RegisterName::Unnamed, PastePosition::Before),
        'u' => Intent::Undo,
        '.' => Intent::RepeatLastChange,
        '*' => Intent::SearchWordForward,
        '#' => Intent::SearchWordBackward,
        'n' => Intent::SearchNext,
        'N' => Intent::SearchPrev,
        '~' => Intent::ToggleCase,
        'J' => Intent::JoinLines(true, count),
        'd' => {
            *pending = PendingState::Operator(OperatorKind::Delete, count);
            Intent::Noop
        }
        'y' => {
            *pending = PendingState::Operator(OperatorKind::Yank, count);
            Intent::Noop
        }
        'c' => {
            *pending = PendingState::Operator(OperatorKind::Change, count);
            Intent::Noop
        }
        '>' => {
            *pending = PendingState::Operator(OperatorKind::Indent, count);
            Intent::Noop
        }
        '<' => {
            *pending = PendingState::Operator(OperatorKind::Outdent, count);
            Intent::Noop
        }
        'g' => {
            *pending = PendingState::G;
            *count_buf = if count > 1 { Some(count) } else { None };
            Intent::Noop
        }
        'z' | 'Z' => {
            *pending = PendingState::Z;
            Intent::Noop
        }
        'r' => {
            *pending = PendingState::ReplaceChar;
            Intent::Noop
        }
        'f' => {
            *pending = PendingState::FindChar(FindCharKind::Forward);
            Intent::Noop
        }
        'F' => {
            *pending = PendingState::FindChar(FindCharKind::Backward);
            Intent::Noop
        }
        't' => {
            *pending = PendingState::FindChar(FindCharKind::TillForward);
            Intent::Noop
        }
        'T' => {
            *pending = PendingState::FindChar(FindCharKind::TillBackward);
            Intent::Noop
        }
        ';' => Intent::RepeatFindChar,
        ',' => Intent::RepeatFindCharReverse,
        '"' => {
            *pending = PendingState::Register;
            Intent::Noop
        }
        'm' => {
            *pending = PendingState::Mark;
            Intent::Noop
        }
        '`' => {
            *pending = PendingState::JumpMark;
            Intent::Noop
        }
        '\'' => {
            *pending = PendingState::JumpMarkLine;
            Intent::Noop
        }
        'q' => {
            *pending = PendingState::MacroRecord;
            Intent::Noop
        }
        '@' => {
            *pending = PendingState::MacroPlay;
            Intent::Noop
        }
        ' ' => {
            *pending = PendingState::Leader;
            Intent::Noop
        }
        _ => Intent::Noop,
    }
}

pub(crate) fn parse_ctrl_key(
    key: &KeyEvent,
    count: usize,
) -> Intent {
    match &key.code {
        KeyCode::Char('r') => Intent::Redo,
        KeyCode::Char('d') => Intent::Scroll(ScrollKind::HalfPageDown),
        KeyCode::Char('u') => Intent::Scroll(ScrollKind::HalfPageUp),
        KeyCode::Char('f') => Intent::Scroll(ScrollKind::FullPageDown),
        KeyCode::Char('b') => Intent::Scroll(ScrollKind::FullPageUp),
        KeyCode::Char('e') => Intent::Scroll(ScrollKind::LineDown),
        KeyCode::Char('y') => Intent::Scroll(ScrollKind::LineUp),
        KeyCode::Char('o') => Intent::JumpListBack,
        KeyCode::Char('i') => Intent::JumpListForward,
        KeyCode::Char('a') => Intent::IncrementNumber(count as i64),
        KeyCode::Char('x') => {
            Intent::IncrementNumber(-(count as i64))
        }
        KeyCode::Char('v') => Intent::EnterMode(Mode::VisualBlock),
        KeyCode::Char('g') => Intent::ExCommand(":file".into()),
        _ => Intent::Noop,
    }
}
