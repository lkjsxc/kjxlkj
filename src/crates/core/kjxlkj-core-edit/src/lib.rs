//! Editing operations: motions, operators, text objects.
//!
//! This crate provides the core editing primitives.

mod motion;
mod operator;
mod text_object;

pub use motion::*;
pub use operator::*;
pub use text_object::*;
