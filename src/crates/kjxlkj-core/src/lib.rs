//! Core facade crate for the kjxlkj editor.
//!
//! Re-exports all editor-facing core APIs.

pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;

// Re-export commonly used types at the root.
pub use kjxlkj_core_state::EditorState;
pub use kjxlkj_core_types::{Cursor, Key, KeyCode, Mode, Modifiers};
pub use kjxlkj_core_ui::EditorSnapshot;
