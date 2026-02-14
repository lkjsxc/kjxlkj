// Tags handler per /docs/spec/api/http.md
// GET /tags, PUT /notes/{id}/tags
use actix_web::{web, HttpResponse};
use kjxlkj_auth::middleware::AuthSession;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::ErrorBody;

/// GET /api/tags — list all tags
pub async fn list(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    match sqlx::query_as::<_, (Uuid, String)>("SELECT id, name FROM tags ORDER BY name")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => {
            let tags: Vec<serde_json::Value> = rows
                .iter()
                .map(|(id, name)| serde_json::json!({"id": id, "name": name}))
                .collect();
            HttpResponse::Ok().json(tags)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
            request_id: rid,
        }),
    }
}

/// PUT /api/notes/{id}/tags — replace tags for a note
pub async fn replace_tags(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<Vec<String>>,
    _auth: AuthSession,
) -> HttpResponse {
    let note_id = path.into_inner();
    let rid = Uuid::now_v7().to_string();
    let tag_names = body.into_inner();

    // Start transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(),
                message: e.to_string(),
                details: None,
                request_id: rid,
            });
        }
    };

    // Remove existing tags
    if let Err(e) = sqlx::query("DELETE FROM note_tags WHERE note_id = $1")
        .bind(note_id)
        .execute(&mut *tx)
        .await
    {
        return HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
            request_id: rid,
        });
    }

    // Insert/upsert tags and link
    for name in &tag_names {
        let tag_id = Uuid::now_v7();
        let _ = sqlx::query(
            "INSERT INTO tags (id, name) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
        )
        .bind(tag_id)
        .bind(name)
        .execute(&mut *tx)
        .await;

        let actual_id: (Uuid,) =
            sqlx::query_as("SELECT id FROM tags WHERE name = $1")
                .bind(name)
                .fetch_one(&mut *tx)
                .await
                .unwrap_or((tag_id,));

        let _ = sqlx::query(
            "INSERT INTO note_tags (note_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(note_id)
        .bind(actual_id.0)
        .execute(&mut *tx)
        .await;
    }

    if let Err(e) = tx.commit().await {
        return HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
            request_id: rid,
        });
    }

    HttpResponse::Ok().json(serde_json::json!({"tags": tag_names}))
}
