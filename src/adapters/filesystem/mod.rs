use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};

use tokio::fs;

use crate::core::content::{
    ensure_unique_slug, is_markdown_file, is_visible, parse_markdown_document, path_for_slug,
    serialize_markdown_document, slug_from_stem, Frontmatter, ParsedMarkdown, VisibilityContext,
};
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct FilesystemAdapter {
    root: PathBuf,
}

impl FilesystemAdapter {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub async fn list_public_slugs(&self) -> Result<Vec<String>, AppError> {
        self.list_visible_slugs(VisibilityContext::Public).await
    }

    pub async fn list_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        self.list_visible_slugs(VisibilityContext::Admin).await
    }

    pub async fn read_article(&self, slug: &str) -> Result<ParsedMarkdown, AppError> {
        let path = path_for_slug(&self.root, slug)?;
        let markdown = fs::read_to_string(&path)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        parse_markdown_document(&markdown).map_err(AppError::from)
    }

    pub fn serialize_article(&self, frontmatter: &Frontmatter, body: &str) -> String {
        serialize_markdown_document(frontmatter, body)
    }

    async fn list_visible_slugs(
        &self,
        context: VisibilityContext,
    ) -> Result<Vec<String>, AppError> {
        let mut read_dir = match fs::read_dir(&self.root).await {
            Ok(read_dir) => read_dir,
            Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(source) => {
                return Err(AppError::content_io(
                    self.root.display().to_string(),
                    source,
                ))
            }
        };

        let mut seen = HashSet::new();
        let mut slugs = Vec::new();

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(|source| AppError::content_io(self.root.display().to_string(), source))?
        {
            let path = entry.path();
            if !is_markdown_file(&path) {
                continue;
            }

            let slug = file_slug(&path)?;
            ensure_unique_slug(&slug, &mut seen)?;

            let markdown = fs::read_to_string(&path)
                .await
                .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
            let parsed = parse_markdown_document(&markdown)?;
            if is_visible(&parsed.frontmatter, context) {
                slugs.push(slug);
            }
        }

        slugs.sort();
        Ok(slugs)
    }
}

fn file_slug(path: &Path) -> Result<String, AppError> {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            AppError::from(crate::core::content::ContentValidationError::InvalidSlug {
                value: path.display().to_string(),
            })
        })?;

    slug_from_stem(stem).map_err(AppError::from)
}
