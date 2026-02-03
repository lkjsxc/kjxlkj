//! Editing primitives for kjxlkj.
//!
//! This crate provides the core editing operations: operators,
//! motions, and text manipulation.

mod motion;
mod operator;
mod transaction;

pub use motion::*;
pub use operator::*;
pub use transaction::*;

#[cfg(test)]
mod tests;
