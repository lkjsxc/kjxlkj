//! JSON-RPC codec for LSP communication.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC request message.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// JSON-RPC response message.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC notification (no id).
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl JsonRpcRequest {
    pub fn new(id: u64, method: &str, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params,
        }
    }
}

/// Encode a JSON-RPC message with Content-Length header.
pub fn encode_message(body: &[u8]) -> Vec<u8> {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    let mut msg = header.into_bytes();
    msg.extend_from_slice(body);
    msg
}

/// Parse Content-Length from LSP header bytes.
pub fn parse_content_length(header: &str) -> Option<usize> {
    for line in header.lines() {
        if let Some(val) = line.strip_prefix("Content-Length: ") {
            return val.trim().parse().ok();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let req = JsonRpcRequest::new(1, "initialize", None);
        let body = serde_json::to_vec(&req).unwrap();
        let msg = encode_message(&body);
        let header_end = msg.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
        let header = std::str::from_utf8(&msg[..header_end]).unwrap();
        let len = parse_content_length(header).unwrap();
        assert_eq!(len, body.len());
    }
}
