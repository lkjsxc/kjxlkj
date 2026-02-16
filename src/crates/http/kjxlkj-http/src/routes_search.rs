use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use kjxlkj_domain::search::{SearchMode, SearchQuery};
use kjxlkj_domain::error::ErrorCode;
use kjxlkj_search::hybrid;
use kjxlkj_db::repo_search;
use kjxlkj_rbac::check::{self, RbacError};
use kjxlkj_domain::permission::Role;
use crate::config::AppState;
use crate::extract;
use crate::response::error_response;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub mode: Option<String>,
}

/// GET /api/search
pub async fn search(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<SearchParams>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &state.pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    if let Err(e) = check::require_role(&state.pool, query.workspace_id, identity.user_id, Role::Viewer).await {
        return match e {
            RbacError::Forbidden => error_response(ErrorCode::RoleForbidden, "role forbidden"),
            RbacError::NotMember => error_response(ErrorCode::WorkspaceForbidden, "not member"),
            RbacError::Db(_) => error_response(ErrorCode::InternalError, "auth check failed"),
        };
    }

    let mode = match &query.mode {
        Some(m) => match SearchMode::from_str_checked(m) {
            Some(mode) => Some(mode),
            None => return error_response(ErrorCode::SearchModeInvalid, "unknown search mode"),
        },
        None => None,
    };

    let search_query = SearchQuery {
        q: query.q.clone(),
        workspace_id: query.workspace_id,
        project_id: query.project_id,
        limit: query.limit,
        mode,
    };

    match hybrid::search(
        &state.pool,
        &search_query,
        &state.config.search_embedding_base_url,
        &state.config.search_embedding_model,
        state.config.search_semantic_enabled,
    )
    .await
    {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => {
            tracing::error!("search failed: {e}");
            error_response(ErrorCode::InternalError, "search failed")
        }
    }
}

/// GET /api/notes/{id}/backlinks
pub async fn backlinks(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let note_id = path.into_inner();
    match repo_search::get_backlinks(&pool, note_id).await {
        Ok(bl) => HttpResponse::Ok().json(
            bl.iter()
                .map(|b| serde_json::json!({
                    "source_note_id": b.source_note_id,
                    "source_title": b.source_title,
                }))
                .collect::<Vec<_>>(),
        ),
        Err(_) => error_response(ErrorCode::InternalError, "failed to get backlinks"),
    }
}
