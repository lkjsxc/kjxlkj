use crate::core::EmbedMetadata;
use crate::error::AppError;
use crate::web::db::{self, DbPool};
use std::collections::HashMap;

pub async fn external_embed_cache(
    pool: &DbPool,
    urls: &[String],
) -> Result<HashMap<String, EmbedMetadata>, AppError> {
    let rows = db::list_external_embeds(pool, urls).await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            (
                row.url,
                EmbedMetadata {
                    provider: row.provider,
                    title: row.title,
                    description: row.description,
                    site_name: row.site_name,
                    author_name: row.author_name,
                    thumbnail_url: row.thumbnail_url,
                },
            )
        })
        .collect())
}
