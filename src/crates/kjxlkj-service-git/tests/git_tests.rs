use kjxlkj_service_git::{
    compute_gutter_signs, count_changes, parse_blame_output, parse_diff_hunks, parse_log,
    parse_status_line, BranchInfo, DiffHunk, FileStatus, GutterSign, StatusEntry,
};

// --- File status variants ---

#[test]
fn status_modified_unstaged() {
    let e = parse_status_line(" M src/main.rs").unwrap();
    assert_eq!(e.status, FileStatus::Modified);
    assert!(!e.staged);
    assert_eq!(e.path, "src/main.rs");
}

#[test]
fn status_added_staged() {
    let e = parse_status_line("A  new_file.rs").unwrap();
    assert_eq!(e.status, FileStatus::Added);
    assert!(e.staged);
}

#[test]
fn status_deleted_staged() {
    let e = parse_status_line("D  old.rs").unwrap();
    assert_eq!(e.status, FileStatus::Deleted);
    assert!(e.staged);
}

#[test]
fn status_untracked() {
    let e = parse_status_line("?? unknown.txt").unwrap();
    assert_eq!(e.status, FileStatus::Untracked);
    assert!(!e.staged);
}

#[test]
fn status_ignored() {
    let e = parse_status_line("!! target/debug").unwrap();
    assert_eq!(e.status, FileStatus::Ignored);
}

#[test]
fn status_short_line_none() {
    assert!(parse_status_line("AB").is_none());
}

#[test]
fn status_entry_creation() {
    let e = StatusEntry {
        path: "test.rs".into(),
        status: FileStatus::Renamed,
        staged: true,
    };
    assert_eq!(e.path, "test.rs");
    assert!(e.staged);
}

// --- Diff hunk parsing ---

#[test]
fn parse_diff_single_hunk() {
    let diff = "@@ -1,3 +1,4 @@\n context\n+added\n context\n context\n";
    let hunks = parse_diff_hunks(diff);
    assert_eq!(hunks.len(), 1);
    assert_eq!(hunks[0].old_start, 1);
    assert_eq!(hunks[0].new_start, 1);
    assert_eq!(hunks[0].new_count, 4);
}

#[test]
fn parse_diff_multiple_hunks() {
    let diff = "@@ -1,2 +1,2 @@\n-old\n+new\n@@ -10,1 +10,2 @@\n line\n+added\n";
    let hunks = parse_diff_hunks(diff);
    assert_eq!(hunks.len(), 2);
}

#[test]
fn gutter_sign_added() {
    let diff = "@@ -1,1 +1,2 @@\n context\n+new line\n";
    let hunks = parse_diff_hunks(diff);
    let signs = compute_gutter_signs(&hunks);
    assert!(signs.iter().any(|(_, s)| *s == GutterSign::Added));
}

// --- Blame parsing ---

#[test]
fn parse_blame_entries() {
    let output = "abc123abc123abc123abc123abc123abc123abc1 1 1 1\nauthor Alice\n\tcontent\n";
    let entries = parse_blame_output(output);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].author, "Alice");
}

// --- Log parsing ---

#[test]
fn parse_log_two_entries() {
    let output = "abc123\nAlice\n2025-01-01\nInitial\ndef456\nBob\n2025-01-02\nFix\n";
    let entries = parse_log(output);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].author, "Alice");
    assert_eq!(entries[1].message, "Fix");
}

#[test]
fn parse_log_empty() {
    assert!(parse_log("").is_empty());
}

// --- Branch info ---

#[test]
fn branch_info_creation() {
    let b = BranchInfo {
        name: "main".into(),
        is_current: true,
        upstream: Some("origin/main".into()),
    };
    assert!(b.is_current);
    assert_eq!(b.upstream.as_deref(), Some("origin/main"));
}

// --- Change counting ---

#[test]
fn count_changes_basic() {
    use kjxlkj_service_git::git_diff::DiffLineKind;
    use kjxlkj_service_git::DiffLine as DL;
    let hunks = vec![DiffHunk {
        old_start: 1,
        old_count: 2,
        new_start: 1,
        new_count: 3,
        lines: vec![
            DL {
                kind: DiffLineKind::Added,
                content: "a".into(),
            },
            DL {
                kind: DiffLineKind::Added,
                content: "b".into(),
            },
            DL {
                kind: DiffLineKind::Removed,
                content: "c".into(),
            },
        ],
    }];
    assert_eq!(count_changes(&hunks), (2, 1));
}
