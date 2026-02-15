pub mod automation_events;
pub mod messages;
pub mod protocol;
pub mod protocol_patch;
pub mod route;
pub mod session_actor;
pub mod subscriptions;

/// Apply simplified patch ops to markdown text.
/// Shared between HTTP and WS crates.
pub fn apply_patch_ops(base: &str, ops: &[serde_json::Value]) -> String {
    let mut result = String::new();
    let chars: Vec<char> = base.chars().collect();
    let mut pos = 0usize;

    for op in ops {
        if let Some(retain) = op.get("retain").and_then(|v| v.as_u64()) {
            let end = (pos + retain as usize).min(chars.len());
            for c in &chars[pos..end] {
                result.push(*c);
            }
            pos = end;
        } else if let Some(text) = op.get("insert").and_then(|v| v.as_str()) {
            result.push_str(text);
        } else if let Some(del) = op.get("delete").and_then(|v| v.as_u64()) {
            pos = (pos + del as usize).min(chars.len());
        }
    }
    for c in &chars[pos..] {
        result.push(*c);
    }
    result
}
