//! Modal state machines and input interpretation.

mod parser; mod parser_modes; mod parser_normal;
mod parser_operators; mod parser_sequences;
mod pending_state; mod state; pub mod transitions;
mod insert_mode_ext; mod replace_mode; mod cmdline_parser;
mod completion_engine;
mod insert_newline;
mod popup_overlay;

pub use parser::KeyParser;
pub use state::ModeState;
