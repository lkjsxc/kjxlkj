use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use kjxlkj::core::content::{Frontmatter, ParsedMarkdown};
use kjxlkj::error::AppError;
use kjxlkj::web::state::{
    ArticleHistory, ArticleHistoryEntry, ArticleNavigation, ArticleSummary, SaveConflict,
    SaveOutcome,
};

use super::helpers::{article_revision, missing};

#[derive(Clone, Default)]
pub struct MockContentState {
    pub active: Arc<Mutex<HashMap<String, ArticleEntry>>>,
    pub trash: Arc<Mutex<HashMap<String, ArticleEntry>>>,
    pub revision_seed: Arc<Mutex<u64>>,
}

#[derive(Clone)]
pub struct ArticleEntry {
    pub parsed: ParsedMarkdown,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub history: Vec<ArticleHistoryEntry>,
    pub revisions: HashMap<String, ParsedMarkdown>,
}

impl MockContentState {
    pub fn read(&self, slug: &str) -> Option<ParsedMarkdown> {
        self.active
            .lock()
            .expect("content lock poisoned")
            .get(slug)
            .map(|entry| entry.parsed.clone())
    }

    pub fn upsert(&self, slug: &str, title: Option<String>, body: &str, private: bool) {
        let now = Utc::now();
        let parsed = ParsedMarkdown {
            frontmatter: Frontmatter { title, private },
            body: body.to_owned(),
        };
        let commit_id = self.next_commit_id(slug);
        let mut active = self.active.lock().expect("content lock poisoned");
        let created_at = active
            .get(slug)
            .map(|entry| entry.created_at)
            .unwrap_or(now);
        let mut history = active
            .get(slug)
            .map(|entry| entry.history.clone())
            .unwrap_or_default();
        let mut revisions = active
            .get(slug)
            .map(|entry| entry.revisions.clone())
            .unwrap_or_default();
        history.insert(
            0,
            ArticleHistoryEntry {
                commit_id: commit_id.clone(),
                committed_at: now,
                message: "autosave".to_owned(),
            },
        );
        revisions.insert(commit_id, parsed.clone());
        active.insert(
            slug.to_owned(),
            ArticleEntry {
                parsed,
                created_at,
                updated_at: now,
                history,
                revisions,
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
            .map(|entry| article_revision(&entry.parsed));
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
            .map(|entry| article_revision(&entry.parsed))
            .ok_or_else(|| missing(slug))?;
        let updated_at = self
            .active
            .lock()
            .expect("content lock poisoned")
            .get(slug)
            .map(|entry| entry.updated_at)
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
            updated_at,
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

    pub fn set_timeline(
        &self,
        slug: &str,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> bool {
        let mut active = self.active.lock().expect("content lock poisoned");
        let Some(entry) = active.get_mut(slug) else {
            return false;
        };
        entry.created_at = created_at;
        entry.updated_at = updated_at;
        true
    }

    pub fn list_slugs(&self, include_private: bool) -> Vec<String> {
        self.list_articles(include_private)
            .into_iter()
            .map(|article| article.slug)
            .collect::<Vec<_>>()
    }

    pub fn list_articles(&self, include_private: bool) -> Vec<ArticleSummary> {
        let hidden = self
            .trash
            .lock()
            .expect("trash lock poisoned")
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        let mut items = self
            .active
            .lock()
            .expect("content lock poisoned")
            .iter()
            .filter_map(|(slug, entry)| {
                if hidden.contains(slug) || (!include_private && entry.parsed.frontmatter.private) {
                    None
                } else {
                    Some(ArticleSummary {
                        slug: slug.clone(),
                        title: entry.parsed.frontmatter.title.clone(),
                        private: entry.parsed.frontmatter.private,
                        created_at: entry.created_at,
                        updated_at: entry.updated_at,
                    })
                }
            })
            .collect::<Vec<_>>();
        items.sort_by(|a, b| a.created_at.cmp(&b.created_at).then(a.slug.cmp(&b.slug)));
        items
    }

    pub fn navigation_for(&self, slug: &str, include_private: bool) -> ArticleNavigation {
        let items = self.list_articles(include_private);
        let index = items.iter().position(|item| item.slug == slug);
        let previous_slug = index
            .and_then(|i| i.checked_sub(1))
            .map(|i| items[i].slug.clone());
        let next_slug = index
            .and_then(|i| i.checked_add(1))
            .filter(|i| *i < items.len())
            .map(|i| items[i].slug.clone());
        ArticleNavigation {
            previous_slug,
            next_slug,
        }
    }

    pub fn history_for(&self, slug: &str) -> ArticleHistory {
        let entries = self
            .active
            .lock()
            .expect("content lock poisoned")
            .get(slug)
            .map(|entry| entry.history.clone())
            .unwrap_or_default();
        ArticleHistory {
            slug: slug.to_owned(),
            entries,
        }
    }

    pub fn restore_version(&self, slug: &str, commit_id: &str) -> Result<(), AppError> {
        let mut active = self.active.lock().expect("content lock poisoned");
        let Some(article) = active.get_mut(slug) else {
            return Err(missing(slug));
        };
        if let Some(parsed) = article.revisions.get(commit_id).cloned() {
            article.parsed = parsed;
            article.updated_at = Utc::now();
            return Ok(());
        }
        Err(missing(slug))
    }

    fn next_commit_id(&self, slug: &str) -> String {
        let mut revision_seed = self
            .revision_seed
            .lock()
            .expect("revision seed lock poisoned");
        *revision_seed = revision_seed.saturating_add(1);
        format!("{slug}-{revision_seed:020}")
    }
}
