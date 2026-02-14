use serde_json::Value;

pub fn envelope(event_type: &str, payload: Value) -> Value {
    serde_json::json!({
        "type": event_type,
        "payload": payload,
    })
}
