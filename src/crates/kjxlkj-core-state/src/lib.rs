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
}
