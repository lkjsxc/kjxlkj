//! Motion key mapping.

use kjxlkj_core_edit::MotionKind;

/// Map a key to a motion kind.
pub fn key_to_motion(key: char) -> Option<MotionKind> {
    match key {
        'h' => Some(MotionKind::Left),
        'l' => Some(MotionKind::Right),
        'j' => Some(MotionKind::Down),
        'k' => Some(MotionKind::Up),
        'w' => Some(MotionKind::WordStart),
        'W' => Some(MotionKind::WordStart),
        'b' => Some(MotionKind::WordStartBackward),
        'B' => Some(MotionKind::WordStartBackward),
        'e' => Some(MotionKind::WordEnd),
        'E' => Some(MotionKind::WordEnd),
        '0' => Some(MotionKind::LineStart),
        '^' => Some(MotionKind::FirstNonBlank),
        '$' => Some(MotionKind::LineEnd),
        'G' => Some(MotionKind::FileEnd),
        '{' => Some(MotionKind::ParagraphBackward),
        '}' => Some(MotionKind::ParagraphForward),
        '(' => Some(MotionKind::SentenceBackward),
        ')' => Some(MotionKind::SentenceForward),
        '%' => Some(MotionKind::MatchingBracket),
        _ => None,
    }
}
