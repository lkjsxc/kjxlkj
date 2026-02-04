//! Modal editing state machines.
//!
//! This crate handles mode-specific key interpretation and intent generation.

mod command;
mod handler;
mod insert;
mod normal;
pub mod parser;
mod replace;
mod visual;

pub use command::CommandMode;
pub use handler::{ModeHandler, ModeResult};
pub use insert::InsertMode;
pub use normal::NormalMode;
pub use parser::{ParseResult, Parser};
pub use replace::ReplaceMode;
pub use visual::VisualMode;
