//! Motion type and inclusivity classification.
//!
//! See /docs/spec/editing/operators/linewise-characterwise.md
//! and /docs/spec/editing/operators/exclusive-inclusive.md.

use crate::{Inclusivity, Motion, RangeType};

/// Default range type for a motion.
pub fn motion_range_type(m: &Motion) -> RangeType {
    match m {
        // Linewise motions
        Motion::Up | Motion::Down => RangeType::Linewise,
        Motion::GotoLine(_)
        | Motion::GotoFirstLine
        | Motion::GotoLastLine => RangeType::Linewise,
        Motion::WindowTop
        | Motion::WindowMiddle
        | Motion::WindowBottom => RangeType::Linewise,
        Motion::ParagraphForward
        | Motion::ParagraphBackward => RangeType::Linewise,
        Motion::SentenceForward
        | Motion::SentenceBackward => RangeType::Linewise,
        // Everything else is characterwise
        _ => RangeType::Characterwise,
    }
}

/// Default inclusivity for a motion.
pub fn motion_inclusivity(m: &Motion) -> Inclusivity {
    match m {
        // Inclusive motions
        Motion::WordEndForward
        | Motion::WordEndBackward
        | Motion::BigWordEndForward
        | Motion::BigWordEndBackward => Inclusivity::Inclusive,
        Motion::LineEnd | Motion::LastNonBlank => {
            Inclusivity::Inclusive
        }
        Motion::FindForward(_)
        | Motion::FindBackward(_) => Inclusivity::Inclusive,
        Motion::MatchParen => Inclusivity::Inclusive,
        // Exclusive motions (most motions)
        _ => Inclusivity::Exclusive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_forward_is_exclusive_characterwise() {
        assert_eq!(
            motion_range_type(&Motion::WordForward),
            RangeType::Characterwise,
        );
        assert_eq!(
            motion_inclusivity(&Motion::WordForward),
            Inclusivity::Exclusive,
        );
    }

    #[test]
    fn word_end_is_inclusive() {
        assert_eq!(
            motion_inclusivity(&Motion::WordEndForward),
            Inclusivity::Inclusive,
        );
    }

    #[test]
    fn j_is_linewise() {
        assert_eq!(
            motion_range_type(&Motion::Down),
            RangeType::Linewise,
        );
    }

    #[test]
    fn find_forward_is_inclusive() {
        assert_eq!(
            motion_inclusivity(&Motion::FindForward('x')),
            Inclusivity::Inclusive,
        );
    }

    #[test]
    fn till_forward_is_exclusive() {
        assert_eq!(
            motion_inclusivity(&Motion::TillForward('x')),
            Inclusivity::Exclusive,
        );
    }

    #[test]
    fn gg_is_linewise() {
        assert_eq!(
            motion_range_type(&Motion::GotoFirstLine),
            RangeType::Linewise,
        );
    }

    #[test]
    fn dollar_is_inclusive() {
        assert_eq!(
            motion_inclusivity(&Motion::LineEnd),
            Inclusivity::Inclusive,
        );
    }
}
