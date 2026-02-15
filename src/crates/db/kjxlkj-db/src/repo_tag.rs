use kjxlkj_domain::ids::{NoteId, WorkspaceId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

/// Tag row per /docs/spec/domain/search.md.
#[derive(FromRow)]
pub struct TagRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
}

/// List all tags in a workspace.
pub async fn list_tags(
    pool: &PgPool,
    workspace_id: WorkspaceId,
) -> Result<Vec<TagRow>, sqlx::Error> {
    sqlx::query_as::<_, TagRow>(
        "SELECT id, workspace_id, name FROM tags
         WHERE workspace_id = $1 ORDER BY name",
    )
    .bind(workspace_id.0)
    .fetch_all(pool)
    .await
}

/// Find or create a tag by name within a workspace.
pub async fn find_or_create_tag(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    name: &str,
) -> Result<Uuid, sqlx::Error> {
    // Try insert, on conflict return existing
    let tag_id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO tags (id, workspace_id, name)
         VALUES ($1, $2, $3)
         ON CONFLICT (workspace_id, name) DO NOTHING",
    )
    .bind(tag_id)
    .bind(workspace_id.0)
    .bind(name)
    .execute(pool)
    .await?;

    // Fetch the actual ID (may be existing)
    let row = sqlx::query_as::<_, TagRow>(
        "SELECT id, workspace_id, name FROM tags
         WHERE workspace_id = $1 AND name = $2",
    )
    .bind(workspace_id.0)
    .bind(name)
    .fetch_one(pool)
    .await?;
    Ok(row.id)
}

/// Replace tags for a note atomically.
pub async fn replace_note_tags(
    pool: &PgPool,
    note_id: NoteId,
    tag_ids: &[Uuid],
) -> Result<(), sqlx::Error> {
    // Delete existing tags
    sqlx::query("DELETE FROM note_tags WHERE note_id = $1")
        .bind(note_id.0)
        .execute(pool)
        .await?;

    // Insert new tags
    for tag_id in tag_ids {
        sqlx::query(
            "INSERT INTO note_tags (note_id, tag_id) VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
        )
        .bind(note_id.0)
        .bind(tag_id)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// List tag names for a note.
pub async fn list_note_tag_names(
    pool: &PgPool,
    note_id: NoteId,
) -> Result<Vec<String>, sqlx::Error> {
    #[derive(FromRow)]
    struct NameRow {
        name: String,
    }
    let rows = sqlx::query_as::<_, NameRow>(
        "SELECT t.name FROM note_tags nt
         JOIN tags t ON t.id = nt.tag_id
         WHERE nt.note_id = $1
         ORDER BY t.name",
    )
    .bind(note_id.0)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.name).collect())
}
