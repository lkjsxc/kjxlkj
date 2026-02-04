//! Terminal input decoding.

mod event;
mod key;

pub use event::InputEvent;
pub use key::{Key, KeyCode, Modifiers};
