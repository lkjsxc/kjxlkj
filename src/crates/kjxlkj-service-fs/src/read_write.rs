//! FS service: file read/write operations.

use tokio::sync::{broadcast, mpsc};

use kjxlkj_core_types::ServiceResponse;

/// Filesystem service handling file I/O operations.
pub struct FsService {
    response_tx: mpsc::Sender<ServiceResponse>,
}

impl FsService {
    pub fn new(
        response_tx: mpsc::Sender<ServiceResponse>,
    ) -> Self {
        Self { response_tx }
    }

    /// Read a file and send its contents back.
    pub async fn read_file(
        &self,
        path: std::path::PathBuf,
    ) -> Result<(), String> {
        let contents = tokio::fs::read(&path)
            .await
            .map_err(|e| format!("{e}"))?;

        let _ = self
            .response_tx
            .send(ServiceResponse::FileContents(
                path,
                contents,
            ))
            .await;
        Ok(())
    }

    /// Write content to a file.
    pub async fn write_file(
        &self,
        path: std::path::PathBuf,
        content: Vec<u8>,
    ) -> Result<(), String> {
        tokio::fs::write(&path, &content)
            .await
            .map_err(|e| format!("{e}"))?;

        let _ = self
            .response_tx
            .send(ServiceResponse::FileWritten(path))
            .await;
        Ok(())
    }

    /// Run the service loop.
    pub async fn run(
        self,
        mut quit_rx: broadcast::Receiver<()>,
    ) {
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
    fn create_fs_service() {
        let (tx, _rx) = mpsc::channel(256);
        let _service = FsService::new(tx);
    }
}
