//! Input task: reads terminal events and sends decoded actions/keys.

use crossterm::event::EventStream;
use futures::StreamExt;
use kjxlkj_core_types::{Action, Key};
use tokio::sync::{broadcast, mpsc};

use crate::decode::{decode_crossterm_event, DecodedEvent};

/// Spawn the input reading task.
pub async fn spawn_input_task(
    action_tx: mpsc::Sender<Action>,
    key_tx: mpsc::Sender<Key>,
    mut quit_rx: broadcast::Receiver<()>,
) {
    let mut stream = EventStream::new();

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                tracing::info!("Input task: quit signal received");
                break;
            }
            maybe_event = stream.next() => {
                match maybe_event {
                    Some(Ok(event)) => {
                        match decode_crossterm_event(event) {
                            DecodedEvent::Action(action) => {
                                if action_tx.send(action).await.is_err() {
                                    break;
                                }
                            }
                            DecodedEvent::Key(key) => {
                                if key_tx.send(key).await.is_err() {
                                    break;
                                }
                            }
                            DecodedEvent::Ignore => {}
                        }
                    }
                    Some(Err(e)) => {
                        tracing::error!("Input error: {e}");
                        break;
                    }
                    None => break,
                }
            }
        }
    }
}
