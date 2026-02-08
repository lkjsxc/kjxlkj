//! Terminal input decoding → actions/events.

mod decode;
mod keymap;
mod reader;

pub use decode::decode_crossterm_event;
pub use keymap::{KeyTrie, KeymapEntry};
pub use reader::InputReader;
