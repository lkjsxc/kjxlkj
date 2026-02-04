//! Operator definitions.

use crate::Motion;
use crate::TextObject;

/// An operator kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    /// Delete text.
    Delete,
    /// Change text (delete and enter insert).
    Change,
    /// Yank text.
    Yank,
    /// Indent right.
    Indent,
    /// Indent left.
    Outdent,
    /// Toggle case.
    ToggleCase,
    /// Uppercase.
    Uppercase,
    /// Lowercase.
    Lowercase,
    /// Format.
    Format,
}

/// An operator with target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// Operator with motion.
    WithMotion {
        kind: OperatorKind,
        motion: Motion,
    },
    /// Operator with text object.
    WithTextObject {
        kind: OperatorKind,
        text_object: TextObject,
    },
    /// Line-wise operator (dd, yy, cc, etc).
    Line {
        kind: OperatorKind,
        count: usize,
    },
}

impl Operator {
    /// Create an operator with motion.
    pub fn with_motion(kind: OperatorKind, motion: Motion) -> Self {
        Self::WithMotion { kind, motion }
    }

    /// Create an operator with text object.
    pub fn with_text_object(kind: OperatorKind, text_object: TextObject) -> Self {
        Self::WithTextObject { kind, text_object }
    }

    /// Create a line-wise operator.
    pub fn line(kind: OperatorKind, count: usize) -> Self {
        Self::Line { kind, count }
    }

    /// Get the operator kind.
    pub fn kind(&self) -> OperatorKind {
        match self {
            Self::WithMotion { kind, .. } => *kind,
            Self::WithTextObject { kind, .. } => *kind,
            Self::Line { kind, .. } => *kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MotionKind, TextObjectKind};

    #[test]
    fn operator_with_motion() {
        let op = Operator::with_motion(OperatorKind::Delete, Motion::new(MotionKind::WordStart));
        assert_eq!(op.kind(), OperatorKind::Delete);
    }

    #[test]
    fn operator_line() {
        let op = Operator::line(OperatorKind::Yank, 2);
        if let Operator::Line { kind, count } = op {
            assert_eq!(kind, OperatorKind::Yank);
            assert_eq!(count, 2);
        } else {
            panic!("Expected Line operator");
        }
    }

    #[test]
    fn operator_with_text_object() {
        let to = TextObject::inner(TextObjectKind::Word);
        let op = Operator::with_text_object(OperatorKind::Change, to);
        assert_eq!(op.kind(), OperatorKind::Change);
    }

    #[test]
    fn operator_kind_variants() {
        assert_eq!(OperatorKind::Delete, OperatorKind::Delete);
        assert_eq!(OperatorKind::Change, OperatorKind::Change);
        assert_eq!(OperatorKind::Yank, OperatorKind::Yank);
        assert_eq!(OperatorKind::Indent, OperatorKind::Indent);
        assert_eq!(OperatorKind::Outdent, OperatorKind::Outdent);
    }

    #[test]
    fn operator_debug_format() {
        let op = Operator::line(OperatorKind::Delete, 1);
        let debug = format!("{:?}", op);
        assert!(debug.contains("Delete"));
    }

    #[test]
    fn operator_case_kinds() {
        assert_eq!(OperatorKind::ToggleCase, OperatorKind::ToggleCase);
        assert_eq!(OperatorKind::Uppercase, OperatorKind::Uppercase);
        assert_eq!(OperatorKind::Lowercase, OperatorKind::Lowercase);
    }

    #[test]
    fn operator_format_kind() {
        assert_eq!(OperatorKind::Format, OperatorKind::Format);
    }

    #[test]
    fn operator_clone() {
        let op = Operator::line(OperatorKind::Yank, 3);
        let cloned = op.clone();
        assert_eq!(op, cloned);
    }

    #[test]
    fn operator_equality() {
        let op1 = Operator::line(OperatorKind::Delete, 1);
        let op2 = Operator::line(OperatorKind::Delete, 1);
        assert_eq!(op1, op2);
    }
}
