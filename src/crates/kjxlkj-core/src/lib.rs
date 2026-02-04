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
    use mode::ModeHandler;

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

    #[test]
    fn test_text_module_exports() {
        let buffer = text::TextBuffer::from_text(BufferId::new(1), "hello\nworld");
        assert_eq!(buffer.line_count(), 2);
    }

    #[test]
    fn test_undo_module_exports() {
        let mut tx = undo::Transaction::new();
        tx.push(undo::Edit::insert(Position::new(0, 0), "test"));
        assert!(!tx.is_empty());
    }

    #[test]
    fn test_edit_module_exports() {
        let motion = edit::Motion::new(MotionIntent::Left, 1);
        assert_eq!(motion.count, 1);
    }

    #[test]
    fn test_mode_module_exports() {
        let parser = mode::Parser::new();
        // Just verify parser can be created
        let _ = parser;
    }

    #[test]
    fn test_ui_module_exports() {
        let viewport = ui::Viewport::new(0, 24, 0, 80);
        assert_eq!(viewport.height, 24);
    }

    #[test]
    fn test_register_types() {
        let name = RegisterName::Unnamed;
        assert_eq!(name.as_char(), '"');
    }

    #[test]
    fn test_selection_types() {
        let sel = Selection::new(Position::new(0, 0), Position::new(1, 5), SelectionKind::Char);
        assert_eq!(sel.start(), Position::new(0, 0));
    }

    #[test]
    fn test_intent_types() {
        let intent = Intent::Nop;
        assert!(matches!(intent, Intent::Nop));
    }

    #[test]
    fn test_scroll_intent() {
        let scroll = ScrollIntent::LineDown;
        assert!(matches!(scroll, ScrollIntent::LineDown));
    }

    #[test]
    fn test_key_code_export() {
        let code = KeyCode::Escape;
        assert!(matches!(code, KeyCode::Escape));
    }

    #[test]
    fn test_key_modifiers_export() {
        let mods = KeyModifiers::CTRL;
        assert!(mods.ctrl);
    }

    #[test]
    fn test_buffer_version_export() {
        let v1 = BufferVersion::new(1);
        let v2 = BufferVersion::new(2);
        assert!(v2 > v1);
    }

    #[test]
    fn test_cursor_export() {
        let cursor = Cursor::new(5, 10);
        assert_eq!(cursor.line(), 5);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_motion_intent_export() {
        let motion = MotionIntent::Right;
        assert!(matches!(motion, MotionIntent::Right));
    }

    #[test]
    fn test_editor_event_export() {
        let event = EditorEvent::Key(KeyEvent::char('x'));
        assert!(matches!(event, EditorEvent::Key(_)));
    }

    #[test]
    fn test_text_object_kind_export() {
        let kind = edit::TextObjectKind::Inner;
        assert!(matches!(kind, edit::TextObjectKind::Inner));
    }

    #[test]
    fn test_operator_export() {
        let op = edit::Operator::Delete;
        assert!(matches!(op, edit::Operator::Delete));
    }

    #[test]
    fn test_normal_mode_export() {
        let mode = mode::NormalMode::new();
        assert_eq!(mode.mode(), Mode::Normal);
    }

    #[test]
    fn test_insert_mode_export() {
        let mode = mode::InsertMode::new();
        assert_eq!(mode.mode(), Mode::Insert);
    }

    #[test]
    fn test_command_mode_export() {
        let mode = mode::CommandMode::new();
        assert_eq!(mode.mode(), Mode::Command);
    }

    #[test]
    fn test_replace_mode_export() {
        let mode = mode::ReplaceMode::new();
        assert_eq!(mode.mode(), Mode::Replace);
    }
}
