//! Live streaming settings helpers

use serde_json::Value;

pub fn normalize_ice_servers_json(value: &str) -> Result<Value, String> {
    let parsed: Value = serde_json::from_str(if value.trim().is_empty() { "[]" } else { value })
        .map_err(|_| "Live ICE servers must be a JSON array".to_string())?;
    let array = parsed
        .as_array()
        .ok_or_else(|| "Live ICE servers must be a JSON array".to_string())?;
    for server in array {
        let object = server
            .as_object()
            .ok_or_else(|| "Live ICE servers must contain objects".to_string())?;
        let urls = object
            .get("urls")
            .ok_or_else(|| "Live ICE servers require urls".to_string())?;
        if !valid_urls(urls) {
            return Err("Live ICE server urls must be non-empty strings".to_string());
        }
    }
    Ok(Value::Array(array.clone()))
}

fn valid_urls(value: &Value) -> bool {
    match value {
        Value::String(url) => !url.trim().is_empty(),
        Value::Array(urls) => !urls.is_empty() && urls.iter().all(valid_urls),
        _ => false,
    }
}
