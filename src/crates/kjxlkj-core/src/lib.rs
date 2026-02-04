//! Core facade crate.
//!
//! This crate re-exports core APIs for the editor.

pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;

// Re-export commonly used types at the top level
pub use kjxlkj_core_state::EditorState;
pub use kjxlkj_core_types::{Cursor, Mode, Position, Range, Register, RegisterName};
pub use kjxlkj_core_ui::Snapshot;
