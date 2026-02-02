//! LSP client implementation.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};

use crate::protocol::{LspNotification, LspRequest, LspResponse};

/// An LSP client connection.
pub struct LspClient {
    /// Server command.
    command: String,
    /// Server arguments.
    args: Vec<String>,
    /// Next request ID.
    next_id: AtomicU64,
    /// Pending requests.
    pending: Arc<RwLock<HashMap<u64, oneshot::Sender<LspResponse>>>>,
    /// Whether initialized.
    initialized: bool,
}

impl LspClient {
    /// Creates a new LSP client.
    pub fn new(command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            command: command.into(),
            args,
            next_id: AtomicU64::new(1),
            pending: Arc::new(RwLock::new(HashMap::new())),
            initialized: false,
        }
    }

    /// Returns the server command.
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Generates a new request ID.
    fn next_request_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Creates a request.
    pub fn create_request(&self, method: &str, params: serde_json::Value) -> LspRequest {
        LspRequest {
            id: self.next_request_id(),
            method: method.to_string(),
            params,
        }
    }

    /// Creates an initialize request.
    pub fn initialize_request(&self, root_uri: &str) -> LspRequest {
        let params = serde_json::json!({
            "processId": std::process::id(),
            "rootUri": root_uri,
            "capabilities": {
                "textDocument": {
                    "completion": {
                        "completionItem": {
                            "snippetSupport": true
                        }
                    },
                    "hover": {},
                    "definition": {},
                    "references": {},
                    "documentSymbol": {},
                    "publishDiagnostics": {}
                }
            }
        });
        self.create_request("initialize", params)
    }

    /// Creates a text document did open notification.
    pub fn did_open_notification(
        &self,
        uri: &str,
        language_id: &str,
        version: i32,
        text: &str,
    ) -> LspNotification {
        LspNotification {
            method: "textDocument/didOpen".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": uri,
                    "languageId": language_id,
                    "version": version,
                    "text": text
                }
            }),
        }
    }

    /// Creates a completion request.
    pub fn completion_request(&self, uri: &str, line: u32, character: u32) -> LspRequest {
        self.create_request(
            "textDocument/completion",
            serde_json::json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character }
            }),
        )
    }

    /// Creates a hover request.
    pub fn hover_request(&self, uri: &str, line: u32, character: u32) -> LspRequest {
        self.create_request(
            "textDocument/hover",
            serde_json::json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character }
            }),
        )
    }

    /// Creates a go to definition request.
    pub fn definition_request(&self, uri: &str, line: u32, character: u32) -> LspRequest {
        self.create_request(
            "textDocument/definition",
            serde_json::json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character }
            }),
        )
    }
}
