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
}
