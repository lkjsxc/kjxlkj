//! Parser types and error definitions.
//!
//! Types for parser errors.

/// Parse error.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Unknown command.
    UnknownCommand(String),
    /// Missing required argument.
    MissingArgument(&'static str),
    /// Invalid substitute syntax.
    InvalidSubstitute,
}
