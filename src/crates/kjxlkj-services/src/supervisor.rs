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

    #[test]
    fn test_supervisor_default() {
        let supervisor = Supervisor::default();
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

    #[tokio::test]
    async fn test_supervisor_spawn_multiple() {
        let mut supervisor = Supervisor::new();
        
        supervisor.spawn(Box::new(TestService { name: "a".to_string() })).unwrap();
        supervisor.spawn(Box::new(TestService { name: "b".to_string() })).unwrap();
        supervisor.spawn(Box::new(TestService { name: "c".to_string() })).unwrap();
        
        assert_eq!(supervisor.service_count(), 3);
        assert!(supervisor.service_names().contains(&"a"));
        assert!(supervisor.service_names().contains(&"b"));
        assert!(supervisor.service_names().contains(&"c"));

        supervisor.shutdown_all().await;
    }

    #[tokio::test]
    async fn test_supervisor_shutdown_clears_services() {
        let mut supervisor = Supervisor::new();
        supervisor.spawn(Box::new(TestService { name: "test".to_string() })).unwrap();
        
        assert_eq!(supervisor.service_count(), 1);
        supervisor.shutdown_all().await;
        assert_eq!(supervisor.service_count(), 0);
    }

    #[test]
    fn test_service_names_empty() {
        let supervisor = Supervisor::new();
        assert!(supervisor.service_names().is_empty());
    }

    #[tokio::test]
    async fn test_supervisor_replace_service() {
        let mut supervisor = Supervisor::new();
        
        supervisor.spawn(Box::new(TestService { name: "test".to_string() })).unwrap();
        // Spawning with same name replaces
        supervisor.spawn(Box::new(TestService { name: "test".to_string() })).unwrap();
        
        assert_eq!(supervisor.service_count(), 1);
        supervisor.shutdown_all().await;
    }

    #[test]
    fn test_supervisor_service_count_initial() {
        let supervisor = Supervisor::new();
        assert_eq!(supervisor.service_count(), 0);
    }

    #[tokio::test]
    async fn test_supervisor_spawn_and_check_names() {
        let mut supervisor = Supervisor::new();
        supervisor.spawn(Box::new(TestService { name: "svc1".to_string() })).unwrap();
        supervisor.spawn(Box::new(TestService { name: "svc2".to_string() })).unwrap();
        
        let names = supervisor.service_names();
        assert!(names.contains(&"svc1"));
        assert!(names.contains(&"svc2"));
        assert!(!names.contains(&"svc3"));

        supervisor.shutdown_all().await;
    }

    #[tokio::test]
    async fn test_supervisor_double_shutdown() {
        let mut supervisor = Supervisor::new();
        supervisor.spawn(Box::new(TestService { name: "test".to_string() })).unwrap();
        
        supervisor.shutdown_all().await;
        // Second shutdown should be a no-op
        supervisor.shutdown_all().await;
        assert_eq!(supervisor.service_count(), 0);
    }

    #[tokio::test]
    async fn test_supervisor_spawn_after_shutdown() {
        let mut supervisor = Supervisor::new();
        supervisor.spawn(Box::new(TestService { name: "a".to_string() })).unwrap();
        supervisor.shutdown_all().await;
        
        // Spawn new service after shutdown
        supervisor.spawn(Box::new(TestService { name: "b".to_string() })).unwrap();
        assert_eq!(supervisor.service_count(), 1);
        assert!(supervisor.service_names().contains(&"b"));

        supervisor.shutdown_all().await;
    }

    #[test]
    fn test_supervisor_names_vec_len() {
        let supervisor = Supervisor::new();
        assert_eq!(supervisor.service_names().len(), 0);
    }

    #[tokio::test]
    async fn test_supervisor_service_names_after_spawn() {
        let mut supervisor = Supervisor::new();
        supervisor.spawn(Box::new(TestService { name: "test_svc".to_string() })).unwrap();
        let names = supervisor.service_names();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "test_svc");
        supervisor.shutdown_all().await;
    }
}
