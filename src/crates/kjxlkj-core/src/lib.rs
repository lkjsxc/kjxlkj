//! Core facade crate - exports editor-facing core APIs.

// Re-export core types
pub use kjxlkj_core_types::{
    BufferId, BufferVersion, Cursor, Mode, Position, Register, RegisterName, Selection,
    SelectionKind,
};

// Re-export text model
pub use kjxlkj_core_text::TextBuffer;

// Re-export editing primitives
pub use kjxlkj_core_edit::{Motion, MotionKind, Operator, OperatorKind, TextObject, TextObjectKind};

// Re-export mode handling
pub use kjxlkj_core_mode::{Intent, ModeState, NormalModeState};

// Re-export undo
pub use kjxlkj_core_undo::{Edit, EditKind, Transaction, UndoHistory};

// Re-export UI types
pub use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};

// Re-export state
pub use kjxlkj_core_state::{EditorState, MarkStore, RegisterStore};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_exports_types() {
        let _pos = Position::new(0, 0);
        let _mode = Mode::Normal;
        let _cursor = Cursor::origin();
    }

    #[test]
    fn facade_exports_state() {
        let state = EditorState::new();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn facade_exports_selection() {
        let sel = Selection::new(Position::new(0, 0), Position::new(0, 1), SelectionKind::Char);
        assert_eq!(sel.anchor.line, 0);
    }

    #[test]
    fn facade_exports_register() {
        let _name = RegisterName::Unnamed;
        let reg = Register::new();
        assert!(reg.content.is_empty());
    }

    #[test]
    fn facade_exports_viewport() {
        let vp = Viewport::new(80, 24);
        assert_eq!(vp.height, 24);
        assert_eq!(vp.width, 80);
    }

    #[test]
    fn facade_exports_motion() {
        let motion = Motion::new(MotionKind::Right);
        assert_eq!(motion.kind, MotionKind::Right);
    }

    #[test]
    fn facade_exports_operator() {
        let op = Operator::line(OperatorKind::Delete, 1);
        assert_eq!(op.kind(), OperatorKind::Delete);
    }

    #[test]
    fn facade_mode_variants() {
        let modes = [
            Mode::Normal,
            Mode::Insert,
            Mode::Visual,
            Mode::VisualLine,
            Mode::Command,
            Mode::Replace,
        ];
        assert_eq!(modes.len(), 6);
    }

    #[test]
    fn facade_motion_kinds() {
        let kinds = [
            MotionKind::Left,
            MotionKind::Right,
            MotionKind::Up,
            MotionKind::Down,
        ];
        assert_eq!(kinds.len(), 4);
    }

    #[test]
    fn facade_operator_kinds() {
        let kinds = [
            OperatorKind::Delete,
            OperatorKind::Yank,
            OperatorKind::Change,
        ];
        assert_eq!(kinds.len(), 3);
    }
}
