//! Editor state aggregation and snapshot production.
//!
//! This crate holds the complete editor state and produces snapshots for rendering.

pub mod buffer_manager;
mod editor;
mod registers;
pub mod session;

pub use buffer_manager::{
    AlternateFile, ArgList, BufferFlags, BufferGroup, BufferInfo, BufferManager, BufferState,
};
pub use editor::EditorState;
pub use registers::Registers;
pub use session::{
    AutoSaveConfig, AutoSaveState, RecentFile, RecentFiles, RecentFilesConfig, SessionConfig,
    SwapFile, UndoFile,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state_export() {
        let state = EditorState::new();
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
    }

    #[test]
    fn test_registers_export() {
        let regs = Registers::new();
        assert_eq!(regs.selected(), kjxlkj_core_types::RegisterName::Unnamed);
    }

    #[test]
    fn test_editor_and_registers() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert!(!snapshot.buffer.lines.is_empty() || snapshot.buffer.line_count == 1);
    }

    #[test]
    fn test_editor_state_cursor_initial() {
        let state = EditorState::new();
        assert_eq!(state.cursor().line(), 0);
        assert_eq!(state.cursor().col(), 0);
    }

    #[test]
    fn test_editor_state_should_not_quit_initially() {
        let state = EditorState::new();
        assert!(!state.should_quit());
    }

    #[test]
    fn test_registers_default_impl() {
        let regs = Registers::default();
        assert_eq!(regs.selected(), kjxlkj_core_types::RegisterName::Unnamed);
    }

    #[test]
    fn test_editor_state_default_impl() {
        let state = EditorState::default();
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
    }

    #[test]
    fn test_editor_state_buffer_access() {
        let state = EditorState::new();
        let _buffer = state.buffer();
        // Access works without panic
    }

    #[test]
    fn test_editor_state_content_initially_empty() {
        let state = EditorState::new();
        assert!(state.content().is_empty() || state.content().is_empty());
    }

    #[test]
    fn test_editor_load_content() {
        let mut state = EditorState::new();
        state.load_content("hello world");
        assert!(state.content().contains("hello"));
    }

    #[test]
    fn test_editor_load_multiline() {
        let mut state = EditorState::new();
        state.load_content("line1\nline2\nline3");
        let snapshot = state.snapshot();
        assert!(snapshot.buffer.line_count >= 3);
    }

    #[test]
    fn test_editor_resize() {
        let mut state = EditorState::new();
        state.resize(100, 50);
        let snapshot = state.snapshot();
        assert_eq!(snapshot.width, 100);
        assert_eq!(snapshot.height, 50);
    }

    #[test]
    fn test_editor_mode_change() {
        let mut state = EditorState::new();
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
        state.handle_key(kjxlkj_core_types::KeyEvent::char('i'));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Insert);
    }

    #[test]
    fn test_editor_escape_back_to_normal() {
        let mut state = EditorState::new();
        state.handle_key(kjxlkj_core_types::KeyEvent::char('i'));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Insert);
        state.handle_key(kjxlkj_core_types::KeyEvent::new(
            kjxlkj_core_types::KeyCode::Escape,
            kjxlkj_core_types::KeyModifiers::NONE,
        ));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
    }

    #[test]
    fn test_editor_snapshot_mode() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
    }

    #[test]
    fn test_editor_snapshot_cursor() {
        let state = EditorState::new();
        let snapshot = state.snapshot();
        assert_eq!(snapshot.cursor.line(), 0);
    }

    #[test]
    fn test_registers_selected_default() {
        let regs = Registers::new();
        assert_eq!(regs.selected(), kjxlkj_core_types::RegisterName::Unnamed);
    }
}
