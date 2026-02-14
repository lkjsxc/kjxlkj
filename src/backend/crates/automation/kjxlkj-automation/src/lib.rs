use serde_json::Value;

pub fn validate_provider(provider_kind: &str) -> bool {
    matches!(provider_kind, "openrouter" | "lmstudio")
}

pub fn protocol_marker() -> Value {
    serde_json::json!({"protocol": "xml_attrless"})
}
