//! LSP JSON-RPC protocol types: messages, requests, responses, notifications.

use serde::{Deserialize, Serialize};

/// A JSON-RPC request ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId { Num(i64), Str(String) }

/// A JSON-RPC request message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: String, pub id: RequestId, pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// A JSON-RPC response message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: String, pub id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

/// A JSON-RPC error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: i32, pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A JSON-RPC notification (no id, no response expected).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub jsonrpc: String, pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// LSP error codes.
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INTERNAL_ERROR: i32 = -32603;
    pub const SERVER_NOT_INITIALIZED: i32 = -32002;
    pub const REQUEST_CANCELLED: i32 = -32800;
}

/// Initialize request params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub process_id: Option<i64>,
    pub root_uri: Option<String>,
    pub capabilities: ClientCapabilities,
}

/// Client capabilities (subset).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document: Option<TextDocClientCaps>,
}

/// Text document client capabilities (subset).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocClientCaps {
    pub completion: Option<bool>, pub hover: Option<bool>,
    pub definition: Option<bool>, pub references: Option<bool>,
}

/// textDocument/didOpen params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenParams { pub text_document: TextDocumentItem }

/// A text document item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    pub uri: String, pub language_id: String, pub version: i32, pub text: String,
}

/// textDocument/didChange params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeParams {
    pub text_document: VersionedTextDocId, pub content_changes: Vec<ContentChange>,
}

/// Versioned text document identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedTextDocId { pub uri: String, pub version: i32 }

/// A content change event (full sync).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChange { pub text: String }

/// Text document identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier { pub uri: String }

/// textDocument/didClose params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidCloseParams { pub text_document: TextDocumentIdentifier }

/// Build an initialize request.
pub fn make_initialize(id: i64, root_uri: Option<&str>) -> Request {
    let params = InitializeParams {
        process_id: Some(std::process::id() as i64),
        root_uri: root_uri.map(|s| s.to_string()),
        capabilities: ClientCapabilities::default(),
    };
    Request {
        jsonrpc: "2.0".into(), id: RequestId::Num(id), method: "initialize".into(),
        params: Some(serde_json::to_value(params).unwrap()),
    }
}

/// Build a textDocument/didOpen notification.
pub fn make_did_open(uri: &str, lang: &str, ver: i32, text: &str) -> Notification {
    let params = DidOpenParams {
        text_document: TextDocumentItem { uri: uri.into(), language_id: lang.into(), version: ver, text: text.into() },
    };
    Notification { jsonrpc: "2.0".into(), method: "textDocument/didOpen".into(),
        params: Some(serde_json::to_value(params).unwrap()) }
}

/// Build a textDocument/didChange notification.
pub fn make_did_change(uri: &str, ver: i32, text: &str) -> Notification {
    let params = DidChangeParams {
        text_document: VersionedTextDocId { uri: uri.into(), version: ver },
        content_changes: vec![ContentChange { text: text.into() }],
    };
    Notification { jsonrpc: "2.0".into(), method: "textDocument/didChange".into(),
        params: Some(serde_json::to_value(params).unwrap()) }
}

/// Encode an LSP message with Content-Length header.
pub fn encode_message(body: &str) -> String {
    format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_id_serde() {
        let id = RequestId::Num(42);
        let j = serde_json::to_string(&id).unwrap();
        assert_eq!(j, "42");
        assert_eq!(serde_json::from_str::<RequestId>(&j).unwrap(), id);
    }

    #[test]
    fn make_initialize_request() {
        let req = make_initialize(1, Some("file:///project"));
        assert_eq!(req.method, "initialize");
        let p = req.params.unwrap();
        assert!(p["rootUri"].as_str().unwrap().contains("project"));
    }

    #[test]
    fn make_did_open_and_change() {
        let open = make_did_open("file:///t.rs", "rust", 1, "fn main() {}");
        assert_eq!(open.method, "textDocument/didOpen");
        let chg = make_did_change("file:///t.rs", 2, "fn main() { println!(); }");
        assert_eq!(chg.params.unwrap()["textDocument"]["version"], 2);
    }

    #[test]
    fn encode_message_header() {
        let msg = encode_message("{\"t\":1}");
        assert!(msg.starts_with("Content-Length: 7\r\n\r\n"));
    }

    #[test]
    fn response_error_and_codes() {
        let err = ResponseError { code: error_codes::PARSE_ERROR, message: "bad".into(), data: None };
        assert_eq!(err.code, -32700);
    }

    #[test]
    fn did_close_serde() {
        let p = DidCloseParams { text_document: TextDocumentIdentifier { uri: "file:///a.rs".into() } };
        let j = serde_json::to_string(&p).unwrap();
        assert!(j.contains("a.rs"));
    }
}
