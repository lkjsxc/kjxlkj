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
        .filter(|(_, parsed)| admin || !parsed.frontmatter.private)
        .filter_map(|(slug, parsed)| {
            let title = parsed.frontmatter.title.clone().unwrap_or_default();
            let haystack = format!("{slug} {title} {}", parsed.body).to_lowercase();
            if !haystack.contains(&query) {
                return None;
            }
            Some(SearchHit {
                slug: slug.clone(),
                title: parsed.frontmatter.title.clone(),
                snippet: snippet_for(&parsed.body, &query),
                private: parsed.frontmatter.private,
            })
        })
        .collect::<Vec<_>>();
    hits.sort_by(|a, b| a.slug.cmp(&b.slug));
    hits
}
