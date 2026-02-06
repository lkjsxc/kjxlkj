//! Service supervisor — manages background service lifecycle.

use std::collections::HashMap;

/// Unique service identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceId(pub String);

/// Status of a managed service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

/// A handle to a running service.
pub struct ServiceHandle {
    pub id: ServiceId,
    pub status: ServiceStatus,
    cancel: tokio::sync::watch::Sender<bool>,
}

/// Supervises the lifecycle of background services (LSP, git, fs watch, etc.).
pub struct ServiceSupervisor {
    services: HashMap<ServiceId, ServiceHandle>,
}

impl ServiceSupervisor {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Register and start a new service. Returns the service ID.
    pub fn register(&mut self, name: impl Into<String>) -> ServiceId {
        let id = ServiceId(name.into());
        let (tx, _rx) = tokio::sync::watch::channel(false);
        let handle = ServiceHandle {
            id: id.clone(),
            status: ServiceStatus::Starting,
            cancel: tx,
        };
        tracing::info!(service = %id.0, "registering service");
        self.services.insert(id.clone(), handle);
        id
    }

    /// Stop a service by ID.
    pub fn stop(&mut self, id: &ServiceId) -> anyhow::Result<()> {
        if let Some(handle) = self.services.get_mut(id) {
            handle.status = ServiceStatus::Stopping;
            let _ = handle.cancel.send(true);
            tracing::info!(service = %id.0, "stopping service");
            Ok(())
        } else {
            anyhow::bail!("service not found: {}", id.0)
        }
    }

    /// Stop all services.
    pub fn stop_all(&mut self) {
        let ids: Vec<_> = self.services.keys().cloned().collect();
        for id in ids {
            let _ = self.stop(&id);
        }
    }

    /// Query the status of a service.
    pub fn status(&self, id: &ServiceId) -> Option<ServiceStatus> {
        self.services.get(id).map(|h| h.status)
    }
}

impl Default for ServiceSupervisor {
    fn default() -> Self {
        Self::new()
    }
}
