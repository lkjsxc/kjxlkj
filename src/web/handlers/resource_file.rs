use crate::core::looks_like_id;
use crate::error::AppError;
use crate::media::MediaVariants;
use crate::storage::Storage;
use crate::web::db::{self, DbPool, ResourceKind};
use crate::web::handlers::session;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileQuery {
    variant: Option<String>,
}

#[get("/{reference}/file")]
pub async fn current_file(
    pool: web::Data<DbPool>,
    storage: web::Data<Storage>,
    req: HttpRequest,
    path: web::Path<String>,
    query: web::Query<FileQuery>,
) -> Result<HttpResponse, AppError> {
    let is_admin = session::check_session(&req, &pool).await?;
    let reference = path.into_inner();
    let variant = query.variant.as_deref();
    let file = if looks_like_id(&reference) {
        resolve_id_backed_file(&pool, &reference, is_admin, variant).await?
    } else {
        db::get_resource_by_alias(&pool, &reference)
            .await?
            .map(|resource| file_from_resource(resource, is_admin, variant))
            .transpose()?
            .flatten()
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
    variant: Option<&str>,
) -> Result<Option<ResourceFileRef>, AppError> {
    if let Some(resource) = db::get_resource(pool, reference).await? {
        return file_from_resource(resource, is_admin, variant);
    }
    db::get_snapshot_target(pool, reference)
        .await?
        .map(|resource| file_from_snapshot(resource, is_admin, variant))
        .transpose()
        .map(Option::flatten)
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
    builder.append_header(("Content-Encoding", "identity"));
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

fn file_from_resource(
    resource: db::Resource,
    is_admin: bool,
    variant: Option<&str>,
) -> Result<Option<ResourceFileRef>, AppError> {
    if resource.kind != ResourceKind::Media || (resource.is_private && !is_admin) {
        return Ok(None);
    }
    if let Some(variant) = variant {
        return variant_file(resource.media_variants, variant);
    }
    Ok(Some(ResourceFileRef {
        file_key: resource.file_key,
        content_type: resource.content_type,
    }))
}

fn file_from_snapshot(
    resource: db::SnapshotTarget,
    is_admin: bool,
    variant: Option<&str>,
) -> Result<Option<ResourceFileRef>, AppError> {
    if resource.snapshot.kind != ResourceKind::Media || (resource.snapshot.is_private && !is_admin)
    {
        return Ok(None);
    }
    if let Some(variant) = variant {
        return variant_file(resource.snapshot.media_variants, variant);
    }
    Ok(Some(ResourceFileRef {
        file_key: resource.snapshot.file_key,
        content_type: resource.snapshot.content_type,
    }))
}

fn variant_file(
    variants: Option<MediaVariants>,
    variant: &str,
) -> Result<Option<ResourceFileRef>, AppError> {
    if !matches!(variant, "card" | "display" | "poster") {
        return Err(AppError::InvalidRequest(
            "unknown media variant".to_string(),
        ));
    }
    Ok(variants.and_then(|variants| {
        variants.get(variant).map(|item| ResourceFileRef {
            file_key: Some(item.key.clone()),
            content_type: Some(item.content_type.clone()),
        })
    }))
}
