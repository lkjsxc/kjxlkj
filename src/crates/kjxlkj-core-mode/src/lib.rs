//! Modal state machines and input interpretation.

mod intent;
mod normal;
mod state;

#[cfg(test)]
mod intent_tests;
#[cfg(test)]
mod state_tests;

pub use intent::Intent;
pub use normal::NormalModeState;
pub use state::ModeState;

