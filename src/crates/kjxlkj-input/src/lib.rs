//! Terminal input decoding.

mod event;
mod key;

#[cfg(test)]
mod key_tests;

pub use event::InputEvent;
pub use key::{Key, KeyCode, Modifiers};
