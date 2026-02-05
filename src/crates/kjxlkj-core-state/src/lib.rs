//! Editor state aggregation and snapshot production.
//!
//! This crate holds the complete editor state and produces snapshots for rendering.

mod editor;
mod registers;

pub use editor::EditorState;
pub use registers::Registers;

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
        assert!(state.content().is_empty() || state.content().len() == 0);
    }
}
