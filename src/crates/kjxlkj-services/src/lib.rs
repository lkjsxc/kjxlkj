//! Service supervisor and wiring.
//!
//! This crate provides:
//! - Service supervisor for managing background services
//! - Message bus for service communication
//! - Service lifecycle management

mod message;
mod supervisor;

pub use message::{ServiceMessage, ServiceRequest, ServiceResponse};
pub use supervisor::ServiceSupervisor;

// Re-export service crates
pub use kjxlkj_service_fs as fs;
pub use kjxlkj_service_git as git;
pub use kjxlkj_service_index as index;
pub use kjxlkj_service_lsp as lsp;
pub use kjxlkj_service_terminal as terminal;

#[cfg(test)]
mod tests;
