use sqlx::PgPool;

pub async fn is_locked(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row: Option<(bool,)> =
        sqlx::query_as("SELECT locked FROM setup_lock LIMIT 1")
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|r| r.0).unwrap_or(false))
}

pub async fn lock(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO setup_lock (id, locked) VALUES (1, true)
         ON CONFLICT (id) DO UPDATE SET locked = true",
    )
    .execute(pool)
    .await?;
    Ok(())
}
