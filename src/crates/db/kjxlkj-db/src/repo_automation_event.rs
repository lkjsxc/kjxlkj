/// Automation event helpers per /docs/spec/domain/automation.md.
/// Automation events are stored as workspace events with
/// automation-prefixed event_type values, ensuring shared
/// workspace stream ordering per /docs/spec/api/websocket.md.

use crate::repo_workspace_event::{self, WorkspaceEventRow};
use kjxlkj_domain::ids::{EventId, UserId, WorkspaceId};
use sqlx::PgPool;
use uuid::Uuid;

/// Emit an automation lifecycle event into the workspace stream.
/// Event types become: automation_queued, automation_running,
/// automation_succeeded, automation_failed,
/// automation_review_requested, automation_review_accepted,
/// automation_review_rejected.
pub async fn emit_lifecycle_event(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    run_id: Uuid,
    status: &str,
    detail: &serde_json::Value,
    actor_id: UserId,
) -> Result<i64, sqlx::Error> {
    let event_type = format!("automation_{status}");
    let payload = serde_json::json!({
        "run_id": run_id.to_string(),
        "status": status,
        "detail": detail,
    });
    let seq = repo_workspace_event::next_workspace_seq(
        pool, workspace_id,
    ).await?;
    let event_id = EventId(Uuid::now_v7());
    repo_workspace_event::append_workspace_event(
        pool, event_id, workspace_id, seq,
        &event_type, &payload, actor_id,
    ).await?;
    Ok(seq)
}

/// List automation events for a specific run from workspace events.
pub async fn list_run_events(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    run_id: Uuid,
) -> Result<Vec<WorkspaceEventRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceEventRow>(
        "SELECT event_id, workspace_id, seq, event_type,
                payload_json, actor_id, created_at
         FROM workspace_events
         WHERE workspace_id = $1
           AND event_type LIKE 'automation_%'
           AND payload_json->>'run_id' = $2
         ORDER BY seq ASC",
    )
    .bind(workspace_id.0)
    .bind(run_id.to_string())
    .fetch_all(pool)
    .await
}
