//! Input decoding and key normalization.

mod decode;
pub mod ime;
pub mod ime_route;
mod ime_tests;
mod input_tests;
mod task;

pub use decode::decode_crossterm_event;
pub use task::spawn_input_task;
