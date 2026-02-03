//! Service supervisor and wiring.
//!
//! Manages async services that run alongside the editor core.

mod bus;
mod supervisor;

pub use bus::MessageBus;
pub use supervisor::ServiceSupervisor;
