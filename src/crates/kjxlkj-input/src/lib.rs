/// Input decode and key normalization.
///
/// Reads terminal events from crossterm EventStream
/// and converts them to internal Action and Key types.
mod decoder;

pub use decoder::{decode_event, InputTask};
