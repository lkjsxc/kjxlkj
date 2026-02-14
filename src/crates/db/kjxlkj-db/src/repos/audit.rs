use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn emit_security_event(
    pool: &PgPool,
    request_id: &str,
    actor_id: Option<Uuid>,
    workspace_id: Option<Uuid>,
    event_type: &str,
    payload_json: Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO security_events (request_id, actor_id, workspace_id, event_type, payload_json)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(request_id)
    .bind(actor_id)
    .bind(workspace_id)
    .bind(event_type)
    .bind(payload_json)
    .execute(pool)
    .await?;
    Ok(())
}
