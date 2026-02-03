//! Terminal input decoding for kjxlkj.
//!
//! Converts terminal events to typed Key events.

mod convert;
mod script;

pub use convert::*;
pub use script::*;

#[cfg(test)]
mod tests;
