//! Terminal service implementation.

use tokio::process::Command;
use tokio::sync::mpsc;

use kjxlkj_core_types::{ServiceEvent, ServiceRequest};

/// Async terminal service for command execution.
pub struct TerminalService {
    request_rx: mpsc::Receiver<ServiceRequest>,
    event_tx: mpsc::Sender<ServiceEvent>,
}

impl TerminalService {
    /// Creates a new terminal service.
    pub fn new(
        request_rx: mpsc::Receiver<ServiceRequest>,
        event_tx: mpsc::Sender<ServiceEvent>,
    ) -> Self {
        Self {
            request_rx,
            event_tx,
        }
    }

    /// Runs the service loop.
    pub async fn run(mut self) {
        while let Some(request) = self.request_rx.recv().await {
            let event = self.handle_request(request).await;
            if self.event_tx.send(event).await.is_err() {
                break;
            }
        }
    }

    async fn handle_request(&self, request: ServiceRequest) -> ServiceEvent {
        match request {
            ServiceRequest::ExecuteCommand { command } => {
                self.execute_command(&command).await
            }
            _ => ServiceEvent::Error {
                message: "Terminal service handles commands only".to_string(),
            },
        }
    }

    async fn execute_command(&self, command: &str) -> ServiceEvent {
        let result = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .await;

        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let first_line = stdout
                    .lines()
                    .next()
                    .unwrap_or("")
                    .to_string();
                ServiceEvent::CommandOutput { output: first_line }
            }
            Err(e) => ServiceEvent::Error {
                message: format!("Command failed: {}", e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn execute_echo() {
        let (req_tx, req_rx) = mpsc::channel(10);
        let (evt_tx, mut evt_rx) = mpsc::channel(10);

        let service = TerminalService::new(req_rx, evt_tx);
        let handle = tokio::spawn(service.run());

        req_tx
            .send(ServiceRequest::ExecuteCommand {
                command: "echo hello".to_string(),
            })
            .await
            .unwrap();

        drop(req_tx);

        if let Some(ServiceEvent::CommandOutput { output }) = evt_rx.recv().await {
            assert_eq!(output, "hello");
        }

        handle.await.unwrap();
    }
}
