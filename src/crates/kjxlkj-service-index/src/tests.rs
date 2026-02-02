//! Tests for index service.

#[cfg(test)]
mod finder_tests {
    use crate::Finder;
    use std::path::PathBuf;

    fn sample_files() -> Vec<PathBuf> {
        vec![
            PathBuf::from("src/main.rs"),
            PathBuf::from("src/lib.rs"),
            PathBuf::from("Cargo.toml"),
            PathBuf::from("README.md"),
        ]
    }

    #[test]
    fn test_finder_new() {
        let finder = Finder::new();
        assert!(finder.search("test").is_empty());
    }

    #[test]
    fn test_finder_add_files() {
        let mut finder = Finder::new();
        finder.add_files(sample_files());
        let results = finder.search("main");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_finder_search_case_insensitive() {
        let mut finder = Finder::new();
        finder.add_files(sample_files());
        let results = finder.search("README");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_finder_clear() {
        let mut finder = Finder::new();
        finder.add_files(sample_files());
        finder.clear();
        assert!(finder.search("main").is_empty());
    }

    #[test]
    fn test_finder_multiple_matches() {
        let mut finder = Finder::new();
        finder.add_files(sample_files());
        let results = finder.search(".rs");
        assert_eq!(results.len(), 2);
    }
}

#[cfg(test)]
mod index_tests {
    use crate::index::{FileEntry, Index};
    use std::path::PathBuf;

    fn sample_entry(name: &str) -> FileEntry {
        FileEntry {
            path: PathBuf::from(name),
            size: 100,
            modified: 1234567890,
        }
    }

    #[test]
    fn test_index_new() {
        let index = Index::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
    }

    #[test]
    fn test_index_add() {
        let mut index = Index::new();
        index.add(sample_entry("test.rs"));
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn test_index_get() {
        let mut index = Index::new();
        let path = PathBuf::from("test.rs");
        index.add(sample_entry("test.rs"));
        let entry = index.get(&path);
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().size, 100);
    }

    #[test]
    fn test_index_remove() {
        let mut index = Index::new();
        let path = PathBuf::from("test.rs");
        index.add(sample_entry("test.rs"));
        index.remove(&path);
        assert!(index.is_empty());
    }

    #[test]
    fn test_index_files_iter() {
        let mut index = Index::new();
        index.add(sample_entry("a.rs"));
        index.add(sample_entry("b.rs"));
        let files: Vec<_> = index.files().collect();
        assert_eq!(files.len(), 2);
    }
}

#[cfg(test)]
mod service_tests {
    use crate::IndexService;
    use std::path::PathBuf;

    #[test]
    fn test_service_new() {
        let svc = IndexService::new();
        assert!(svc.index().is_empty());
    }

    #[test]
    fn test_service_set_root() {
        let mut svc = IndexService::new();
        svc.set_root(PathBuf::from("/home/user/project"));
        assert!(svc.index().is_empty());
    }

    #[test]
    fn test_service_finder() {
        let svc = IndexService::new();
        let results = svc.finder().search("anything");
        assert!(results.is_empty());
    }
}
