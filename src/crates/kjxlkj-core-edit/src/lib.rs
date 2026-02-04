//! Editing primitives and operators.
//!
//! This crate provides the editing operations for the editor.

mod motions;
mod operators;
mod text_objects;

pub use motions::*;
pub use operators::*;
pub use text_objects::*;
