//! Core domain entities and value objects
//! 
//! This module defines the canonical data models for the kjxlkj platform.
//! All business rules and invariants are enforced at the type level.

pub mod note;
pub mod workspace;
pub mod event;
pub mod search;
pub mod automation;

pub use note::*;
pub use workspace::*;
pub use event::*;
pub use search::*;
pub use automation::*;
