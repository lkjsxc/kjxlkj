use crate::models::tag::TagRow;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_tags(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<Vec<TagRow>, sqlx::Error> {
    sqlx::query_as::<_, TagRow>("SELECT note_id, tag FROM tags WHERE note_id = $1")
        .bind(note_id)
        .fetch_all(pool)
        .await
}

pub async fn replace_tags(
    pool: &PgPool,
    note_id: Uuid,
    tags: &[String],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM tags WHERE note_id = $1")
        .bind(note_id)
        .execute(&mut *tx)
        .await?;
    for tag in tags {
        sqlx::query("INSERT INTO tags (note_id, tag) VALUES ($1, $2)")
            .bind(note_id)
            .bind(tag)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await
}
