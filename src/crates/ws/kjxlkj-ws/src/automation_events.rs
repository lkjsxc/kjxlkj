/// Automation event classification per /docs/spec/api/websocket.md.
/// Detects automation-prefixed workspace events and converts them
/// to typed AutomationEvent server messages for WS broadcast.
///
/// Per spec, automation events share the workspace stream's
/// monotonic event_seq but are emitted as distinct message types.

use crate::messages::ServerMessage;
use kjxlkj_db::repo_workspace_event::WorkspaceEventRow;
use uuid::Uuid;

/// Automation event type prefix per /docs/spec/domain/automation.md.
const AUTOMATION_PREFIX: &str = "automation_";

/// Check if a workspace event_type is an automation event.
pub fn is_automation_event(event_type: &str) -> bool {
    event_type.starts_with(AUTOMATION_PREFIX)
}

/// Classify a workspace event row; if it is an automation event,
/// return the corresponding AutomationEvent server message.
/// Non-automation events return None.
pub fn classify_event(row: &WorkspaceEventRow) -> Option<ServerMessage> {
    if !is_automation_event(&row.event_type) {
        return None;
    }

    let run_id = row
        .payload_json
        .get("run_id")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<Uuid>().ok())
        .unwrap_or(Uuid::nil());

    let status = row
        .payload_json
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    Some(ServerMessage::AutomationEvent {
        workspace_id: row.workspace_id,
        run_id,
        status,
        event_seq: row.seq,
        event_type: row.event_type.clone(),
        payload: row.payload_json.clone(),
    })
}

/// Build an automation event payload for workspace event storage.
pub fn build_automation_payload(
    run_id: Uuid,
    status: &str,
    detail: &serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "run_id": run_id.to_string(),
        "status": status,
        "detail": detail,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automation_prefix_detection() {
        assert!(is_automation_event("automation_queued"));
        assert!(is_automation_event("automation_running"));
        assert!(is_automation_event("automation_succeeded"));
        assert!(is_automation_event("automation_failed"));
        assert!(!is_automation_event("project_created"));
        assert!(!is_automation_event("member_added"));
    }
}
