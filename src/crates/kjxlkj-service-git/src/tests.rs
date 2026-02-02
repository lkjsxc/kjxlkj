//! Tests for git service.

mod hunk_tests {
    use super::super::hunk::{Hunk, HunkKind};

    #[test]
    fn test_hunk_new() {
        let hunk = Hunk::new(HunkKind::Add, 10, 5);
        assert_eq!(hunk.kind, HunkKind::Add);
        assert_eq!(hunk.start, 10);
        assert_eq!(hunk.count, 5);
    }

    #[test]
    fn test_hunk_add() {
        let hunk = Hunk::add(5, 3);
        assert_eq!(hunk.kind, HunkKind::Add);
        assert_eq!(hunk.start, 5);
        assert_eq!(hunk.count, 3);
    }

    #[test]
    fn test_hunk_remove() {
        let hunk = Hunk::remove(8, 2);
        assert_eq!(hunk.kind, HunkKind::Remove);
        assert_eq!(hunk.start, 8);
        assert_eq!(hunk.count, 2);
    }

    #[test]
    fn test_hunk_change() {
        let hunk = Hunk::change(1, 10);
        assert_eq!(hunk.kind, HunkKind::Change);
        assert_eq!(hunk.start, 1);
        assert_eq!(hunk.count, 10);
    }

    #[test]
    fn test_hunk_kind_equality() {
        assert_eq!(HunkKind::Add, HunkKind::Add);
        assert_ne!(HunkKind::Add, HunkKind::Remove);
        assert_ne!(HunkKind::Remove, HunkKind::Change);
    }
}

mod repo_tests {
    use super::super::repo::{FileStatus, GitRepo};
    use std::path::PathBuf;

    #[test]
    fn test_repo_open_nonexistent() {
        let repo = GitRepo::open(PathBuf::from("/nonexistent/path"));
        assert!(repo.is_none());
    }

    #[test]
    fn test_file_status_equality() {
        assert_eq!(FileStatus::Modified, FileStatus::Modified);
        assert_ne!(FileStatus::Added, FileStatus::Deleted);
        assert_ne!(FileStatus::Renamed, FileStatus::Untracked);
    }

    #[test]
    fn test_file_status_debug() {
        let status = FileStatus::Modified;
        let debug = format!("{:?}", status);
        assert_eq!(debug, "Modified");
    }
}

mod hunk_kind_tests {
    use super::super::hunk::HunkKind;

    #[test]
    fn test_hunk_kind_debug() {
        assert_eq!(format!("{:?}", HunkKind::Add), "Add");
        assert_eq!(format!("{:?}", HunkKind::Remove), "Remove");
        assert_eq!(format!("{:?}", HunkKind::Change), "Change");
    }

    #[test]
    fn test_hunk_kind_clone() {
        let kind = HunkKind::Add;
        let cloned = kind;
        assert_eq!(kind, cloned);
    }
}
