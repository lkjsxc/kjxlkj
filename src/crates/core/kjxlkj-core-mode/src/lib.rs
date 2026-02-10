//! Mode handling and dispatch.
//!
//! This crate provides mode state machines and key dispatch.

mod dispatch;
mod handler;
mod insert;
mod normal;
mod other_modes;
mod state;

pub use dispatch::*;
pub use handler::*;
pub use state::*;
