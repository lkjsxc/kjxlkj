/// Backlink extraction and projection per /docs/spec/domain/search.md
use kjxlkj_domain::search::extract_wiki_links;

/// Extract backlink targets from markdown body.
/// Wiki links [[target]] must update backlink projections.
pub fn extract_targets(markdown: &str) -> Vec<String> {
    extract_wiki_links(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backlink_extraction() {
        let targets = extract_targets("Link to [[daily]] and [[project-a]].");
        assert_eq!(targets, vec!["daily", "project-a"]);
    }
}
