/// In-memory SearchRepo implementation.
///
/// Spec: /docs/spec/domain/search.md
use crate::repo::SearchRepo;
use kjxlkj_domain::search::*;
use kjxlkj_domain::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Stored note text for lexical search simulation.
#[derive(Debug, Clone)]
struct NoteText {
    note_id: Uuid,
    workspace_id: Uuid,
    title: String,
    markdown: String,
    #[allow(dead_code)]
    updated_at: chrono::NaiveDateTime,
}

/// Thread-safe in-memory search store.
pub struct InMemorySearchRepo {
    texts: RwLock<HashMap<Uuid, NoteText>>,
    backlinks: RwLock<Vec<Backlink>>,
}

impl InMemorySearchRepo {
    pub fn new() -> Self {
        Self {
            texts: RwLock::new(HashMap::new()),
            backlinks: RwLock::new(Vec::new()),
        }
    }

    /// Index a note for search (called on create/update).
    pub fn index_note(
        &self,
        note_id: Uuid,
        workspace_id: Uuid,
        title: &str,
        markdown: &str,
    ) {
        let mut texts = self.texts.write().unwrap();
        texts.insert(
            note_id,
            NoteText {
                note_id,
                workspace_id,
                title: title.to_string(),
                markdown: markdown.to_string(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
        );
    }

    /// Remove a note from search index.
    pub fn remove_note(&self, note_id: Uuid) {
        let mut texts = self.texts.write().unwrap();
        texts.remove(&note_id);
    }

    /// Update backlink projections for a note.
    pub fn update_backlinks(
        &self,
        source_note_id: Uuid,
        source_title: &str,
        target_ids: Vec<Uuid>,
    ) {
        let mut blinks = self.backlinks.write().unwrap();
        blinks.retain(|b| b.source_note_id != source_note_id);
        let now = chrono::Utc::now().naive_utc();
        for target in target_ids {
            blinks.push(Backlink {
                source_note_id,
                target_note_id: target,
                source_title: source_title.to_string(),
                updated_at: now,
            });
        }
    }
}

impl Default for InMemorySearchRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchRepo for InMemorySearchRepo {
    fn search_notes(
        &self,
        query: &SearchQuery,
    ) -> Result<Vec<SearchResult>, DomainError> {
        let texts = self.texts.read().unwrap();
        let q_lower = query.q.to_lowercase();
        let limit = query.limit.unwrap_or(20) as usize;
        let mut results: Vec<SearchResult> = texts
            .values()
            .filter(|t| t.workspace_id == query.workspace_id)
            .filter(|t| {
                t.title.to_lowercase().contains(&q_lower)
                    || t.markdown.to_lowercase().contains(&q_lower)
            })
            .map(|t| {
                let title_match = t.title.to_lowercase().contains(&q_lower);
                let body_match = t.markdown.to_lowercase().contains(&q_lower);
                let score = if title_match { 0.9 } else if body_match { 0.5 } else { 0.0 };
                let snippet = if body_match {
                    let lower = t.markdown.to_lowercase();
                    let pos = lower.find(&q_lower).unwrap_or(0);
                    let start = pos.saturating_sub(30);
                    let end = (pos + q_lower.len() + 30).min(t.markdown.len());
                    t.markdown[start..end].to_string()
                } else {
                    String::new()
                };
                SearchResult {
                    note_id: t.note_id,
                    title: t.title.clone(),
                    snippet,
                    score_lexical: score,
                    score_semantic: 0.0,
                    score_final: score,
                }
            })
            .collect();
        results.sort_by(|a, b| {
            b.score_final
                .partial_cmp(&a.score_final)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(limit);
        Ok(results)
    }

    fn get_backlinks(
        &self,
        note_id: Uuid,
    ) -> Result<Vec<Backlink>, DomainError> {
        let blinks = self.backlinks.read().unwrap();
        let mut results: Vec<Backlink> = blinks
            .iter()
            .filter(|b| b.target_note_id == note_id)
            .cloned()
            .collect();
        results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_search() {
        let repo = InMemorySearchRepo::new();
        let ws_id = Uuid::new_v4();
        let id1 = Uuid::new_v4();
        repo.index_note(id1, ws_id, "Meeting Notes", "# Meeting\nDiscuss project.");
        let query = SearchQuery {
            q: "meeting".into(),
            workspace_id: ws_id,
            project_id: None,
            limit: None,
            mode: None,
        };
        let results = repo.search_notes(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].score_final > 0.0);
    }

    #[test]
    fn test_backlinks() {
        let repo = InMemorySearchRepo::new();
        let src = Uuid::new_v4();
        let tgt = Uuid::new_v4();
        repo.update_backlinks(src, "Source Note", vec![tgt]);
        let blinks = repo.get_backlinks(tgt).unwrap();
        assert_eq!(blinks.len(), 1);
        assert_eq!(blinks[0].source_title, "Source Note");
    }
}
