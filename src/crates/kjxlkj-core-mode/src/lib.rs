//! Modal state machines for kjxlkj.
//!
//! Modes interpret key input and emit intents.

mod command;
mod insert;
mod keymap;
mod normal;
mod state;
mod visual;

pub use command::*;
pub use insert::*;
pub use keymap::*;
pub use normal::*;
pub use state::*;
pub use visual::*;

#[cfg(test)]
mod tests;
