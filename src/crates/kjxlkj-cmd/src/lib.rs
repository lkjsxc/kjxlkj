//! Command-line parsing and execution for kjxlkj editor.
//!
//! This crate handles Ex command parsing and compilation to intents.

mod command;
mod completion;
mod parser;
mod range;

#[cfg(test)]
mod tests;

pub use command::{Command, CommandKind};
pub use completion::{Candidate, CommandCompletion, CommandRegistry, CompletionSource};
pub use parser::CommandParser;
pub use range::Range;
