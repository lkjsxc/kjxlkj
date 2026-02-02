//! PTY wrapper.

use std::path::PathBuf;

/// PTY instance.
pub struct Pty {
    /// PTY ID.
    id: u64,
    /// Shell command.
    shell: PathBuf,
    /// Running flag.
    running: bool,
}

impl Pty {
    /// Creates a new PTY.
    pub fn new(id: u64, shell: PathBuf) -> Self {
        Self {
            id,
            shell,
            running: false,
        }
    }

    /// Returns the ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Starts the PTY.
    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        Ok(())
    }

    /// Stops the PTY.
    pub async fn stop(&mut self) {
        self.running = false;
    }

    /// Writes data to the PTY.
    pub async fn write(&mut self, _data: &[u8]) -> anyhow::Result<()> {
        Ok(())
    }

    /// Resizes the PTY.
    pub fn resize(&mut self, _cols: u16, _rows: u16) {}

    /// Returns true if running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}
