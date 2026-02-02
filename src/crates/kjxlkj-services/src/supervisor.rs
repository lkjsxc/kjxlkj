//! Service supervisor for managing multiple services.

use crate::{MessageBus, ServiceHandle};
use std::collections::HashMap;

/// Manages multiple services with supervision.
pub struct ServiceSupervisor {
    /// Running services by name.
    services: HashMap<&'static str, ServiceHandle>,
    /// Message bus for inter-service communication.
    bus: MessageBus,
}

impl Default for ServiceSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceSupervisor {
    /// Creates a new service supervisor.
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            bus: MessageBus::new(),
        }
    }

    /// Returns a reference to the message bus.
    pub fn bus(&self) -> &MessageBus {
        &self.bus
    }

    /// Adds and starts a service.
    pub fn add(&mut self, handle: ServiceHandle) {
        let name = handle.name();
        self.services.insert(name, handle);
    }

    /// Stops a service by name.
    pub fn stop(&mut self, name: &str) {
        if let Some(mut handle) = self.services.remove(name) {
            handle.shutdown();
        }
    }

    /// Stops all services.
    pub fn stop_all(&mut self) {
        for (_, mut handle) in self.services.drain() {
            handle.shutdown();
        }
    }

    /// Returns the number of running services.
    pub fn count(&self) -> usize {
        self.services.values().filter(|h| h.is_running()).count()
    }

    /// Returns names of all services.
    pub fn service_names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.services.keys().copied()
    }

    /// Checks if a service is running.
    pub fn is_running(&self, name: &str) -> bool {
        self.services.get(name).is_some_and(|h| h.is_running())
    }

    /// Waits for all services to finish.
    pub async fn join_all(mut self) {
        let handles: Vec<_> = self.services.drain().map(|(_, h)| h).collect();
        for handle in handles {
            handle.join().await;
        }
    }
}

impl Drop for ServiceSupervisor {
    fn drop(&mut self) {
        self.stop_all();
    }
}
