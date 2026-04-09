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
    let file = if looks_like_id(&reference) {
        resolve_id_backed_file(&pool, &reference, is_admin).await?
    } else {
        db::get_record_by_alias(&pool, &reference)
            .await?
            .and_then(|record| file_from_record(record, is_admin))
    };
    let Some(file) = file else {
        return Err(AppError::NotFound("resource file not found".to_string()));
    };
    stream_file(
        &storage,
        file.file_key.as_deref(),
        file.content_type.as_deref(),
        req.headers()
            .get("Range")
            .and_then(|value| value.to_str().ok()),
    )
    .await
}

async fn resolve_id_backed_file(
    pool: &DbPool,
    reference: &str,
    is_admin: bool,
) -> Result<Option<ResourceFileRef>, AppError> {
    if let Some(record) = db::get_record(pool, reference).await? {
        return Ok(file_from_record(record, is_admin));
    }
    Ok(db::get_snapshot_resource(pool, reference)
        .await?
        .and_then(|resource| file_from_snapshot(resource, is_admin)))
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

struct ResourceFileRef {
    file_key: Option<String>,
    content_type: Option<String>,
}

fn file_from_record(record: db::Record, is_admin: bool) -> Option<ResourceFileRef> {
    if record.kind != RecordKind::Media || (record.is_private && !is_admin) {
        return None;
    }
    Some(ResourceFileRef {
        file_key: record.file_key,
        content_type: record.content_type,
    })
}

fn file_from_snapshot(resource: db::SnapshotResource, is_admin: bool) -> Option<ResourceFileRef> {
    if resource.snapshot.kind != RecordKind::Media || (resource.snapshot.is_private && !is_admin) {
        return None;
    }
    Some(ResourceFileRef {
        file_key: resource.snapshot.file_key,
        content_type: resource.snapshot.content_type,
    })
}
