//! Character commands for normal mode.

use crate::command::{InsertVariant, VisualVariant};
use kjxlkj_core_types::{
    motion::{Direction, Motion},
    operator::Operator,
};

/// Parses simple motion characters.
pub fn parse_motion_char(c: char) -> Option<Motion> {
    match c {
        'h' => Some(Motion::Left),
        'j' => Some(Motion::Down),
        'k' => Some(Motion::Up),
        'l' => Some(Motion::Right),
        'w' => Some(Motion::WordForward),
        'W' => Some(Motion::BigWordForward),
        'b' => Some(Motion::WordBackward),
        'B' => Some(Motion::BigWordBackward),
        'e' => Some(Motion::WordEnd),
        'E' => Some(Motion::BigWordEnd),
        '0' => Some(Motion::FirstColumn),
        '^' => Some(Motion::FirstNonBlank),
        '$' => Some(Motion::LineEnd),
        '{' => Some(Motion::ParagraphBackward),
        '}' => Some(Motion::ParagraphForward),
        '(' => Some(Motion::SentenceBackward),
        ')' => Some(Motion::SentenceForward),
        '%' => Some(Motion::MatchingBracket),
        ';' => Some(Motion::RepeatFindChar),
        ',' => Some(Motion::RepeatFindCharReverse),
        'n' => Some(Motion::NextSearchResult),
        'N' => Some(Motion::PrevSearchResult),
        _ => None,
    }
}

/// Parses insert mode entry characters.
pub fn parse_insert_char(c: char) -> Option<InsertVariant> {
    match c {
        'i' => Some(InsertVariant::Before),
        'I' => Some(InsertVariant::StartOfLine),
        'a' => Some(InsertVariant::After),
        'A' => Some(InsertVariant::EndOfLine),
        'o' => Some(InsertVariant::OpenBelow),
        'O' => Some(InsertVariant::OpenAbove),
        's' => Some(InsertVariant::Substitute),
        'S' => Some(InsertVariant::SubstituteLine),
        'C' => Some(InsertVariant::ChangeToEnd),
        _ => None,
    }
}

/// Parses operator characters.
pub fn parse_operator_char(c: char) -> Option<Operator> {
    match c {
        'd' => Some(Operator::Delete),
        'c' => Some(Operator::Change),
        'y' => Some(Operator::Yank),
        '>' => Some(Operator::IndentRight),
        '<' => Some(Operator::IndentLeft),
        _ => None,
    }
}

/// Parses visual mode entry characters.
pub fn parse_visual_char(c: char) -> Option<VisualVariant> {
    match c {
        'v' => Some(VisualVariant::Char),
        'V' => Some(VisualVariant::Line),
        _ => None,
    }
}

/// Parses search motion characters.
pub fn parse_search_motion(c: char) -> Option<Motion> {
    match c {
        '*' => Some(Motion::SearchWordUnderCursor { direction: Direction::Forward }),
        '#' => Some(Motion::SearchWordUnderCursor { direction: Direction::Backward }),
        _ => None,
    }
}
