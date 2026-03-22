use actix_web::web;

use crate::core::content::{revision_token, serialize_markdown_document};
use crate::error::AppError;
use crate::web::state::WebState;

use super::admin_fragments::{AdminListItem, EditorDocument};

pub async fn load_admin_items(state: &web::Data<WebState>) -> Result<Vec<AdminListItem>, AppError> {
    let slugs = state.content_store.list_admin_slugs().await?;
    let mut items = Vec::with_capacity(slugs.len());

    for slug in slugs {
        match state.content_store.read_article(&slug).await {
            Ok(parsed) => items.push(AdminListItem {
                slug,
                private: parsed.frontmatter.private,
            }),
            Err(error) if is_missing_article(&error) => {}
            Err(error) => return Err(error),
        }
    }

    Ok(items)
}

pub async fn load_editor_document(
    state: &web::Data<WebState>,
    slug: &str,
) -> Result<EditorDocument, AppError> {
    let parsed = state.content_store.read_article(slug).await?;
    let markdown = serialize_markdown_document(&parsed.frontmatter, &parsed.body);
    Ok(EditorDocument {
        slug: slug.to_owned(),
        title: parsed.frontmatter.title,
        body: parsed.body,
        private: parsed.frontmatter.private,
        revision: revision_token(&markdown),
    })
}

pub fn is_missing_article(error: &AppError) -> bool {
    matches!(
        error,
        AppError::ContentIo { source, .. } if source.kind() == std::io::ErrorKind::NotFound
    )
}
