//! Mode state machines and transition logic.
//!
//! Per /docs/spec/modes/transitions.md, mode transitions are deterministic
//! and never block on external IO.

mod command;
mod command_helpers;
#[cfg(test)]
mod command_tests;
mod insert;
mod normal;
mod normal_commands;
mod normal_g_z;
mod normal_keys;
mod normal_nav;
mod normal_single;
#[cfg(test)]
mod normal_tests;
mod transition;
mod visual;
mod visual_ops;
#[cfg(test)]
mod visual_tests;

pub use command::CommandModeState;
pub use insert::InsertModeState;
pub use normal::NormalModeState;
pub use transition::{ModeTransition, TransitionResult};
pub use visual::VisualModeState;
