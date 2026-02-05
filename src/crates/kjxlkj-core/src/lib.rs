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

    #[test]
    fn test_visual_mode_char_export() {
        use mode::VisualMode;
        let mode = VisualMode::char_wise();
        assert_eq!(mode.mode(), Mode::Visual);
    }

    #[test]
    fn test_visual_mode_line_export() {
        use mode::VisualMode;
        let mode = VisualMode::line_wise();
        assert_eq!(mode.mode(), Mode::VisualLine);
    }

    #[test]
    fn test_visual_mode_block_export() {
        use mode::VisualMode;
        let mode = VisualMode::block_wise();
        assert_eq!(mode.mode(), Mode::VisualBlock);
    }

    #[test]
    fn test_undo_history_export() {
        let history = undo::UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_text_buffer_operations() {
        let buffer = text::TextBuffer::from_text(BufferId::new(1), "hello");
        assert_eq!(buffer.line_count(), 1);
    }

    #[test]
    fn test_word_kind_export() {
        use text::WordKind;
        let kind = WordKind::Word;
        assert!(matches!(kind, WordKind::Word));
        let kind2 = WordKind::WORD;
        assert!(matches!(kind2, WordKind::WORD));
    }

    #[test]
    fn test_buffer_id_display() {
        let id = BufferId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_mode_normal_is_normal() {
        assert_eq!(Mode::Normal, Mode::Normal);
    }

    #[test]
    fn test_mode_insert_is_insert() {
        assert_eq!(Mode::Insert, Mode::Insert);
    }

    #[test]
    fn test_mode_visual_is_visual() {
        assert_eq!(Mode::Visual, Mode::Visual);
    }

    #[test]
    fn test_mode_command_is_command() {
        assert_eq!(Mode::Command, Mode::Command);
    }

    #[test]
    fn test_mode_replace_is_replace() {
        assert_eq!(Mode::Replace, Mode::Replace);
    }

    #[test]
    fn test_mode_visualline_is_visualline() {
        assert_eq!(Mode::VisualLine, Mode::VisualLine);
    }

    #[test]
    fn test_mode_visualblock_is_visualblock() {
        assert_eq!(Mode::VisualBlock, Mode::VisualBlock);
    }

    #[test]
    fn test_position_new_zero() {
        let pos = Position::new(0, 0);
        assert_eq!(pos.line, 0);
        assert_eq!(pos.col, 0);
    }

    #[test]
    fn test_position_new_nonzero() {
        let pos = Position::new(10, 20);
        assert_eq!(pos.line, 10);
        assert_eq!(pos.col, 20);
    }

    #[test]
    fn test_cursor_line_col() {
        let cursor = Cursor::new(5, 10);
        assert_eq!(cursor.line(), 5);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_key_event_char_creation() {
        let event = KeyEvent::char('a');
        assert!(matches!(event.code, KeyCode::Char('a')));
    }

    #[test]
    fn test_key_event_escape() {
        let event = KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE);
        assert!(matches!(event.code, KeyCode::Escape));
    }

    #[test]
    fn test_key_event_enter() {
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        assert!(matches!(event.code, KeyCode::Enter));
    }

    #[test]
    fn test_text_buffer_empty() {
        let buffer = text::TextBuffer::from_text(BufferId::new(1), "");
        assert_eq!(buffer.line_count(), 1);
    }

    #[test]
    fn test_undo_transaction_empty() {
        let tx = undo::Transaction::new();
        assert!(tx.is_empty());
    }
}
