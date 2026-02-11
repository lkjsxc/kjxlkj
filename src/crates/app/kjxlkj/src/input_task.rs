//! Input task per /docs/spec/architecture/runtime.md.
//!
//! Reads crossterm EventStream, decodes events, and sends
//! keys/actions to the core task through bounded mpsc channels.

use crate::channels::InputSenders;
use crossterm::event::EventStream;
use kjxlkj_core_types::Action;
use kjxlkj_input::{decode_crossterm_event, InputEvent};
use tokio::sync::broadcast;
use tokio_stream::StreamExt;

/// Run the input task until quit signal or stream exhaustion.
pub async fn input_task(
    senders: InputSenders,
    mut quit_rx: broadcast::Receiver<()>,
) {
    let mut reader = EventStream::new();

    loop {
        tokio::select! {
            biased;
            _ = quit_rx.recv() => break,
            event = reader.next() => {
                match event {
                    Some(Ok(evt)) => {
                        let input =
                            decode_crossterm_event(evt);
                        if !dispatch_input(
                            input, &senders,
                        ).await {
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        tracing::error!(
                            "input error: {}", e
                        );
                    }
                    None => break,
                }
            }
        }
    }
}

/// Dispatch a decoded input event to the appropriate channel.
///
/// Returns false if a channel is closed (caller should stop).
async fn dispatch_input(
    input: InputEvent,
    senders: &InputSenders,
) -> bool {
    match input {
        InputEvent::Key(key, mods) => {
            senders.key_tx.send((key, mods)).await.is_ok()
        }
        InputEvent::Resize(c, r) => {
            senders
                .action_tx
                .send(Action::Resize(c, r))
                .await
                .is_ok()
        }
        InputEvent::Paste(text) => {
            senders
                .action_tx
                .send(Action::Paste(text))
                .await
                .is_ok()
        }
        InputEvent::FocusGained => {
            senders
                .action_tx
                .send(Action::FocusGained)
                .await
                .is_ok()
        }
        InputEvent::FocusLost => {
            senders
                .action_tx
                .send(Action::FocusLost)
                .await
                .is_ok()
        }
        InputEvent::Ignored => true,
    }
}
