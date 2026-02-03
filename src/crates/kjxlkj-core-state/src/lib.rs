//! Editor state aggregation and snapshot production.
//!
//! This is the single-writer core state that owns all editor data.

mod command;
mod editor;

pub use command::CommandParser;
pub use editor::EditorState;
