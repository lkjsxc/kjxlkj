//! Modal state machines and input interpretation.

mod command;
mod input;
mod normal;
mod state;

pub use command::CommandState;
pub use input::{InputResult, ParsedInput};
pub use normal::NormalState;
pub use state::ModeState;
