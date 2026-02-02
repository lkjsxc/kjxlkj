//! PTY process management.

use std::path::PathBuf;
use tokio::sync::mpsc;

/// PTY process configuration.
#[derive(Debug, Clone)]
pub struct PtyConfig {
    /// Shell command.
    pub shell: PathBuf,
    /// Shell arguments.
    pub args: Vec<String>,
    /// Working directory.
    pub cwd: Option<PathBuf>,
    /// Environment variables.
    pub env: Vec<(String, String)>,
    /// Initial size (cols, rows).
    pub size: (u16, u16),
}

impl Default for PtyConfig {
    fn default() -> Self {
        let shell = std::env::var("SHELL")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/bin/sh"));

        Self {
            shell,
            args: Vec::new(),
            cwd: None,
            env: Vec::new(),
            size: (80, 24),
        }
    }
}

/// A PTY process.
pub struct PtyProcess {
    /// Process ID.
    pid: Option<u32>,
    /// Configuration.
    config: PtyConfig,
    /// Input sender.
    input_tx: mpsc::Sender<Vec<u8>>,
    /// Output receiver.
    output_rx: mpsc::Receiver<Vec<u8>>,
}

impl PtyProcess {
    /// Creates a new PTY process (not yet spawned).
    pub fn new(config: PtyConfig) -> Self {
        let (input_tx, _input_rx) = mpsc::channel(256);
        let (_output_tx, output_rx) = mpsc::channel(256);

        Self {
            pid: None,
            config,
            input_tx,
            output_rx,
        }
    }

    /// Returns the process ID if running.
    pub fn pid(&self) -> Option<u32> {
        self.pid
    }

    /// Returns the configuration.
    pub fn config(&self) -> &PtyConfig {
        &self.config
    }

    /// Writes input to the PTY.
    pub async fn write(&self, data: Vec<u8>) -> Result<(), mpsc::error::SendError<Vec<u8>>> {
        self.input_tx.send(data).await
    }

    /// Reads output from the PTY.
    pub async fn read(&mut self) -> Option<Vec<u8>> {
        self.output_rx.recv().await
    }

    /// Resizes the PTY.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.config.size = (cols, rows);
        // TODO: Send SIGWINCH to process
    }
}
