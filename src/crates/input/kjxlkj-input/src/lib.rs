#![forbid(unsafe_code)]

mod input_event;
mod map;

pub use input_event::InputEvent;
pub use map::{map_event, map_key_event};
