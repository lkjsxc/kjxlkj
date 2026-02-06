//! Core facade crate — re-exports all core sub-crates under a single namespace.

pub mod explorer_bridge;
mod buffer_metadata;
mod lsp_request;
mod lsp_features;

pub use kjxlkj_core_types as types;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_undo as undo;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_state as state;
