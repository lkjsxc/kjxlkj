use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use kjxlkj::core::content::{Frontmatter, ParsedMarkdown};
use kjxlkj::error::AppError;
use kjxlkj::web::state::{SaveConflict, SaveOutcome};

use super::helpers::{article_revision, missing};

#[derive(Clone, Default)]
pub struct MockContentState {
    pub active: Arc<Mutex<HashMap<String, ParsedMarkdown>>>,
    pub trash: Arc<Mutex<HashMap<String, ParsedMarkdown>>>,
}

impl MockContentState {
    pub fn read(&self, slug: &str) -> Option<ParsedMarkdown> {
        self.active
            .lock()
            .expect("content lock poisoned")
            .get(slug)
            .cloned()
    }

    pub fn upsert(&self, slug: &str, title: Option<String>, body: &str, private: bool) {
        self.active.lock().expect("content lock poisoned").insert(
            slug.to_owned(),
            ParsedMarkdown {
                frontmatter: Frontmatter { title, private },
                body: body.to_owned(),
            },
        );
    }

    pub fn save(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
        last_known_revision: Option<&str>,
    ) -> Result<SaveOutcome, AppError> {
        let persisted_revision = self
            .active
            .lock()
            .expect("content lock poisoned")
            .get(slug)
            .map(article_revision);
        let submitted_revision = last_known_revision
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);
        self.upsert(slug, title, body, private);
        let saved_revision = self
            .active
            .lock()
            .expect("content lock poisoned")
            .get(slug)
            .map(article_revision)
            .ok_or_else(|| missing(slug))?;
        let conflict = match (submitted_revision, persisted_revision) {
            (Some(submitted), Some(persisted)) if submitted != persisted => Some(SaveConflict {
                persisted_revision: persisted,
                submitted_revision: submitted,
            }),
            _ => None,
        };
        Ok(SaveOutcome {
            revision: saved_revision,
            conflict,
        })
    }

    pub fn rename(&self, slug: &str, new_slug: &str) {
        let mut content = self.active.lock().expect("content lock poisoned");
        if let Some(value) = content.remove(slug) {
            content.insert(new_slug.to_owned(), value);
        }
    }

    pub fn move_to_trash(&self, slug: &str) {
        let mut active = self.active.lock().expect("content lock poisoned");
        if let Some(value) = active.remove(slug) {
            self.trash
                .lock()
                .expect("trash lock poisoned")
                .insert(slug.to_owned(), value);
        }
    }

    pub fn toggle_private(&self, slug: &str) -> Option<bool> {
        let mut content = self.active.lock().expect("content lock poisoned");
        let article = content.get_mut(slug)?;
        article.frontmatter.private = !article.frontmatter.private;
        Some(article.frontmatter.private)
    }

    pub fn list_trash_slugs(&self) -> Vec<String> {
        let mut slugs = self
            .trash
            .lock()
            .expect("trash lock poisoned")
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        slugs.sort();
        slugs
    }

    pub fn restore(&self, slug: &str) {
        let mut trash = self.trash.lock().expect("trash lock poisoned");
        if let Some(value) = trash.remove(slug) {
            self.active
                .lock()
                .expect("content lock poisoned")
                .insert(slug.to_owned(), value);
        }
    }

    pub fn remove_from_trash(&self, slug: &str) -> bool {
        self.trash
            .lock()
            .expect("trash lock poisoned")
            .remove(slug)
            .is_some()
    }

    pub fn insert_simple(&self, slug: &str, private: bool, body: &str) {
        self.upsert(slug, None, body, private);
    }

    pub fn list_slugs(&self, include_private: bool) -> Vec<String> {
        let hidden = self
            .trash
            .lock()
            .expect("trash lock poisoned")
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        let mut slugs = self
            .active
            .lock()
            .expect("content lock poisoned")
            .iter()
            .filter_map(|(slug, parsed)| {
                if hidden.contains(slug) || (!include_private && parsed.frontmatter.private) {
                    None
                } else {
                    Some(slug.to_owned())
                }
            })
            .collect::<Vec<_>>();
        slugs.sort();
        slugs
    }
}
