use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use kjxlkj::core::content::{Frontmatter, ParsedMarkdown};
use kjxlkj::error::AppError;
use kjxlkj::web::state::ContentStore;

#[derive(Clone, Default)]
pub struct MockContentStore {
    inner: Arc<Mutex<HashMap<String, ParsedMarkdown>>>,
}

#[async_trait]
impl ContentStore for MockContentStore {
    async fn list_public_slugs(&self) -> Result<Vec<String>, AppError> {
        Ok(self.list_slugs(false))
    }

    async fn list_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        Ok(self.list_slugs(true))
    }

    async fn read_article(&self, slug: &str) -> Result<ParsedMarkdown, AppError> {
        let content = self.inner.lock().expect("content lock poisoned");
        content.get(slug).cloned().ok_or_else(|| {
            AppError::content_io(slug.to_owned(), io::Error::from(io::ErrorKind::NotFound))
        })
    }

    async fn create_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError> {
        self.inner.lock().expect("content lock poisoned").insert(
            slug.to_owned(),
            ParsedMarkdown {
                frontmatter: Frontmatter { title, private },
                body: body.to_owned(),
            },
        );
        Ok(())
    }

    async fn save_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError> {
        self.create_article(slug, title, body, private).await
    }

    async fn rename_article(&self, slug: &str, new_slug: &str) -> Result<(), AppError> {
        let mut content = self.inner.lock().expect("content lock poisoned");
        if let Some(value) = content.remove(slug) {
            content.insert(new_slug.to_owned(), value);
        }
        Ok(())
    }

    async fn delete_article(&self, slug: &str) -> Result<(), AppError> {
        self.inner
            .lock()
            .expect("content lock poisoned")
            .remove(slug);
        Ok(())
    }

    async fn toggle_article_private(&self, slug: &str) -> Result<bool, AppError> {
        let mut content = self.inner.lock().expect("content lock poisoned");
        let article = content.get_mut(slug).ok_or_else(|| {
            AppError::content_io(slug.to_owned(), io::Error::from(io::ErrorKind::NotFound))
        })?;
        article.frontmatter.private = !article.frontmatter.private;
        Ok(article.frontmatter.private)
    }
}

impl MockContentStore {
    pub fn insert_article(&self, slug: &str, private: bool, body: &str) {
        self.inner.lock().expect("content lock poisoned").insert(
            slug.to_owned(),
            ParsedMarkdown {
                frontmatter: Frontmatter {
                    title: None,
                    private,
                },
                body: body.to_owned(),
            },
        );
    }

    fn list_slugs(&self, include_private: bool) -> Vec<String> {
        let mut slugs = self
            .inner
            .lock()
            .expect("content lock poisoned")
            .iter()
            .filter_map(|(slug, parsed)| {
                if include_private || !parsed.frontmatter.private {
                    Some(slug.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        slugs.sort();
        slugs
    }
}
