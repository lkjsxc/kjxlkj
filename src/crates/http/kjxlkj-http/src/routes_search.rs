use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{NoteId, WorkspaceId};
use kjxlkj_search::{backlinks, fts};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// GET /search per /docs/spec/api/http.md.
/// Searches notes by text, tags, metadata filters.
pub async fn search(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_str = query.get("workspace_id").map(|s| s.as_str()).unwrap_or("");
    let ws_id = match ws_str.parse::<Uuid>() {
        Ok(u) => WorkspaceId(u),
        Err(_) => return domain_error_response(
            DomainError::BadRequest("workspace_id required".into()), &rid,
        ),
    };
    let q = query.get("q").map(|s| s.as_str()).unwrap_or("");
    if q.is_empty() {
        return domain_error_response(
            DomainError::BadRequest("q (query) required".into()), &rid,
        );
    }

    match fts::search_notes(pool.get_ref(), ws_id, q).await {
        Ok(results) => {
            let list: Vec<SearchResultResponse> = results.into_iter().map(|r| {
                SearchResultResponse {
                    note_id: r.note_id,
                    title: r.title,
                    rank: r.rank,
                }
            }).collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// GET /notes/{id}/backlinks per /docs/spec/api/http.md.
pub async fn get_backlinks(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let note_id = NoteId(path.into_inner());

    match backlinks::get_backlinks(pool.get_ref(), note_id).await {
        Ok(results) => {
            let list: Vec<BacklinkResponse> = results.into_iter().map(|r| {
                BacklinkResponse {
                    source_note_id: r.source_note_id,
                    title: r.title,
                }
            }).collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}
