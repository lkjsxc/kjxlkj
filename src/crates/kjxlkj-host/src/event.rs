//! Host event types.

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use kjxlkj_core_ui::Dimensions;
use tokio_stream::StreamExt;

/// Events from the terminal host.
#[derive(Debug, Clone)]
pub enum HostEvent {
    /// Key press.
    Key(KeyEvent),
    /// Terminal resize.
    Resize(Dimensions),
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
}

/// Stream of host events.
pub struct HostEventStream {
    inner: EventStream,
}

impl HostEventStream {
    /// Creates a new event stream.
    pub fn new() -> Self {
        Self {
            inner: EventStream::new(),
        }
    }

    /// Gets the next event.
    pub async fn next(&mut self) -> Option<HostEvent> {
        loop {
            match self.inner.next().await {
                Some(Ok(event)) => {
                    if let Some(host_event) = Self::convert_event(event) {
                        return Some(host_event);
                    }
                }
                Some(Err(_)) => continue,
                None => return None,
            }
        }
    }

    /// Converts a crossterm event.
    fn convert_event(event: Event) -> Option<HostEvent> {
        match event {
            Event::Key(key) => Some(HostEvent::Key(key)),
            Event::Resize(width, height) => {
                Some(HostEvent::Resize(Dimensions::new(width, height)))
            }
            Event::FocusGained => Some(HostEvent::FocusGained),
            Event::FocusLost => Some(HostEvent::FocusLost),
            _ => None,
        }
    }
}

impl Default for HostEventStream {
    fn default() -> Self {
        Self::new()
    }
}

impl HostEvent {
    /// Returns true if this is Ctrl+C.
    pub fn is_quit(&self) -> bool {
        matches!(
            self,
            HostEvent::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            })
        )
    }

    /// Returns true if this is Escape.
    pub fn is_escape(&self) -> bool {
        matches!(
            self,
            HostEvent::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            })
        )
    }
}
