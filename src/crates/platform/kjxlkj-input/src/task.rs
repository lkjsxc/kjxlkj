//! Input task for reading terminal events.

use crate::{decode_event, DecodedEvent};
use crossterm::event::EventStream;
use futures::StreamExt;
use kjxlkj_core_types::InputAction;
use tokio::sync::{mpsc, broadcast};
use tracing::{info, debug, error};

impl From<DecodedEvent> for InputAction {
    fn from(event: DecodedEvent) -> Self {
        match event {
            DecodedEvent::Key(key) => InputAction::Key(key),
            DecodedEvent::Resize(cols, rows) => InputAction::Resize(cols, rows),
            DecodedEvent::Paste(text) => InputAction::Paste(text),
            DecodedEvent::FocusGained => InputAction::FocusGained,
            DecodedEvent::FocusLost => InputAction::FocusLost,
        }
    }
}

/// Input task that reads terminal events.
pub struct InputTask {
    event_tx: mpsc::Sender<InputAction>,
    quit_rx: broadcast::Receiver<()>,
}

impl InputTask {
    /// Create a new input task.
    pub fn new(event_tx: mpsc::Sender<InputAction>, quit_rx: broadcast::Receiver<()>) -> Self {
        Self { event_tx, quit_rx }
    }

    /// Run the input loop.
    pub async fn run(mut self) {
        info!("Input task started");

        let mut reader = EventStream::new();

        loop {
            tokio::select! {
                biased;

                _ = self.quit_rx.recv() => {
                    info!("Input task received quit signal");
                    break;
                }

                maybe_event = reader.next() => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            if let Some(decoded) = decode_event(event) {
                                debug!(?decoded, "Decoded event");
                                let action = InputAction::from(decoded);
                                if self.event_tx.send(action).await.is_err() {
                                    error!("Failed to send event");
                                    break;
                                }
                            }
                        }
                        Some(Err(e)) => {
                            error!(?e, "Error reading event");
                            break;
                        }
                        None => {
                            info!("Event stream ended");
                            break;
                        }
                    }
                }
            }
        }

        info!("Input task ended");
    }
}
