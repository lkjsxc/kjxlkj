//! Modal state machines and input interpretation.

mod parser; mod parser_modes; mod parser_normal;
mod parser_operators; mod parser_sequences;
mod pending_state; mod state; pub mod transitions;

pub use parser::KeyParser;
pub use state::ModeState;
