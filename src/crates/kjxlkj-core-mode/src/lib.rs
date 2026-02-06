//! Modal state machines and input interpretation.

mod parser;
mod parser_modes;
mod parser_normal;
mod parser_sequences;
mod state;

pub use parser::KeyParser;
pub use state::ModeState;
