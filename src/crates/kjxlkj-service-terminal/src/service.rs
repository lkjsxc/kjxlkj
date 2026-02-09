//! Terminal service: manages PTY processes and screen buffers.

use std::collections::HashMap;

use tokio::sync::{broadcast, mpsc};

use kjxlkj_core_types::{ServiceResponse, TerminalId};

use crate::screen::ScreenBuffer;

/// Terminal service managing multiple PTY sessions.
pub struct TerminalService {
    response_tx: mpsc::Sender<ServiceResponse>,
    screens: HashMap<TerminalId, ScreenBuffer>,
    next_id: u64,
}

impl TerminalService {
    pub fn new(response_tx: mpsc::Sender<ServiceResponse>) -> Self {
        Self {
            response_tx,
            screens: HashMap::new(),
            next_id: 1,
        }
    }

    /// Allocate a new terminal ID.
    pub fn alloc_id(&mut self) -> TerminalId {
        let id = TerminalId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Create a new screen buffer.
    pub fn create_screen(&mut self, cols: u16, rows: u16) -> TerminalId {
        let id = self.alloc_id();
        let screen = ScreenBuffer::new(id, cols, rows);
        self.screens.insert(id, screen);
        id
    }

    /// Get screen buffer by ID.
    pub fn get_screen(&self, id: TerminalId) -> Option<&ScreenBuffer> {
        self.screens.get(&id)
    }

    /// Get screen buffer mutably.
    pub fn get_screen_mut(&mut self, id: TerminalId) -> Option<&mut ScreenBuffer> {
        self.screens.get_mut(&id)
    }

    /// Run the terminal service loop.
    pub async fn run(self, mut quit_rx: broadcast::Receiver<()>) {
        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                _ = tokio::time::sleep(
                    std::time::Duration::from_secs(3600)
                ) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_terminal() {
        let (tx, _rx) = mpsc::channel(256);
        let mut svc = TerminalService::new(tx);
        let id = svc.create_screen(80, 24);
        assert!(svc.get_screen(id).is_some());
    }
}
