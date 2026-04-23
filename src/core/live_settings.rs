//! Live streaming settings helpers

use serde_json::Value;

pub const LIVE_HEIGHTS: &[i64] = &[360, 480, 720, 1080, 1440, 2160];
pub const LIVE_FPS_VALUES: &[i64] = &[15, 30, 45, 60, 120];

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
            return Err("Live ICE server urls must be non-empty strings with known schemes".to_string());
        }
        if has_turn_url(urls) {
            let has_user = object.get("username").and_then(Value::as_str).map_or(false, |s| !s.is_empty());
            let has_cred = object.get("credential").and_then(Value::as_str).map_or(false, |s| !s.is_empty());
            if has_user && !has_cred {
                return Err("Live ICE TURN servers with username require credential".to_string());
            }
        }
    }
    Ok(Value::Array(array.clone()))
}

pub fn normalize_live_source(value: &str) -> Result<String, String> {
    let value = value.trim();
    if matches!(value, "screen" | "camera") {
        Ok(value.to_string())
    } else {
        Err("Live default source must be screen or camera".to_string())
    }
}

pub fn validate_live_height(value: i64) -> Result<i64, String> {
    if LIVE_HEIGHTS.contains(&value) {
        Ok(value)
    } else {
        Err("Live default quality must use an approved height".to_string())
    }
}

pub fn validate_live_fps(value: i64) -> Result<i64, String> {
    if LIVE_FPS_VALUES.contains(&value) {
        Ok(value)
    } else {
        Err("Live default frame rate must use an approved fps".to_string())
    }
}

fn valid_urls(value: &Value) -> bool {
    match value {
        Value::String(url) => valid_url_str(url),
        Value::Array(urls) => !urls.is_empty() && urls.iter().all(valid_urls),
        _ => false,
    }
}

fn valid_url_str(url: &str) -> bool {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return false;
    }
    trimmed.starts_with("stun:") || trimmed.starts_with("turn:") || trimmed.starts_with("turns:")
}

fn has_turn_url(value: &Value) -> bool {
    match value {
        Value::String(url) => url.trim().starts_with("turn:" ) || url.trim().starts_with("turns:"),
        Value::Array(urls) => urls.iter().any(has_turn_url),
        _ => false,
    }
}
