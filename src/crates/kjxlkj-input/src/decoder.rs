use crossterm::event::{Event, KeyEvent, KeyModifiers};
use kjxlkj_core_types::{Action, Key, KeyCode, Modifier};
use tokio::sync::mpsc;

/// Convert a crossterm Event into Action or Key.
pub fn decode_event(event: Event) -> Option<DecodedEvent> {
    match event {
        Event::Key(key_event) => {
            let key = decode_key(key_event);
            Some(DecodedEvent::Key(key))
        }
        Event::Resize(cols, rows) => Some(DecodedEvent::Action(Action::Resize(cols, rows))),
        Event::Paste(text) => Some(DecodedEvent::Action(Action::Paste(text))),
        Event::FocusGained => Some(DecodedEvent::Action(Action::FocusGained)),
        Event::FocusLost => Some(DecodedEvent::Action(Action::FocusLost)),
        Event::Mouse(_) => None, // Keyboard-only invariant
    }
}

/// Decoded event: either an Action or a Key.
#[derive(Debug)]
pub enum DecodedEvent {
    Action(Action),
    Key(Key),
}

fn decode_key(key_event: KeyEvent) -> Key {
    let code = match key_event.code {
        crossterm::event::KeyCode::Char(c) => {
            // Normalize shifted printable keys
            if key_event.modifiers.contains(KeyModifiers::SHIFT) && c.is_ascii_lowercase() {
                KeyCode::Char(c.to_ascii_uppercase())
            } else {
                KeyCode::Char(c)
            }
        }
        crossterm::event::KeyCode::Enter => KeyCode::Enter,
        crossterm::event::KeyCode::Esc => KeyCode::Esc,
        crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
        crossterm::event::KeyCode::Tab => KeyCode::Tab,
        crossterm::event::KeyCode::BackTab => KeyCode::BackTab,
        crossterm::event::KeyCode::Delete => KeyCode::Delete,
        crossterm::event::KeyCode::Up => KeyCode::Up,
        crossterm::event::KeyCode::Down => KeyCode::Down,
        crossterm::event::KeyCode::Left => KeyCode::Left,
        crossterm::event::KeyCode::Right => KeyCode::Right,
        crossterm::event::KeyCode::Home => KeyCode::Home,
        crossterm::event::KeyCode::End => KeyCode::End,
        crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
        crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
        crossterm::event::KeyCode::F(n) => KeyCode::F(n),
        _ => KeyCode::Char('\0'),
    };

    let mut modifiers = Modifier::NONE;
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
        modifiers = modifiers | Modifier::CTRL;
    }
    if key_event.modifiers.contains(KeyModifiers::ALT) {
        modifiers = modifiers | Modifier::ALT;
    }
    // Only flag shift for non-printable keys
    if key_event.modifiers.contains(KeyModifiers::SHIFT) && !matches!(code, KeyCode::Char(_)) {
        modifiers = modifiers | Modifier::SHIFT;
    }

    Key::new(code, modifiers)
}

/// Input task: reads events and sends to channels.
pub struct InputTask;

impl InputTask {
    /// Run the input reading loop.
    pub async fn run(
        action_tx: mpsc::Sender<Action>,
        key_tx: mpsc::Sender<Key>,
        mut quit_rx: tokio::sync::broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        use crossterm::event::EventStream;
        use tokio_stream::StreamExt;

        let mut reader = EventStream::new();

        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                maybe_event = reader.next() => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            if let Some(decoded) = decode_event(event) {
                                match decoded {
                                    DecodedEvent::Action(action) => {
                                        let _ = action_tx.send(action).await;
                                    }
                                    DecodedEvent::Key(key) => {
                                        let _ = key_tx.send(key).await;
                                    }
                                }
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
        Ok(())
    }
}
