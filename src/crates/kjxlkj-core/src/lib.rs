//! Core facade for kjxlkj editor.
//!
//! This crate re-exports core APIs and provides the core task.

mod action;
mod core_task;
mod intent_handler;
pub mod motion;
mod motion_core;
pub mod operator;
mod operator_exec;
mod register_ops;
mod search_ops;
mod text_ops;
mod window_ops;

#[cfg(test)]
mod tests;

pub use action::{Action, ActionResult};
pub use core_task::{CoreTask, CoreHandle};
pub use motion::execute_motion;
pub use operator::{execute_operator, OperatorResult};

// Re-exports
pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;
