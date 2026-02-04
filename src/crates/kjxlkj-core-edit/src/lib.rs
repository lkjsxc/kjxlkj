//! Editing primitives and operators.
//!
//! This crate provides editing operations that work on text buffers.

mod motion;
mod operator;
mod text_object;

pub use motion::{apply_motion, Motion};
pub use operator::{apply_operator, Operator};
pub use text_object::{find_text_object, TextObject, TextObjectKind};

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::MotionIntent;

    #[test]
    fn test_motion_export() {
        let motion = Motion::new(MotionIntent::Left, 1);
        assert_eq!(motion.count, 1);
    }

    #[test]
    fn test_motion_with_count() {
        let motion = Motion::new(MotionIntent::Right, 5);
        assert_eq!(motion.count, 5);
    }

    #[test]
    fn test_motion_min_count() {
        let motion = Motion::new(MotionIntent::Up, 0);
        // Count should be at least 1
        assert_eq!(motion.count, 1);
    }

    #[test]
    fn test_motion_intent() {
        let motion = Motion::new(MotionIntent::Down, 1);
        assert_eq!(motion.intent, MotionIntent::Down);
    }

    #[test]
    fn test_text_object_kind_export() {
        let kind = TextObjectKind::Inner;
        assert!(matches!(kind, TextObjectKind::Inner));
    }

    #[test]
    fn test_text_object_kind_around() {
        let kind = TextObjectKind::Around;
        assert!(matches!(kind, TextObjectKind::Around));
    }

    #[test]
    fn test_text_object_word() {
        let obj = TextObject::Word;
        assert!(matches!(obj, TextObject::Word));
    }

    #[test]
    fn test_motion_clone() {
        let motion = Motion::new(MotionIntent::Left, 2);
        let cloned = motion.clone();
        assert_eq!(motion, cloned);
    }

    #[test]
    fn test_text_object_word_big() {
        let obj = TextObject::WORD;
        assert!(matches!(obj, TextObject::WORD));
    }

    #[test]
    fn test_text_object_paragraph() {
        let obj = TextObject::Paragraph;
        assert!(matches!(obj, TextObject::Paragraph));
    }

    #[test]
    fn test_text_object_sentence() {
        let obj = TextObject::Sentence;
        assert!(matches!(obj, TextObject::Sentence));
    }

    #[test]
    fn test_text_object_parens() {
        let obj = TextObject::Parens;
        assert!(matches!(obj, TextObject::Parens));
    }

    #[test]
    fn test_text_object_brackets() {
        let obj = TextObject::Brackets;
        assert!(matches!(obj, TextObject::Brackets));
    }

    #[test]
    fn test_text_object_braces() {
        let obj = TextObject::Braces;
        assert!(matches!(obj, TextObject::Braces));
    }

    #[test]
    fn test_text_object_angle_brackets() {
        let obj = TextObject::Angles;
        assert!(matches!(obj, TextObject::Angles));
    }

    #[test]
    fn test_motion_line_start() {
        let motion = Motion::new(MotionIntent::LineStart, 1);
        assert_eq!(motion.intent, MotionIntent::LineStart);
    }

    #[test]
    fn test_motion_line_end() {
        let motion = Motion::new(MotionIntent::LineEnd, 1);
        assert_eq!(motion.intent, MotionIntent::LineEnd);
    }

    #[test]
    fn test_motion_word_start() {
        let motion = Motion::new(MotionIntent::WordStart, 1);
        assert_eq!(motion.intent, MotionIntent::WordStart);
    }

    #[test]
    fn test_motion_word_end() {
        let motion = Motion::new(MotionIntent::WordEnd, 1);
        assert_eq!(motion.intent, MotionIntent::WordEnd);
    }
}
