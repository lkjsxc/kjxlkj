//! kjxlkj-core - Core facade for the kjxlkj editor.
//!
//! This crate provides the main entry point to the editor core,
//! re-exporting types from sub-crates and providing the Editor struct.

mod buffer;
mod buffer_cursor;

pub use kjxlkj_core_types as types;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_undo as undo;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_state as state;

pub use buffer::Buffer;

// Re-export commonly used types
pub use types::{
    ids::{BufferId, WindowId},
    mode::Mode,
    position::Position,
};
