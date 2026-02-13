//! Domain types and business logic for kjxlkj.
//!
//! This crate contains core domain types, invariants, and business rules.

pub mod types;
pub mod notes;
pub mod users;
pub mod workspaces;
pub mod projects;
pub mod automation;
pub mod events;

pub use types::*;
pub use notes::*;
pub use users::*;
pub use workspaces::*;
pub use projects::*;
pub use automation::*;
pub use events::*;
