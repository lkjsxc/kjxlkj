//! Async terminal input reader.

use futures::StreamExt;
use tokio::sync::mpsc;

use kjxlkj_core_types::{Action, Key};

use crate::decode::{decode_crossterm_event, decode_crossterm_key};

/// Reads terminal events and sends keys/actions to the core task.
pub struct InputReader {
    /// Channel to send decoded actions.
    action_tx: mpsc::Sender<Action>,
    /// Channel to send raw keys (for mode-aware dispatch).
    key_tx: mpsc::Sender<Key>,
}

impl InputReader {
    /// Create a new input reader.
    pub fn new(action_tx: mpsc::Sender<Action>, key_tx: mpsc::Sender<Key>) -> Self {
        Self { action_tx, key_tx }
    }

    /// Run the input reader loop.
    ///
    /// This should be spawned as a Tokio task. It reads events from
    /// crossterm's async EventStream and dispatches them.
    pub async fn run(self, mut quit_rx: tokio::sync::broadcast::Receiver<()>) {
        let mut reader = crossterm::event::EventStream::new();

        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                event = reader.next() => {
                    match event {
                        Some(Ok(ref ev)) => {
                            self.handle_event(ev).await;
                        }
                        Some(Err(_)) => break,
                        None => break,
                    }
                }
            }
        }
    }

    async fn handle_event(&self, event: &crossterm::event::Event) {
        // First check for non-key events.
        if let Some(action) = decode_crossterm_event(event) {
            let _ = self.action_tx.send(action).await;
            return;
        }

        // For key events, send the decoded key.
        if let crossterm::event::Event::Key(key_event) = event {
            let key = decode_crossterm_key(key_event);
            let _ = self.key_tx.send(key).await;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn input_reader_types() {
        // Compile-only test to verify types exist.
        use super::InputReader;
        let _ = std::mem::size_of::<InputReader>();
    }
}
