//! Service supervision tests.
//!
//! These tests verify the service supervision properties specified in
//! `/docs/spec/architecture/runtime.md`.

use kjxlkj_services::{Service, ServiceMessage, Supervisor};
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc;

/// A test service that can be configured to fail.
struct TestService {
    name: String,
    fail_immediately: bool,
}

impl TestService {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fail_immediately: false,
        }
    }

    fn with_fail_immediately(mut self) -> Self {
        self.fail_immediately = true;
        self
    }
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
            if self.fail_immediately {
                // Simulate failure by returning early
                return;
            }

            // Normal service loop
            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => break,
                    ServiceMessage::Custom(_) => {}
                }
            }
        })
    }
}

/// Test: Supervisor creation is clean.
#[tokio::test]
async fn test_supervisor_creation() {
    let supervisor = Supervisor::new();
    // Supervisor should be usable immediately
    assert_eq!(supervisor.service_count(), 0);
}

/// Test: Service spawn and stop lifecycle.
#[tokio::test]
async fn test_service_lifecycle() {
    let mut supervisor = Supervisor::new();

    let service = TestService::new("test-service");
    supervisor.spawn(Box::new(service)).expect("spawn should work");

    // Give it a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Supervisor should have the service
    assert_eq!(supervisor.service_count(), 1);

    // Stop all services
    supervisor.shutdown_all().await;

    // Should be empty after shutdown
    assert_eq!(supervisor.service_count(), 0);
}

/// Test: Service failure does not corrupt supervisor.
#[tokio::test]
async fn test_service_failure_isolation() {
    let mut supervisor = Supervisor::new();

    // Spawn a service that will fail immediately
    let failing_service = TestService::new("failing").with_fail_immediately();
    supervisor.spawn(Box::new(failing_service)).expect("spawn should work");

    // Spawn a normal service
    let normal_service = TestService::new("normal");
    supervisor.spawn(Box::new(normal_service)).expect("spawn should work");

    // Give them time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Supervisor should still be functional (both were registered)
    assert_eq!(supervisor.service_count(), 2);

    // Clean shutdown
    supervisor.shutdown_all().await;
}

/// Test: Multiple services can be spawned.
#[tokio::test]
async fn test_multiple_services() {
    let mut supervisor = Supervisor::new();

    for i in 0..5 {
        let service = TestService::new(&format!("service-{}", i));
        supervisor.spawn(Box::new(service)).expect("spawn should work");
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // All services should be tracked
    assert_eq!(supervisor.service_count(), 5);

    supervisor.shutdown_all().await;
}

/// Test: Shutdown is deterministic.
#[tokio::test]
async fn test_shutdown_determinism() {
    let mut supervisor1 = Supervisor::new();
    let mut supervisor2 = Supervisor::new();

    // Same service configuration
    for i in 0..3 {
        supervisor1.spawn(Box::new(TestService::new(&format!("svc-{}", i)))).expect("spawn");
        supervisor2.spawn(Box::new(TestService::new(&format!("svc-{}", i)))).expect("spawn");
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Both supervisors should shut down cleanly
    supervisor1.shutdown_all().await;
    supervisor2.shutdown_all().await;

    // Both should be empty after shutdown
    assert_eq!(supervisor1.service_count(), 0);
    assert_eq!(supervisor2.service_count(), 0);
}

/// Test: Service names are tracked.
#[tokio::test]
async fn test_service_names() {
    let mut supervisor = Supervisor::new();

    supervisor.spawn(Box::new(TestService::new("alpha"))).expect("spawn");
    supervisor.spawn(Box::new(TestService::new("beta"))).expect("spawn");
    supervisor.spawn(Box::new(TestService::new("gamma"))).expect("spawn");

    let names = supervisor.service_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));

    supervisor.shutdown_all().await;
}
