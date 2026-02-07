//! JSON-RPC protocol types for LSP communication.

use serde::{Deserialize, Serialize};

/// A JSON-RPC message (request, response, or notification).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}

/// A JSON-RPC request with an id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// A JSON-RPC response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// A JSON-RPC error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A JSON-RPC notification (no id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// Encode a JSON-RPC message with Content-Length header.
pub fn encode_message(msg: &JsonRpcMessage) -> String {
    let body = serde_json::to_string(msg).expect("serialize json-rpc");
    format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
}

/// Decode a JSON-RPC message from raw input (body only, no header).
pub fn decode_message(input: &str) -> Option<JsonRpcMessage> {
    serde_json::from_str(input).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let req = JsonRpcMessage::Request(JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: 1,
            method: "initialize".into(),
            params: None,
        });
        let encoded = encode_message(&req);
        assert!(encoded.starts_with("Content-Length: "));
        let body = encoded.split("\r\n\r\n").nth(1).unwrap();
        let decoded = decode_message(body);
        assert!(decoded.is_some());
    }

    #[test]
    fn decode_invalid_returns_none() {
        assert!(decode_message("not json").is_none());
    }
}
