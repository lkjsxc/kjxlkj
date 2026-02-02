//! Unit tests for editing primitives.

use super::*;
use kjxlkj_core_types::{BufferId, BufferVersion, Position, Range};

#[cfg(test)]
mod motion_tests {
    use super::*;

    #[test]
    fn test_motion_new() {
        let motion = Motion::new(MotionKind::Left);
        assert_eq!(motion.kind, MotionKind::Left);
        assert_eq!(motion.count, 1);
    }

    #[test]
    fn test_motion_with_count() {
        let motion = Motion::new(MotionKind::Right).with_count(5);
        assert_eq!(motion.count, 5);
    }

    #[test]
    fn test_motion_with_char() {
        let motion = Motion::new(MotionKind::FindChar).with_char('x');
        assert_eq!(motion.char_arg, Some('x'));
    }

    #[test]
    fn test_motion_is_inclusive() {
        assert!(MotionKind::Right.is_inclusive());
        assert!(MotionKind::WordEnd.is_inclusive());
        assert!(!MotionKind::Left.is_inclusive());
        assert!(!MotionKind::WordStart.is_inclusive());
    }

    #[test]
    fn test_motion_is_linewise() {
        assert!(MotionKind::Up.is_linewise());
        assert!(MotionKind::Down.is_linewise());
        assert!(!MotionKind::Left.is_linewise());
    }
}

#[cfg(test)]
mod operator_tests {
    use super::*;

    #[test]
    fn test_operator_new() {
        let op = Operator::new(OperatorKind::Delete);
        assert_eq!(op.kind, OperatorKind::Delete);
        assert_eq!(op.count, 1);
        assert!(op.register.is_none());
    }

    #[test]
    fn test_operator_with_register() {
        let op = Operator::new(OperatorKind::Yank).with_register('a');
        assert_eq!(op.register, Some('a'));
    }

    #[test]
    fn test_operator_with_count() {
        let op = Operator::new(OperatorKind::Delete).with_count(3);
        assert_eq!(op.count, 3);
    }
}

#[cfg(test)]
mod text_object_tests {
    use super::*;

    #[test]
    fn test_text_object_inner() {
        let obj = TextObject::inner(TextObjectKind::Word);
        assert_eq!(obj.kind, TextObjectKind::Word);
        assert_eq!(obj.modifier, TextObjectModifier::Inner);
        assert!(obj.is_inner());
    }

    #[test]
    fn test_text_object_around() {
        let obj = TextObject::around(TextObjectKind::Word);
        assert_eq!(obj.modifier, TextObjectModifier::Around);
        assert!(!obj.is_inner());
    }

    #[test]
    fn test_text_object_with_count() {
        let obj = TextObject::inner(TextObjectKind::Paragraph).with_count(3);
        assert_eq!(obj.count, 3);
    }
}

#[cfg(test)]
mod edit_tests {
    use super::*;

    #[test]
    fn test_edit_insert() {
        let buf_id = BufferId::new(1);
        let edit = Edit::insert(buf_id, Position::new(0, 5), "hello");
        assert!(matches!(edit.kind, EditKind::Insert { .. }));
        assert_eq!(edit.buffer_id.raw(), 1);
    }

    #[test]
    fn test_edit_delete() {
        let buf_id = BufferId::new(1);
        let range = Range::from_coords(0, 0, 0, 5);
        let edit = Edit::delete(buf_id, range);
        assert!(matches!(edit.kind, EditKind::Delete));
    }

    #[test]
    fn test_edit_replace() {
        let buf_id = BufferId::new(1);
        let range = Range::from_coords(0, 0, 0, 5);
        let edit = Edit::replace(buf_id, range, "world");
        assert!(matches!(edit.kind, EditKind::Replace { .. }));
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;

    #[test]
    fn test_transaction_new() {
        let tx = Transaction::new(BufferVersion::initial());
        assert!(tx.is_empty());
    }

    #[test]
    fn test_transaction_push() {
        let mut tx = Transaction::new(BufferVersion::initial());
        let edit = Edit::insert(BufferId::new(1), Position::origin(), "a");
        tx.push(edit);
        assert!(!tx.is_empty());
        assert_eq!(tx.edits.len(), 1);
    }
}
