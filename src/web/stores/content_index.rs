use crate::app_state::AppState;
use crate::core::content::parse_markdown_document;
use crate::error::AppError;

pub async fn reindex_all(app_state: &AppState) -> Result<(), AppError> {
    app_state.postgres.search().clear_index().await?;
    let slugs = app_state.filesystem.list_admin_slugs().await?;
    for slug in slugs {
        let parsed = app_state.filesystem.read_article(&slug).await?;
        app_state
            .postgres
            .search()
            .index_article(
                &slug,
                parsed.frontmatter.title.as_deref(),
                &parsed.body,
                parsed.frontmatter.private,
                false,
            )
            .await?;
    }
    Ok(())
}

pub async fn index_saved_article(
    app_state: &AppState,
    slug: &str,
    markdown: &str,
) -> Result<(), AppError> {
    let parsed = parse_markdown_document(markdown)?;
    app_state
        .postgres
        .search()
        .index_article(
            slug,
            parsed.frontmatter.title.as_deref(),
            &parsed.body,
            parsed.frontmatter.private,
            false,
        )
        .await
}
