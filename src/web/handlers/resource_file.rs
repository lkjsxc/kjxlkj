use crate::core::looks_like_id;
use crate::error::AppError;
use crate::media::MediaVariants;
use crate::storage::Storage;
use crate::web::db::{self, DbPool, ResourceKind};
use crate::web::handlers::http;
use crate::web::handlers::resource_file_support::inline_image_fallback_allowed;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileQuery {
    variant: Option<String>,
}

pub async fn current_file(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(reference): Path<String>,
    Query(query): Query<FileQuery>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let is_admin = session::check_session(&headers, pool).await?;
    let variant = query.variant.as_deref();
    let file = if looks_like_id(&reference) {
        resolve_id_backed_file(pool, &reference, is_admin, variant).await?
    } else {
        db::get_resource_by_alias(pool, &reference)
            .await?
            .map(|resource| file_from_resource(resource, is_admin, variant))
            .transpose()?
            .flatten()
    };
    let Some(file) = file else {
        return Err(AppError::NotFound("resource file not found".to_string()));
    };
    stream_file(
        &state.storage,
        file.file_key.as_deref(),
        file.content_type.as_deref(),
        headers
            .get(header::RANGE)
            .and_then(|value| value.to_str().ok()),
    )
    .await
}

pub async fn current_file_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_user, reference)): Path<(String, String)>,
    Query(query): Query<FileQuery>,
) -> Result<Response, AppError> {
    current_file(State(state), headers, Path(reference), Query(query)).await
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
) -> Result<Response, AppError> {
    let object = storage
        .get_object(
            file_key.ok_or_else(|| AppError::NotFound("file not found".to_string()))?,
            range,
        )
        .await?;
    let status = if object.content_range.is_some() {
        StatusCode::PARTIAL_CONTENT
    } else {
        StatusCode::OK
    };
    let mut response = http::bytes_with_type(
        status,
        content_type.unwrap_or("application/octet-stream"),
        object.body,
    );
    http::set_header(&mut response, header::ACCEPT_RANGES, "bytes");
    http::set_header(
        &mut response,
        header::CONTENT_LENGTH,
        &object.content_length.to_string(),
    );
    http::set_header(&mut response, header::CONTENT_ENCODING, "identity");
    if let Some(range) = object.content_range {
        http::set_header(&mut response, header::CONTENT_RANGE, &range);
    }
    Ok(response)
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
        return variant_file(
            resource.file_key,
            resource.content_type,
            resource.original_filename,
            resource.media_variants,
            variant,
        );
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
        return variant_file(
            resource.snapshot.file_key,
            resource.snapshot.content_type,
            resource.snapshot.original_filename,
            resource.snapshot.media_variants,
            variant,
        );
    }
    Ok(Some(ResourceFileRef {
        file_key: resource.snapshot.file_key,
        content_type: resource.snapshot.content_type,
    }))
}

fn variant_file(
    file_key: Option<String>,
    content_type: Option<String>,
    original_filename: Option<String>,
    variants: Option<MediaVariants>,
    variant: &str,
) -> Result<Option<ResourceFileRef>, AppError> {
    if !matches!(variant, "card" | "display" | "poster") {
        return Err(AppError::InvalidRequest(
            "unknown media variant".to_string(),
        ));
    }
    if let Some(item) = variants.as_ref().and_then(|variants| variants.get(variant)) {
        return Ok(Some(ResourceFileRef {
            file_key: Some(item.key.clone()),
            content_type: Some(item.content_type.clone()),
        }));
    }
    if matches!(variant, "card" | "display")
        && inline_image_fallback_allowed(content_type.as_deref(), original_filename.as_deref())
    {
        return Ok(file_key.map(|key| ResourceFileRef {
            file_key: Some(key),
            content_type,
        }));
    }
    Ok(None)
}
