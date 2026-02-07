//! LSP request builders and method tracking.

use crate::protocol::{JsonRpcNotification, JsonRpcRequest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LSP method kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LspMethod {
    Initialize,
    Shutdown,
    Completion,
    Hover,
    Definition,
    References,
    Rename,
    SignatureHelp,
    CodeAction,
    DocumentSymbol,
    WorkspaceSymbol,
    Formatting,
    GotoImplementation,
    TypeDefinition,
    CodeLens,
}

impl LspMethod {
    /// Return the LSP method string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::Shutdown => "shutdown",
            Self::Completion => "textDocument/completion",
            Self::Hover => "textDocument/hover",
            Self::Definition => "textDocument/definition",
            Self::References => "textDocument/references",
            Self::Rename => "textDocument/rename",
            Self::SignatureHelp => "textDocument/signatureHelp",
            Self::CodeAction => "textDocument/codeAction",
            Self::DocumentSymbol => "textDocument/documentSymbol",
            Self::WorkspaceSymbol => "workspace/symbol",
            Self::Formatting => "textDocument/formatting",
            Self::GotoImplementation => "textDocument/implementation",
            Self::TypeDefinition => "textDocument/typeDefinition",
            Self::CodeLens => "textDocument/codeLens",
        }
    }
}

/// Tracks pending (sent but not yet responded) requests.
#[derive(Debug, Default)]
pub struct PendingRequests {
    next_id: u64,
    pending: HashMap<u64, LspMethod>,
}

impl PendingRequests {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            pending: HashMap::new(),
        }
    }

    /// Register a new request, returning the assigned id.
    pub fn send(&mut self, method: LspMethod) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.pending.insert(id, method);
        id
    }

    /// Mark a request as completed. Returns the method if it was pending.
    pub fn complete(&mut self, id: u64) -> Option<LspMethod> {
        self.pending.remove(&id)
    }

    /// Number of pending requests.
    pub fn count(&self) -> usize {
        self.pending.len()
    }
}

/// Advertised server capabilities (feature flags).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub completion: bool,
    pub hover: bool,
    pub definition: bool,
    pub references: bool,
    pub rename: bool,
    pub code_action: bool,
    pub formatting: bool,
    pub signature_help: bool,
    pub document_symbol: bool,
    pub code_lens: bool,
}

/// Build an `initialize` JSON-RPC request.
pub fn build_initialize_request(id: u64) -> JsonRpcRequest {
    let params = serde_json::json!({
        "processId": std::process::id(),
        "capabilities": {},
        "rootUri": null,
    });
    JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id,
        method: "initialize".into(),
        params: Some(params),
    }
}

/// Build a `textDocument/didOpen` notification.
pub fn build_did_open(uri: &str, text: &str) -> JsonRpcNotification {
    let params = serde_json::json!({
        "textDocument": {
            "uri": uri,
            "languageId": "plaintext",
            "version": 1,
            "text": text,
        }
    });
    JsonRpcNotification {
        jsonrpc: "2.0".into(),
        method: "textDocument/didOpen".into(),
        params: Some(params),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_requests_lifecycle() {
        let mut pr = PendingRequests::new();
        let id = pr.send(LspMethod::Hover);
        assert_eq!(pr.count(), 1);
        assert_eq!(pr.complete(id), Some(LspMethod::Hover));
        assert_eq!(pr.count(), 0);
    }

    #[test]
    fn lsp_method_strings() {
        assert_eq!(LspMethod::Initialize.as_str(), "initialize");
        assert_eq!(LspMethod::Completion.as_str(), "textDocument/completion");
    }
}
