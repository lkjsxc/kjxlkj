//! Tests for index/finder service.

#[cfg(test)]
mod tests {
    use crate::service::{fuzzy_score, IndexService};
    use crate::types::{FinderConfig, FinderQuery, SearchQuery};
    use std::path::PathBuf;

    #[test]
    fn fuzzy_score_exact_match() {
        let s = fuzzy_score("main", "main.rs");
        assert!(s.0 > 0.0);
    }

    #[test]
    fn fuzzy_score_partial_match() {
        let s = fuzzy_score("mn", "main.rs");
        assert!(s.0 > 0.0);
    }

    #[test]
    fn fuzzy_score_no_match() {
        let s = fuzzy_score("xyz", "main.rs");
        assert_eq!(s.0, 0.0);
    }

    #[test]
    fn fuzzy_score_case_insensitive() {
        let s = fuzzy_score("Main", "main.rs");
        assert!(s.0 > 0.0);
    }

    #[test]
    fn fuzzy_score_empty_pattern() {
        let s = fuzzy_score("", "main.rs");
        assert_eq!(s.0, 1.0);
    }

    #[test]
    fn fuzzy_word_boundary_bonus() {
        let s1 = fuzzy_score("mr", "main_result.rs");
        let s2 = fuzzy_score("mr", "xmmmmrmmm.rs");
        assert!(s1.0 > s2.0);
    }

    #[test]
    fn finder_files_query() {
        let mut svc = IndexService::new();
        svc.set_root(PathBuf::from("/workspace"));
        svc.set_file_cache(vec![
            PathBuf::from("src/main.rs"),
            PathBuf::from("src/lib.rs"),
            PathBuf::from("Cargo.toml"),
            PathBuf::from("README.md"),
        ]);

        let result = svc.find(&FinderQuery::Files {
            pattern: "main".into(),
        });
        assert!(!result.items.is_empty());
        assert_eq!(result.items[0].label, "main.rs");
    }

    #[test]
    fn finder_files_no_match() {
        let mut svc = IndexService::new();
        svc.set_file_cache(vec![PathBuf::from("src/lib.rs")]);

        let result = svc.find(&FinderQuery::Files {
            pattern: "xyz".into(),
        });
        assert!(result.items.is_empty());
    }

    #[test]
    fn finder_config_max_results() {
        let mut svc = IndexService::new();
        svc.set_config(FinderConfig {
            max_results: 2,
            respect_gitignore: true,
            include_hidden: false,
        });
        svc.set_file_cache(vec![
            PathBuf::from("a.rs"),
            PathBuf::from("ab.rs"),
            PathBuf::from("abc.rs"),
            PathBuf::from("abcd.rs"),
        ]);
        let result = svc.find(&FinderQuery::Files {
            pattern: "a".into(),
        });
        assert!(result.items.len() <= 2);
        assert!(result.truncated);
    }

    #[test]
    fn finder_buffers_returns_empty() {
        let svc = IndexService::new();
        let result = svc.find(&FinderQuery::Buffers {
            pattern: "test".into(),
        });
        assert!(result.items.is_empty());
    }

    #[test]
    fn grep_returns_empty() {
        let svc = IndexService::new();
        let matches = svc.grep(&SearchQuery {
            pattern: "TODO".into(),
            is_regex: false,
            case_sensitive: false,
            directory: None,
            file_glob: None,
        });
        assert!(matches.is_empty());
    }

    #[test]
    fn finder_total_candidates() {
        let mut svc = IndexService::new();
        svc.set_file_cache(vec![
            PathBuf::from("a.rs"),
            PathBuf::from("b.rs"),
            PathBuf::from("c.rs"),
        ]);
        let result = svc.find(&FinderQuery::Files { pattern: "".into() });
        assert_eq!(result.total_candidates, 3);
        assert_eq!(result.items.len(), 3);
    }
}
