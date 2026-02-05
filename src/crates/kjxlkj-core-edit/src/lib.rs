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

    #[test]
    fn test_motion_first_non_blank() {
        let motion = Motion::new(MotionIntent::FirstNonBlank, 1);
        assert_eq!(motion.intent, MotionIntent::FirstNonBlank);
    }

    #[test]
    fn test_motion_file_start() {
        let motion = Motion::new(MotionIntent::FileStart, 1);
        assert_eq!(motion.intent, MotionIntent::FileStart);
    }

    #[test]
    fn test_motion_file_end() {
        let motion = Motion::new(MotionIntent::FileEnd, 1);
        assert_eq!(motion.intent, MotionIntent::FileEnd);
    }

    #[test]
    fn test_text_object_double_quotes() {
        let obj = TextObject::DoubleQuotes;
        assert!(matches!(obj, TextObject::DoubleQuotes));
    }

    #[test]
    fn test_text_object_single_quotes() {
        let obj = TextObject::SingleQuotes;
        assert!(matches!(obj, TextObject::SingleQuotes));
    }

    #[test]
    fn test_text_object_backticks() {
        let obj = TextObject::Backticks;
        assert!(matches!(obj, TextObject::Backticks));
    }

    #[test]
    fn test_motion_debug() {
        let motion = Motion::new(MotionIntent::Left, 1);
        let debug = format!("{:?}", motion);
        assert!(debug.contains("Motion"));
    }

    #[test]
    fn test_text_object_kind_debug() {
        let kind = TextObjectKind::Inner;
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Inner"));
    }

    #[test]
    fn test_text_object_clone() {
        let obj = TextObject::Word;
        let cloned = obj;
        assert_eq!(obj, cloned);
    }

    #[test]
    fn test_text_object_kind_clone() {
        let kind = TextObjectKind::Around;
        let cloned = kind;
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_motion_equality() {
        let m1 = Motion::new(MotionIntent::Left, 2);
        let m2 = Motion::new(MotionIntent::Left, 2);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_motion_inequality() {
        let m1 = Motion::new(MotionIntent::Left, 2);
        let m2 = Motion::new(MotionIntent::Right, 2);
        assert_ne!(m1, m2);
    }

    #[test]
    fn test_text_object_equality() {
        assert_eq!(TextObject::Word, TextObject::Word);
        assert_ne!(TextObject::Word, TextObject::WORD);
    }

    #[test]
    fn test_text_object_kind_equality() {
        assert_eq!(TextObjectKind::Inner, TextObjectKind::Inner);
        assert_ne!(TextObjectKind::Inner, TextObjectKind::Around);
    }

    #[test]
    fn test_motion_word_start_back() {
        let motion = Motion::new(MotionIntent::WordStartBack, 1);
        assert_eq!(motion.intent, MotionIntent::WordStartBack);
    }

    #[test]
    fn test_motion_word_end_back() {
        let motion = Motion::new(MotionIntent::WordEndBack, 1);
        assert_eq!(motion.intent, MotionIntent::WordEndBack);
    }

    #[test]
    fn test_motion_screen_top() {
        let motion = Motion::new(MotionIntent::ScreenTop, 1);
        assert_eq!(motion.intent, MotionIntent::ScreenTop);
    }

    #[test]
    fn test_motion_screen_middle() {
        let motion = Motion::new(MotionIntent::ScreenMiddle, 1);
        assert_eq!(motion.intent, MotionIntent::ScreenMiddle);
    }

    #[test]
    fn test_motion_screen_bottom() {
        let motion = Motion::new(MotionIntent::ScreenBottom, 1);
        assert_eq!(motion.intent, MotionIntent::ScreenBottom);
    }

    #[test]
    fn test_text_object_tag() {
        let obj = TextObject::Tag;
        assert!(matches!(obj, TextObject::Tag));
    }

    #[test]
    fn test_motion_clone_again() {
        let motion = Motion::new(MotionIntent::Up, 3);
        let cloned = motion.clone();
        assert_eq!(motion, cloned);
    }

    #[test]
    fn test_text_object_kind_copy() {
        let kind = TextObjectKind::Inner;
        let copied = kind; // Copy
        assert_eq!(kind, copied);
    }

    #[test]
    fn test_text_object_copy() {
        let obj = TextObject::Word;
        let copied = obj; // Copy
        assert_eq!(obj, copied);
    }
}
