//! Modal state machines and input interpretation.
//!
//! This crate provides:
//! - Mode state machine
//! - Key sequence parsing
//! - Intent generation from key sequences

mod intent;
mod key;
mod mode_state;
mod parser;

pub use intent::{Intent, IntentKind};
pub use key::{Key, KeyCode, KeyModifiers};
pub use mode_state::ModeState;
pub use parser::KeyParser;

#[cfg(test)]
mod tests;
