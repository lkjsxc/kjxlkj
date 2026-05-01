use super::{DbPool, ExternalEmbed};
use crate::error::AppError;
use chrono::{Duration, Utc};

pub async fn list_external_embeds(
    pool: &DbPool,
    urls: &[String],
) -> Result<Vec<ExternalEmbed>, AppError> {
    if urls.is_empty() {
        return Ok(Vec::new());
    }
    client(pool)
        .await?
        .query(
            "SELECT url, provider, title, description, site_name, author_name, thumbnail_url \
             FROM external_embed_cache WHERE url = ANY($1) AND fetched_at IS NOT NULL",
            &[&urls],
        )
        .await
        .map(|rows| rows.into_iter().map(row_to_embed).collect())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn stale_external_embed_urls(
    pool: &DbPool,
    urls: &[String],
) -> Result<Vec<String>, AppError> {
    if urls.is_empty() {
        return Ok(Vec::new());
    }
    let rows = client(pool)
        .await?
        .query(
            "SELECT input.url FROM unnest($1::TEXT[]) AS input(url) \
             LEFT JOIN external_embed_cache cache ON cache.url = input.url \
             WHERE cache.url IS NULL OR cache.expires_at IS NULL OR cache.expires_at <= NOW()",
            &[&urls],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(rows.into_iter().map(|row| row.get("url")).collect())
}

pub async fn upsert_external_embed(pool: &DbPool, embed: &ExternalEmbed) -> Result<(), AppError> {
    let expires_at = Utc::now() + Duration::days(14);
    client(pool)
        .await?
        .execute(
            "INSERT INTO external_embed_cache \
             (url_hash, url, provider, kind, title, description, site_name, author_name, \
              thumbnail_url, fetched_at, expires_at, last_error, error_at) \
             VALUES (encode(digest($1, 'sha256'), 'hex'), $1, $2, 'bookmark', $3, $4, $5, $6, $7, NOW(), $8, NULL, NULL) \
             ON CONFLICT (url_hash) DO UPDATE SET provider = EXCLUDED.provider, title = EXCLUDED.title, \
             description = EXCLUDED.description, site_name = EXCLUDED.site_name, author_name = EXCLUDED.author_name, \
             thumbnail_url = EXCLUDED.thumbnail_url, fetched_at = NOW(), expires_at = EXCLUDED.expires_at, \
             last_error = NULL, error_at = NULL, updated_at = NOW()",
            &[
                &embed.url,
                &embed.provider,
                &embed.title,
                &embed.description,
                &embed.site_name,
                &embed.author_name,
                &embed.thumbnail_url,
                &expires_at,
            ],
        )
        .await
        .map(|_| ())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn upsert_external_embed_error(
    pool: &DbPool,
    url: &str,
    provider: &str,
    error: &str,
) -> Result<(), AppError> {
    let expires_at = Utc::now() + Duration::hours(6);
    let error = error.chars().take(240).collect::<String>();
    client(pool)
        .await?
        .execute(
            "INSERT INTO external_embed_cache (url_hash, url, provider, kind, expires_at, last_error, error_at) \
             VALUES (encode(digest($1, 'sha256'), 'hex'), $1, $2, 'bookmark', $3, $4, NOW()) \
             ON CONFLICT (url_hash) DO UPDATE SET expires_at = EXCLUDED.expires_at, \
             last_error = EXCLUDED.last_error, error_at = NOW(), updated_at = NOW()",
            &[&url, &provider, &expires_at, &error],
        )
        .await
        .map(|_| ())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

fn row_to_embed(row: tokio_postgres::Row) -> ExternalEmbed {
    ExternalEmbed {
        url: row.get("url"),
        provider: row.get("provider"),
        title: row.get("title"),
        description: row.get("description"),
        site_name: row.get("site_name"),
        author_name: row.get("author_name"),
        thumbnail_url: row.get("thumbnail_url"),
    }
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
