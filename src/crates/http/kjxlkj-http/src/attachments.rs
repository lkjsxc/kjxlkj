// Attachments handler per /docs/spec/api/http.md
// POST /notes/{id}/attachments, GET /attachments/{id}, DELETE /attachments/{id}
use actix_web::{web, HttpResponse};
use kjxlkj_auth::middleware::AuthSession;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::ErrorBody;

/// POST /api/notes/{id}/attachments — upload attachment
pub async fn upload(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Bytes,
    auth: AuthSession,
) -> HttpResponse {
    let note_id = path.into_inner();
    let rid = Uuid::now_v7().to_string();
    let _actor = auth.user.id;
    let data = body.to_vec();
    let size = data.len() as i64;

    if size > 10 * 1024 * 1024 {
        return HttpResponse::PayloadTooLarge().json(ErrorBody {
            code: "ATTACHMENT_TOO_LARGE".into(),
            message: "Attachment exceeds 10MB limit".into(),
            details: None,
            request_id: rid,
        });
    }

    let att_id = Uuid::now_v7();
    let sha = format!("{:x}", md5_lite(&data));

    match sqlx::query(
        "INSERT INTO attachments (id, note_id, filename, mime, size_bytes, sha256, chunk_count)
         VALUES ($1, $2, $3, $4, $5, $6, 1)",
    )
    .bind(att_id)
    .bind(note_id)
    .bind("upload")
    .bind("application/octet-stream")
    .bind(size)
    .bind(&sha)
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            // Store as single chunk
            let _ = sqlx::query(
                "INSERT INTO attachment_chunks (attachment_id, chunk_index, data)
                 VALUES ($1, 0, $2)",
            )
            .bind(att_id)
            .bind(&data)
            .execute(pool.get_ref())
            .await;
            HttpResponse::Created()
                .json(serde_json::json!({"id": att_id, "size": size}))
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
            request_id: rid,
        }),
    }
}

/// GET /api/attachments/{id} — download attachment
pub async fn download(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    _auth: AuthSession,
) -> HttpResponse {
    let att_id = path.into_inner();
    let rid = Uuid::now_v7().to_string();

    // Fetch metadata
    let meta: Option<(String, String)> = sqlx::query_as(
        "SELECT filename, mime FROM attachments WHERE id = $1",
    )
    .bind(att_id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    let Some((_filename, mime)) = meta else {
        return HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(),
            message: "Attachment not found".into(),
            details: None,
            request_id: rid,
        });
    };

    // Fetch chunks
    let chunks: Vec<(Vec<u8>,)> = sqlx::query_as(
        "SELECT data FROM attachment_chunks WHERE attachment_id = $1
         ORDER BY chunk_index",
    )
    .bind(att_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let body: Vec<u8> = chunks.into_iter().flat_map(|(d,)| d).collect();
    HttpResponse::Ok().content_type(mime).body(body)
}

/// DELETE /api/attachments/{id} — delete attachment
pub async fn delete(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    _auth: AuthSession,
) -> HttpResponse {
    let att_id = path.into_inner();
    let _ = sqlx::query("DELETE FROM attachment_chunks WHERE attachment_id = $1")
        .bind(att_id)
        .execute(pool.get_ref())
        .await;
    let _ = sqlx::query("DELETE FROM attachments WHERE id = $1")
        .bind(att_id)
        .execute(pool.get_ref())
        .await;
    HttpResponse::NoContent().finish()
}

/// Simple hash for attachment dedup (not cryptographic).
fn md5_lite(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
