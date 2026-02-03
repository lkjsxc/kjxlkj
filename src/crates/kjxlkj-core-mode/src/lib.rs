//! Modal state machines and input interpretation.
//!
//! Modes interpret input keys into editor actions.

mod handler;
mod state;

pub use handler::{CommandLineState, KeyCode, KeyInput, ModeHandler, Modifiers};
pub use state::ModeState;
