//! Core editor logic.
//!
//! This crate provides the main core task and action processing.

mod action;
mod command;
mod editing;
mod motion;
mod task;

pub use action::*;
pub use task::*;

// Re-export commonly used types.
pub use kjxlkj_core_mode::{dispatch_key, HandleResult, ModeAction};
pub use kjxlkj_core_state::EditorState;
pub use kjxlkj_core_types::*;
pub use kjxlkj_core_ui::EditorSnapshot;
