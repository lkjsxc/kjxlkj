use kjxlkj::web::state::SearchHit;

use super::helpers::snippet_for;
use super::state::MockContentState;

pub fn search_hits(state: &MockContentState, query: &str, admin: bool) -> Vec<SearchHit> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return Vec::new();
    }
    let content = state.active.lock().expect("content lock poisoned");
    let mut hits = content
        .iter()
        .filter(|(_, entry)| admin || !entry.parsed.frontmatter.private)
        .filter_map(|(slug, entry)| {
            let title = entry.parsed.frontmatter.title.clone().unwrap_or_default();
            let haystack = format!("{slug} {title} {}", entry.parsed.body).to_lowercase();
            if !haystack.contains(&query) {
                return None;
            }
            Some(SearchHit {
                slug: slug.clone(),
                title: entry.parsed.frontmatter.title.clone(),
                snippet: snippet_for(&entry.parsed.body, &query),
                private: entry.parsed.frontmatter.private,
            })
        })
        .collect::<Vec<_>>();
    hits.sort_by(|a, b| a.slug.cmp(&b.slug));
    hits
}
