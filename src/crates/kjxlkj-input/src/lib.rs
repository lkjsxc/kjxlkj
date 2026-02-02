//! Terminal input decoding for kjxlkj editor.
//!
//! This crate converts key events into intents.

mod cmdline;
mod key;
mod mapping;
mod parser;

pub use cmdline::CommandLine;
pub use key::{Key, KeyCodeWrapper, KeySequence, Modifiers};
pub use mapping::KeyMap;
pub use parser::InputParser;
