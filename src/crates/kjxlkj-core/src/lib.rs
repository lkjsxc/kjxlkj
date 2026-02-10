//! Core facade: re-exports the core crate family.

pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;

pub use kjxlkj_core_state::EditorState;
pub use kjxlkj_core_types::{Action, Key, Mode};
pub use kjxlkj_core_ui::EditorSnapshot;
