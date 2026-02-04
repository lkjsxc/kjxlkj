//! Modal state machines and input interpretation.
//!
//! This crate handles mode transitions and input parsing.

mod parser;
mod state;

pub use parser::{ParseResult, PendingInput};
pub use state::ModeState;
