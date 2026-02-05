//! kjxlkj-core - Public API facade for the editor core.
//!
//! This crate re-exports the core types and modules needed
//! to use kjxlkj as a library.

pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;

// Re-export commonly used types at the crate root
pub use kjxlkj_core_state::Editor;
pub use kjxlkj_core_types::{
    BufferId, BufferName, BufferVersion, Cursor, EditorEvent, Intent, KeyEvent, Mode, Position,
    Range, WindowId,
};
pub use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, Viewport, WindowSnapshot};

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::edit::{apply_motion, Motion, Operator, TextObject};
    pub use crate::mode::ModeState;
    pub use crate::state::{BufferState, Editor, WindowState};
    pub use crate::text::TextBuffer;
    pub use crate::types::{
        BufferId, BufferName, Cursor, EditorEvent, Intent, KeyEvent, Mode, Position, Range,
        WindowId,
    };
    pub use crate::ui::{BufferSnapshot, EditorSnapshot, Viewport, WindowSnapshot};
    pub use crate::undo::UndoHistory;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports() {
        // Verify that core types are accessible
        let _mode = Mode::Normal;
        let _pos = Position::new(0, 0);
        let _cursor = Cursor::origin();
    }

    #[test]
    fn test_prelude() {
        use crate::prelude::*;

        let _editor = Editor::new(80, 24);
        let _buf = TextBuffer::new();
    }
}
