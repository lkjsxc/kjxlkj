//! LSP client: manages communication with a language server.

use std::collections::HashMap;
use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::oneshot;

use crate::codec;

/// LSP client connected to a language server process.
pub struct LspClient {
    /// Language server process.
    child: Option<Child>,
    /// Next request ID.
    next_id: u64,
    /// Pending response callbacks.
    pending: HashMap<u64, oneshot::Sender<serde_json::Value>>,
}

impl LspClient {
    /// Spawn a language server.
    pub async fn spawn(command: &str, args: &[&str]) -> Result<Self, String> {
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("spawn: {e}"))?;

        Ok(Self {
            child: Some(child),
            next_id: 1,
            pending: HashMap::new(),
        })
    }

    /// Send a request to the server.
    pub async fn request(
        &mut self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let id = self.next_id;
        self.next_id += 1;

        let req = codec::JsonRpcRequest::new(id, method, params);
        let body = serde_json::to_vec(&req).map_err(|e| format!("serialize: {e}"))?;
        let msg = codec::encode_message(&body);

        if let Some(ref mut child) = self.child {
            if let Some(ref mut stdin) = child.stdin {
                stdin
                    .write_all(&msg)
                    .await
                    .map_err(|e| format!("write: {e}"))?;
            }
        }

        // Simplified: return empty value.
        Ok(serde_json::Value::Null)
    }

    /// Send a notification (no response expected).
    pub async fn notify(
        &mut self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<(), String> {
        let notif = codec::JsonRpcNotification {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params,
        };
        let body = serde_json::to_vec(&notif).map_err(|e| format!("serialize: {e}"))?;
        let msg = codec::encode_message(&body);

        if let Some(ref mut child) = self.child {
            if let Some(ref mut stdin) = child.stdin {
                stdin
                    .write_all(&msg)
                    .await
                    .map_err(|e| format!("write: {e}"))?;
            }
        }

        Ok(())
    }

    /// Shut down the language server.
    pub async fn shutdown(&mut self) -> Result<(), String> {
        let _ = self.request("shutdown", None).await;
        let _ = self.notify("exit", None).await;

        if let Some(mut child) = self.child.take() {
            let _ = child.kill().await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn client_types() {
        use super::LspClient;
        let _ = std::mem::size_of::<LspClient>();
    }
}
