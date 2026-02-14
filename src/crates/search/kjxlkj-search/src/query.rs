// Search query service per /docs/spec/domain/search.md
use kjxlkj_domain::types::NoteProjection;
use sqlx::PgPool;
use uuid::Uuid;

/// Full-text search across notes in a workspace.
pub async fn search_notes(
    pool: &PgPool,
    workspace_id: Uuid,
    query: &str,
) -> Result<Vec<NoteProjection>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, Option<Uuid>, String, String, i64, String, serde_json::Value)> =
        sqlx::query_as(
            "SELECT np.note_id, np.workspace_id, np.project_id, np.title, np.note_kind,
                    np.version, np.markdown, np.metadata_json
             FROM note_projections np
             JOIN note_streams ns ON ns.id = np.note_id
             WHERE np.workspace_id = $1
               AND ns.deleted_at IS NULL
               AND np.search_vector @@ plainto_tsquery('english', $2)
             ORDER BY ts_rank(np.search_vector, plainto_tsquery('english', $2)) DESC",
        )
        .bind(workspace_id)
        .bind(query)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| {
        use kjxlkj_domain::types::NoteKind;
        NoteProjection {
            note_id: r.0, workspace_id: r.1, project_id: r.2, title: r.3,
            note_kind: match r.4.as_str() {
                "settings" => NoteKind::Settings,
                "media_image" => NoteKind::MediaImage,
                "media_video" => NoteKind::MediaVideo,
                _ => NoteKind::Markdown,
            },
            version: r.5, markdown: r.6, metadata_json: r.7,
        }
    }).collect())
}
