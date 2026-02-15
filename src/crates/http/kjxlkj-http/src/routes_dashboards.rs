//! Dashboard widget route handlers per /docs/spec/api/http.md.
//! Optional extension.

use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_dashboard;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::WorkspaceId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto_views::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// GET /dashboards per /docs/spec/api/http.md.
pub async fn list_dashboards(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(u) => WorkspaceId(u),
        None => {
            return domain_error_response(
                DomainError::BadRequest("workspace_id required".into()),
                &rid,
            )
        }
    };
    match repo_dashboard::list_widgets(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let list: Vec<DashboardWidgetResponse> = rows
                .into_iter()
                .map(|w| DashboardWidgetResponse {
                    id: w.id,
                    workspace_id: w.workspace_id,
                    widget_type: w.widget_type,
                    config_json: w.config_json,
                    layout: w.layout,
                    created_at: w.created_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// POST /dashboards/widgets per /docs/spec/api/http.md.
pub async fn upsert_widget(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<UpsertWidgetRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_id = WorkspaceId(body.workspace_id);
    let widget_id = body.id.unwrap_or_else(Uuid::now_v7);
    match repo_dashboard::upsert_widget(
        pool.get_ref(),
        widget_id,
        ws_id,
        &body.widget_type,
        &body.config_json,
        body.layout.as_ref(),
    )
    .await
    {
        Ok(w) => HttpResponse::Ok().json(DashboardWidgetResponse {
            id: w.id,
            workspace_id: w.workspace_id,
            widget_type: w.widget_type,
            config_json: w.config_json,
            layout: w.layout,
            created_at: w.created_at.to_string(),
        }),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}
