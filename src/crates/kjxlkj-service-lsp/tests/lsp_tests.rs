use kjxlkj_service_lsp::requests::{
    build_did_open, build_initialize_request, LspMethod, PendingRequests, ServerCapabilities,
};
use kjxlkj_service_lsp::{
    decode_message, encode_message, Diagnostic, DiagnosticSeverity, DiagnosticStore,
    JsonRpcMessage, JsonRpcRequest,
};

// --- LspMethod coverage ---

#[test]
fn lsp_method_initialize_str() {
    assert_eq!(LspMethod::Initialize.as_str(), "initialize");
}

#[test]
fn lsp_method_completion_str() {
    assert_eq!(LspMethod::Completion.as_str(), "textDocument/completion");
}

#[test]
fn lsp_method_hover_str() {
    assert_eq!(LspMethod::Hover.as_str(), "textDocument/hover");
}

#[test]
fn lsp_method_definition_str() {
    assert_eq!(LspMethod::Definition.as_str(), "textDocument/definition");
}

#[test]
fn lsp_method_all_variants_have_str() {
    let methods = [
        LspMethod::Initialize,
        LspMethod::Shutdown,
        LspMethod::Completion,
        LspMethod::Hover,
        LspMethod::Definition,
        LspMethod::References,
        LspMethod::Rename,
        LspMethod::SignatureHelp,
        LspMethod::CodeAction,
        LspMethod::DocumentSymbol,
        LspMethod::WorkspaceSymbol,
        LspMethod::Formatting,
        LspMethod::GotoImplementation,
        LspMethod::TypeDefinition,
        LspMethod::CodeLens,
    ];
    for m in methods {
        assert!(!m.as_str().is_empty());
    }
}

// --- JSON-RPC serialization ---

#[test]
fn encode_request_has_content_length() {
    let msg = JsonRpcMessage::Request(JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: 1,
        method: "initialize".into(),
        params: None,
    });
    let encoded = encode_message(&msg);
    assert!(encoded.starts_with("Content-Length: "));
}

#[test]
fn encode_decode_roundtrip() {
    let msg = JsonRpcMessage::Request(JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: 42,
        method: "test".into(),
        params: None,
    });
    let encoded = encode_message(&msg);
    let body = encoded.split("\r\n\r\n").nth(1).unwrap();
    assert!(decode_message(body).is_some());
}

#[test]
fn decode_invalid_returns_none() {
    assert!(decode_message("not json!").is_none());
}

#[test]
fn build_initialize_has_process_id() {
    let req = build_initialize_request(1);
    assert_eq!(req.method, "initialize");
    let params = req.params.unwrap();
    assert!(params.get("processId").is_some());
}

#[test]
fn build_did_open_notification() {
    let notif = build_did_open("file:///test.rs", "fn main() {}");
    assert_eq!(notif.method, "textDocument/didOpen");
    assert!(notif.params.is_some());
}

// --- Pending requests ---

#[test]
fn pending_send_and_complete() {
    let mut pr = PendingRequests::new();
    let id = pr.send(LspMethod::Hover);
    assert_eq!(pr.count(), 1);
    assert_eq!(pr.complete(id), Some(LspMethod::Hover));
    assert_eq!(pr.count(), 0);
}

#[test]
fn pending_complete_unknown_returns_none() {
    let mut pr = PendingRequests::new();
    assert_eq!(pr.complete(999), None);
}

#[test]
fn pending_multiple_requests() {
    let mut pr = PendingRequests::new();
    let id1 = pr.send(LspMethod::Completion);
    let id2 = pr.send(LspMethod::Definition);
    assert_eq!(pr.count(), 2);
    pr.complete(id1);
    assert_eq!(pr.count(), 1);
    pr.complete(id2);
    assert_eq!(pr.count(), 0);
}

// --- Diagnostic store ---

#[test]
fn diagnostic_store_add_get() {
    let mut store = DiagnosticStore::new();
    store.add(
        "file:///a.rs",
        Diagnostic {
            message: "unused".into(),
            severity: DiagnosticSeverity::Warning,
            line: 1,
            col: 0,
        },
    );
    assert_eq!(store.get("file:///a.rs").len(), 1);
    assert_eq!(store.error_count(), 0);
}

#[test]
fn diagnostic_store_remove() {
    let mut store = DiagnosticStore::new();
    store.add(
        "file:///b.rs",
        Diagnostic {
            message: "err".into(),
            severity: DiagnosticSeverity::Error,
            line: 5,
            col: 2,
        },
    );
    assert_eq!(store.error_count(), 1);
    store.remove("file:///b.rs");
    assert_eq!(store.total_count(), 0);
}

#[test]
fn diagnostic_severity_ordering() {
    assert!(DiagnosticSeverity::Error < DiagnosticSeverity::Warning);
    assert!(DiagnosticSeverity::Warning < DiagnosticSeverity::Hint);
}

// --- Server capabilities ---

#[test]
fn server_capabilities_defaults() {
    let caps = ServerCapabilities::default();
    assert!(!caps.completion);
    assert!(!caps.hover);
}
