//! Tests for git service.

#[cfg(test)]
mod tests {
    use crate::service::GitService;
    use crate::types::{DiffHunk, FileStatus, GitCommand, GitNotification, GitSign, StatusEntry};
    use std::path::PathBuf;

    #[test]
    fn git_service_default_available() {
        let svc = GitService::new();
        assert!(svc.is_available());
        assert!(svc.root().is_none());
    }

    #[test]
    fn git_service_set_root() {
        let mut svc = GitService::new();
        svc.set_root(PathBuf::from("/repo"));
        assert_eq!(svc.root(), Some(&PathBuf::from("/repo")));
    }

    #[test]
    fn git_unavailable_returns_error() {
        let mut svc = GitService::new();
        svc.set_unavailable();
        let notification = svc.handle_command(&GitCommand::RefreshStatus);
        match notification {
            GitNotification::Error(msg) => {
                assert!(msg.contains("not found"));
            }
            _ => panic!("expected error"),
        }
    }

    #[test]
    fn git_status_cache() {
        let mut svc = GitService::new();
        svc.set_status(vec![
            StatusEntry {
                path: PathBuf::from("src/main.rs"),
                status: FileStatus::Modified,
                staged: None,
            },
            StatusEntry {
                path: PathBuf::from("README.md"),
                status: FileStatus::Added,
                staged: Some(FileStatus::Added),
            },
        ]);
        assert_eq!(svc.cached_status().len(), 2);
        let resp = svc.handle_command(&GitCommand::RefreshStatus);
        match resp {
            GitNotification::StatusUpdated(entries) => {
                assert_eq!(entries.len(), 2);
            }
            _ => panic!("expected status update"),
        }
    }

    #[test]
    fn git_hunks_and_signs() {
        let mut svc = GitService::new();
        let file = PathBuf::from("src/lib.rs");
        svc.set_hunks(
            file.clone(),
            vec![
                DiffHunk {
                    start_line: 10,
                    count: 3,
                    sign: GitSign::Changed,
                    old_text: vec!["old".into()],
                    new_text: vec!["new".into()],
                },
                DiffHunk {
                    start_line: 20,
                    count: 1,
                    sign: GitSign::Added,
                    old_text: vec![],
                    new_text: vec!["added".into()],
                },
            ],
        );
        assert_eq!(svc.sign_for_line(&file, 11), Some(GitSign::Changed));
        assert_eq!(svc.sign_for_line(&file, 20), Some(GitSign::Added));
        assert_eq!(svc.sign_for_line(&file, 5), None);
    }

    #[test]
    fn git_hunk_navigation() {
        let mut svc = GitService::new();
        let file = PathBuf::from("nav.rs");
        svc.set_hunks(
            file.clone(),
            vec![
                DiffHunk {
                    start_line: 5,
                    count: 2,
                    sign: GitSign::Changed,
                    old_text: vec![],
                    new_text: vec![],
                },
                DiffHunk {
                    start_line: 15,
                    count: 1,
                    sign: GitSign::Added,
                    old_text: vec![],
                    new_text: vec![],
                },
                DiffHunk {
                    start_line: 30,
                    count: 3,
                    sign: GitSign::Removed,
                    old_text: vec![],
                    new_text: vec![],
                },
            ],
        );
        assert_eq!(svc.next_hunk(&file, 1), Some(5));
        assert_eq!(svc.next_hunk(&file, 10), Some(15));
        assert_eq!(svc.next_hunk(&file, 30), None);
        assert_eq!(svc.prev_hunk(&file, 30), Some(15));
        assert_eq!(svc.prev_hunk(&file, 10), Some(5));
        assert_eq!(svc.prev_hunk(&file, 1), None);
    }

    #[test]
    fn git_blame_returns_empty() {
        let mut svc = GitService::new();
        let resp = svc.handle_command(&GitCommand::Blame {
            file: PathBuf::from("test.rs"),
        });
        match resp {
            GitNotification::BlameResult { entries, .. } => {
                assert!(entries.is_empty());
            }
            _ => panic!("expected blame result"),
        }
    }

    #[test]
    fn git_diff_returns_empty() {
        let mut svc = GitService::new();
        let resp = svc.handle_command(&GitCommand::Diff {
            file: PathBuf::from("test.rs"),
        });
        match resp {
            GitNotification::DiffContent { content, .. } => {
                assert!(content.is_empty());
            }
            _ => panic!("expected diff content"),
        }
    }
}
