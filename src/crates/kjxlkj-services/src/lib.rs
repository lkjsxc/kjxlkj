//! kjxlkj-services - Service infrastructure.
//!
//! This crate provides the message bus and service traits.

mod bus;
mod service;
mod supervisor;

pub use bus::{Message, MessageBus, Subscription};
pub use service::{Service, ServiceHandle};
pub use supervisor::ServiceSupervisor;
