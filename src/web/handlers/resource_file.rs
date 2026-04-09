use crate::core::looks_like_id;
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, DbPool, RecordKind};
use crate::web::handlers::session;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse};

#[get("/{reference}/file")]
pub async fn current_file(
    pool: web::Data<DbPool>,
    storage: web::Data<Storage>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let is_admin = session::check_session(&req, &pool).await?;
    let reference = path.into_inner();
    let record = if looks_like_id(&reference) {
        db::get_record(&pool, &reference).await?
    } else {
        db::get_record_by_alias(&pool, &reference).await?
    };
    let Some(record) = record else {
        return Err(AppError::NotFound("resource file not found".to_string()));
    };
    if record.kind != RecordKind::Media || (record.is_private && !is_admin) {
        return Err(AppError::NotFound("resource file not found".to_string()));
    }
    stream_file(
        &storage,
        record.file_key.as_deref(),
        record.content_type.as_deref(),
        req.headers().get("Range").and_then(|value| value.to_str().ok()),
    )
    .await
}

#[get("/{snapshot_id}/file")]
pub async fn snapshot_file(
    pool: web::Data<DbPool>,
    storage: web::Data<Storage>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let is_admin = session::check_session(&req, &pool).await?;
    let snapshot_id = path.into_inner();
    let Some(resource) = db::get_snapshot_resource(&pool, &snapshot_id).await? else {
        return Err(AppError::NotFound("snapshot file not found".to_string()));
    };
    if resource.snapshot.kind != RecordKind::Media || (resource.snapshot.is_private && !is_admin) {
        return Err(AppError::NotFound("snapshot file not found".to_string()));
    }
    stream_file(
        &storage,
        resource.snapshot.file_key.as_deref(),
        resource.snapshot.content_type.as_deref(),
        req.headers().get("Range").and_then(|value| value.to_str().ok()),
    )
    .await
}

async fn stream_file(
    storage: &Storage,
    file_key: Option<&str>,
    content_type: Option<&str>,
    range: Option<&str>,
) -> Result<HttpResponse, AppError> {
    let object = storage
        .get_object(
            file_key.ok_or_else(|| AppError::NotFound("file not found".to_string()))?,
            range,
        )
        .await?;
    let mut builder = HttpResponse::build(if object.content_range.is_some() {
        StatusCode::PARTIAL_CONTENT
    } else {
        StatusCode::OK
    });
    builder.append_header(("Accept-Ranges", "bytes"));
    builder.append_header(("Content-Length", object.content_length.to_string()));
    if let Some(range) = object.content_range {
        builder.append_header(("Content-Range", range));
    }
    Ok(builder
        .content_type(content_type.unwrap_or("application/octet-stream"))
        .body(object.body))
}
