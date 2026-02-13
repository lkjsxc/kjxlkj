//! Database layer for kjxlkj.
//!
//! This crate contains SQLite repositories and migrations.

pub mod pool;
pub mod users;
pub mod workspaces;
pub mod notes;
pub mod automation;

pub use pool::*;
