//! PTY process management.

use std::process::Stdio;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

/// PTY error.
#[derive(Debug, Error)]
pub enum PtyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PTY not running")]
    NotRunning,
}

/// PTY process wrapper.
pub struct Pty {
    /// Child process.
    child: Child,
    /// Write channel.
    tx: mpsc::Sender<Vec<u8>>,
    /// Read channel.
    rx: mpsc::Receiver<Vec<u8>>,
}

impl Pty {
    /// Spawn a new PTY process.
    pub async fn spawn(shell: &str) -> Result<Self, PtyError> {
        let mut child = Command::new(shell)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();

        let (write_tx, mut write_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) =
            mpsc::channel(256);
        let (read_tx, read_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) =
            mpsc::channel(256);

        // Writer task.
        if let Some(mut stdin) = stdin {
            tokio::spawn(async move {
                while let Some(data) = write_rx.recv().await {
                    if stdin.write_all(&data).await.is_err() {
                        break;
                    }
                }
            });
        }

        // Reader task.
        if let Some(mut stdout) = stdout {
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                loop {
                    match stdout.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            if read_tx.send(buf[..n].to_vec()).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }

        Ok(Self {
            child,
            tx: write_tx,
            rx: read_rx,
        })
    }

    /// Write data to the PTY.
    pub async fn write(&self, data: &[u8]) -> Result<(), PtyError> {
        self.tx
            .send(data.to_vec())
            .await
            .map_err(|_| PtyError::NotRunning)
    }

    /// Read data from the PTY.
    pub async fn read(&mut self) -> Option<Vec<u8>> {
        self.rx.recv().await
    }

    /// Check if the process is still running.
    pub fn is_running(&mut self) -> bool {
        self.child.try_wait().ok().flatten().is_none()
    }
}
