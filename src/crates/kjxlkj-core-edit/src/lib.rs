//! kjxlkj-core-edit - Edit operations for the editor.
//!
//! This crate provides the core editing operations.

mod operation;
mod executor;
mod motion;
mod motion_word;
mod motion_misc;
mod operator;
mod repeat;

pub use operation::{EditOperation, OperationResult};
pub use executor::EditExecutor;
pub use motion::{MotionContext, MotionExecutor};
pub use operator::{OperatorContext, OperatorExecutor, OperatorResult};
pub use repeat::{InsertType, RecordedChange, RepeatTracker};
