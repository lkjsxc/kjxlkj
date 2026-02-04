//! Service supervisor implementation.

use crate::{Service, ServiceHandle, ServiceMessage, ServiceResult, ServiceStatus};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Channel buffer size for service messages.
const CHANNEL_BUFFER_SIZE: usize = 32;

/// Service supervisor.
pub struct Supervisor {
    /// Running service handles.
    services: HashMap<String, ServiceHandle>,
}

impl Supervisor {
    /// Create a new supervisor.
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Spawn a service.
    pub fn spawn(&mut self, service: Box<dyn Service>) -> ServiceResult<()> {
        let name = service.name().to_string();
        info!(%name, "Spawning service");

        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        let handle = ServiceHandle::new(name.clone(), tx);

        // Spawn the service task
        let name_for_spawn = name.clone();
        let fut = service.run(rx);
        tokio::spawn(async move {
            debug!(%name_for_spawn, "Service started");
            fut.await;
            debug!(%name_for_spawn, "Service finished");
        });

        self.services.insert(name, handle);
        Ok(())
    }

    /// Shutdown all services.
    pub async fn shutdown_all(&mut self) {
        info!("Shutting down all services");

        for (name, handle) in &self.services {
            debug!(%name, "Requesting shutdown");
            if let Err(e) = handle.shutdown().await {
                warn!(%name, ?e, "Failed to send shutdown");
            }
        }

        self.services.clear();
    }

    /// Get service names.
    pub fn service_names(&self) -> Vec<&str> {
        self.services.keys().map(|s| s.as_str()).collect()
    }

    /// Get service count.
    pub fn service_count(&self) -> usize {
        self.services.len()
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;

    struct TestService {
        name: String,
    }

    impl Service for TestService {
        fn name(&self) -> &str {
            &self.name
        }

        fn run(
            self: Box<Self>,
            mut rx: mpsc::Receiver<ServiceMessage>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async move {
                while let Some(msg) = rx.recv().await {
                    match msg {
                        ServiceMessage::Shutdown => break,
                        _ => {}
                    }
                }
            })
        }
    }

    #[test]
    fn test_supervisor_new() {
        let supervisor = Supervisor::new();
        assert_eq!(supervisor.service_count(), 0);
    }

    #[tokio::test]
    async fn test_supervisor_spawn() {
        let mut supervisor = Supervisor::new();
        let service = Box::new(TestService {
            name: "test".to_string(),
        });
        supervisor.spawn(service).unwrap();
        assert_eq!(supervisor.service_count(), 1);
        assert!(supervisor.service_names().contains(&"test"));

        supervisor.shutdown_all().await;
    }
}
