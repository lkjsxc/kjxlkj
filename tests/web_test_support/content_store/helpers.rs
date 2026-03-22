use std::io;

use kjxlkj::core::content::{revision_token, serialize_markdown_document, ParsedMarkdown};
use kjxlkj::error::AppError;

pub fn article_revision(article: &ParsedMarkdown) -> String {
    revision_token(&serialize_markdown_document(
        &article.frontmatter,
        &article.body,
    ))
}

pub fn missing(slug: &str) -> AppError {
    AppError::content_io(slug.to_owned(), io::Error::from(io::ErrorKind::NotFound))
}

pub fn snippet_for(body: &str, query: &str) -> String {
    let lower = body.to_lowercase();
    if let Some(index) = lower.find(query) {
        let start = index.saturating_sub(20);
        let end = (index + query.len() + 40).min(body.len());
        return body[start..end].to_owned();
    }
    body.chars().take(60).collect()
}
