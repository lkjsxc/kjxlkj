//! Modal state machines for kjxlkj editor.
//!
//! This crate implements the mode state machine and input interpretation.

mod command;
mod insert;
mod intent;
mod normal;
mod state;
mod visual;

pub use command::CommandState;
pub use insert::InsertState;
pub use intent::{Intent, IntentKind};
pub use normal::NormalState;
pub use state::ModeState;
pub use visual::VisualState;
