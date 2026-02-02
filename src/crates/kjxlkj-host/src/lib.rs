//! Terminal host integration for kjxlkj editor.
//!
//! This crate handles terminal setup and lifecycle.

mod event;
mod terminal;

pub use event::{HostEvent, HostEventStream};
pub use terminal::TerminalHost;
