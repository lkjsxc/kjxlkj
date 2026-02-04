//! Core facade for kjxlkj editor.
//!
//! This crate re-exports core functionality from component crates.

// Re-export types
pub use kjxlkj_core_types::{
    BufferId, BufferVersion, Cursor, EditorEvent, Intent, KeyCode, KeyEvent, KeyModifiers, Mode,
    MotionIntent, Position, Register, RegisterName, ScrollIntent, Selection, SelectionKind,
};

// Re-export text module
pub mod text {
    pub use kjxlkj_core_text::{
        grapheme, word, TextBuffer, RopeSliceExt, WordKind,
    };
}

// Re-export undo module
pub mod undo {
    pub use kjxlkj_core_undo::{Edit, Transaction, UndoHistory};
}

// Re-export edit module
pub mod edit {
    pub use kjxlkj_core_edit::{
        apply_motion, apply_operator, Motion, Operator, TextObject, TextObjectKind,
    };
}

// Re-export mode module
pub mod mode {
    pub use kjxlkj_core_mode::{
        CommandMode, InsertMode, ModeHandler, ModeResult, NormalMode, Parser, ReplaceMode,
        VisualMode, parser::OperatorKind,
    };
}

// Re-export ui module
pub mod ui {
    pub use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};
}

// Re-export state module
pub mod state {
    pub use kjxlkj_core_state::{EditorState, Registers};
}

/// Prelude for common imports.
pub mod prelude {
    pub use crate::{
        BufferId, Cursor, EditorEvent, Intent, KeyCode, KeyEvent, Mode, Position,
        state::EditorState,
        ui::EditorSnapshot,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_imports() {
        // Test that prelude exports work
        let _ = Position::new(0, 0);
        let _ = Mode::Normal;
        let _ = KeyEvent::char('a');
    }

    #[test]
    fn test_editor_state_access() {
        let mut editor = state::EditorState::new();
        assert_eq!(editor.mode(), Mode::Normal);
        editor.load_content("hello");
        assert_eq!(editor.content(), "hello");
    }
}
