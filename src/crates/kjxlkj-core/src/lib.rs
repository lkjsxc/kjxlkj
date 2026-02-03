//! Core facade crate.
//!
//! Re-exports all core APIs for the editor.

pub use kjxlkj_core_edit::{Buffer, CursorOps, Transaction};
pub use kjxlkj_core_mode::{
    CommandLineState, KeyCode, KeyInput, ModeHandler, ModeState, Modifiers,
};
pub use kjxlkj_core_state::{CommandParser, EditorState};
pub use kjxlkj_core_text::{grapheme_width, line_grapheme_count, RopeText};
pub use kjxlkj_core_types::{
    BufferId, BufferVersion, ByteOffset, CharOffset, Cursor, EditorAction,
    EditorEvent, LineCol, Mode, Point, ServiceEvent, ServiceRequest, WindowId,
};
pub use kjxlkj_core_ui::{
    BufferSnapshot, EditorSnapshot, StatusLine, Viewport,
};
pub use kjxlkj_core_undo::{EditOperation, UndoGroup, UndoHistory};
