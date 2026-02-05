//! Core facade crate.
//!
//! This crate re-exports editor-facing core APIs.

pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;

// Re-export commonly used types at top level
pub use kjxlkj_core_state::Editor;
pub use kjxlkj_core_types::{EditorEvent, KeyEvent, Mode, Modifier, Position, Range};
