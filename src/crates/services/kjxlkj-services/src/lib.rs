//! Service supervisor and shared service types.
//!
//! See /docs/spec/architecture/runtime.md for service topology.

/// Service request sent from core to a service.
#[derive(Debug, Clone)]
pub enum ServiceRequest {
    /// Placeholder; concrete requests defined per service crate.
    Noop,
}

/// Service response sent from a service back to core.
#[derive(Debug, Clone)]
pub enum ServiceResponse {
    /// Placeholder; concrete responses defined per service crate.
    Noop,
}
