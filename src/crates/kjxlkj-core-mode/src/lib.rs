//! Mode handling for kjxlkj editor.
//!
//! This crate handles modal input interpretation and key parsing.

mod parser;
mod state;

pub use parser::{KeySequence, ParseResult, Parser};
pub use state::ModeState;
