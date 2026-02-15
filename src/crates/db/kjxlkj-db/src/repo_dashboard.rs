//! Dashboard widgets repository per /docs/spec/api/http.md.

use kjxlkj_domain::ids::WorkspaceId;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct DashboardWidgetRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub widget_type: String,
    pub config_json: serde_json::Value,
    pub layout: Option<serde_json::Value>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn list_widgets(
    pool: &PgPool,
    workspace_id: WorkspaceId,
) -> Result<Vec<DashboardWidgetRow>, sqlx::Error> {
    sqlx::query_as::<_, DashboardWidgetRow>(
        "SELECT id, workspace_id, widget_type, config_json, layout,
                created_at, updated_at
         FROM dashboard_widgets WHERE workspace_id = $1
         ORDER BY created_at",
    )
    .bind(workspace_id.0)
    .fetch_all(pool)
    .await
}

pub async fn upsert_widget(
    pool: &PgPool,
    id: Uuid,
    workspace_id: WorkspaceId,
    widget_type: &str,
    config_json: &serde_json::Value,
    layout: Option<&serde_json::Value>,
) -> Result<DashboardWidgetRow, sqlx::Error> {
    sqlx::query_as::<_, DashboardWidgetRow>(
        "INSERT INTO dashboard_widgets
         (id, workspace_id, widget_type, config_json, layout)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (id) DO UPDATE
         SET widget_type = EXCLUDED.widget_type,
             config_json = EXCLUDED.config_json,
             layout = EXCLUDED.layout,
             updated_at = now()
         RETURNING id, workspace_id, widget_type, config_json, layout,
                   created_at, updated_at",
    )
    .bind(id)
    .bind(workspace_id.0)
    .bind(widget_type)
    .bind(config_json)
    .bind(layout)
    .fetch_one(pool)
    .await
}
