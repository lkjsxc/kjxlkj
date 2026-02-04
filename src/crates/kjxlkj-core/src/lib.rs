//! Core facade crate for kjxlkj editor.
//!
//! This crate re-exports the essential core APIs for the editor.

// Re-export core types
pub use kjxlkj_core_types::{
    BufferId, BufferName, BufferVersion, ByteOffset, CharOffset, Cursor, CursorShape,
    EditorError, EditorResult, LineCol, Mode, Position,
};

// Re-export text model
pub use kjxlkj_core_text::{display_width, grapheme_count, line_graphemes, RopeText};

// Re-export editing primitives
pub use kjxlkj_core_edit::{
    EditOp, EditResult, Motion, MotionKind, Operator, OperatorKind, TextObject, TextObjectKind,
};

// Re-export mode handling
pub use kjxlkj_core_mode::{Intent, IntentKind, Key, KeyCode, KeyModifiers, KeyParser, ModeState};

// Re-export undo
pub use kjxlkj_core_undo::{Transaction, UndoHistory};

// Re-export UI model
pub use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, StatusSection, Viewport};

// Re-export state management
pub use kjxlkj_core_state::{BufferState, EditorState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_exports_work() {
        // Verify types are accessible through facade
        let _mode = Mode::Normal;
        let _cursor = Cursor::origin();
        let _editor = EditorState::new();
    }
}
