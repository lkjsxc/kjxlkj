//! Mode state machines and transition logic.
//!
//! Per /docs/spec/modes/transitions.md, mode transitions are deterministic
//! and never block on external IO.

mod command;
mod command_helpers;
mod insert;
mod normal;
mod normal_g_z;
mod normal_keys;
mod normal_single;
mod transition;
mod visual;

pub use command::CommandModeState;
pub use insert::InsertModeState;
pub use normal::NormalModeState;
pub use transition::{ModeTransition, TransitionResult};
pub use visual::VisualModeState;
