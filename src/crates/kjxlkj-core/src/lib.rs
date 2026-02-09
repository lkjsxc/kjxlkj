//! Core facade crate — re-exports editor-facing core APIs.
//!
//! This crate serves as the single dependency for higher layers
//! (input, render, host, binary) that need access to any of the
//! core sub-crates. Rather than listing seven separate deps in
//! every consumer's Cargo.toml, they depend on `kjxlkj-core` and
//! reach everything via one namespace.
//!
//! # Sub-crate modules
//!
//! | Module | Crate | Purpose |
//! |--------|-------|---------|
//! | `types` | `kjxlkj-core-types` | Shared types, actions, keys |
//! | `text` | `kjxlkj-core-text` | Rope-backed text model |
//! | `edit` | `kjxlkj-core-edit` | Editing primitives |
//! | `mode` | `kjxlkj-core-mode` | Mode state machines |
//! | `undo` | `kjxlkj-core-undo` | Undo tree |
//! | `ui` | `kjxlkj-core-ui` | UI model types |
//! | `state` | `kjxlkj-core-state` | Editor state |

pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;

// Convenience re-exports of the most used types.
pub use kjxlkj_core_types::{
    Action, BufferId, Key, KeyCode, KeyModifiers, Mode,
    Motion, Operator, WindowId,
};
pub use kjxlkj_core_ui::EditorSnapshot;
pub use kjxlkj_core_state::EditorState;

/// Crate version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Minimum terminal dimensions.
pub const MIN_COLS: u16 = 20;
/// Minimum terminal rows.
pub const MIN_ROWS: u16 = 5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn re_exports_accessible() {
        let _m = Mode::Normal;
        let _b = BufferId(1);
        let _w = WindowId(1);
    }

    #[test]
    fn version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn min_dims() {
        assert!(MIN_COLS >= 10);
        assert!(MIN_ROWS >= 3);
    }
}
