//! Service trait and handle.

use std::future::Future;
use std::pin::Pin;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

/// A background service.
pub trait Service: Send + 'static {
    /// The service name.
    fn name(&self) -> &'static str;

    /// Runs the service.
    fn run(self, shutdown: oneshot::Receiver<()>) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

/// Handle to a running service.
pub struct ServiceHandle {
    name: &'static str,
    shutdown_tx: Option<oneshot::Sender<()>>,
    join_handle: JoinHandle<()>,
}

impl ServiceHandle {
    /// Creates a new service handle.
    pub fn new<S: Service>(service: S) -> Self {
        let name = service.name();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let join_handle = tokio::spawn(async move {
            service.run(shutdown_rx).await;
        });

        Self {
            name,
            shutdown_tx: Some(shutdown_tx),
            join_handle,
        }
    }

    /// Returns the service name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Signals the service to shut down.
    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }

    /// Waits for the service to finish.
    pub async fn join(self) {
        let _ = self.join_handle.await;
    }

    /// Returns true if the service is still running.
    pub fn is_running(&self) -> bool {
        !self.join_handle.is_finished()
    }
}
