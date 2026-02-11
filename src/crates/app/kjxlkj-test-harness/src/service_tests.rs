//! Service integration tests for cross-surface workflows.

use kjxlkj_service_lsp::{LspService, ServerState};
use kjxlkj_service_git::{GitService, FileStatus, Hunk, HunkKind};
use kjxlkj_service_index::IndexService;
use std::path::PathBuf;

/// SVC-01: LSP service can be started for Rust files
#[test]
fn svc_01_lsp_rust_server_lifecycle() {
    let mut lsp = LspService::new();
    
    // Find config for Rust
    let config = lsp.find_server_for_extension("rs");
    assert!(config.is_some(), "Should have config for Rust");
    
    // Start server
    assert_eq!(lsp.server_state("rust"), ServerState::Stopped);
    lsp.start_server("rust").unwrap();
    assert_eq!(lsp.server_state("rust"), ServerState::Running);
    
    // Stop server
    lsp.stop_server("rust").unwrap();
    assert_eq!(lsp.server_state("rust"), ServerState::Stopped);
}

/// SVC-02: LSP service handles multiple language servers
#[test]
fn svc_02_lsp_multiple_servers() {
    let mut lsp = LspService::new();
    
    assert!(lsp.find_server_for_extension("rs").is_some());
    assert!(lsp.find_server_for_extension("ts").is_some());
    assert!(lsp.find_server_for_extension("py").is_some());
    
    // Can run multiple servers
    lsp.start_server("rust").unwrap();
    lsp.start_server("python").unwrap();
    
    assert_eq!(lsp.server_state("rust"), ServerState::Running);
    assert_eq!(lsp.server_state("python"), ServerState::Running);
}

/// SVC-03: Git service detects repository
#[test]
fn svc_03_git_repo_detection() {
    let git = GitService::new();
    
    // Current directory should be a git repo (we're in kjxlkj)
    let cwd = std::env::current_dir().unwrap();
    let is_repo = git.is_repo(&cwd);
    
    // This test running from the project means we're in a git repo
    assert!(is_repo, "Current directory should be a git repo");
}

/// SVC-04: Git file status types are comprehensive
#[test]
fn svc_04_git_file_status_types() {
    let statuses = [
        FileStatus::Untracked,
        FileStatus::Modified,
        FileStatus::Staged,
        FileStatus::StagedModified,
        FileStatus::Deleted,
        FileStatus::Renamed,
        FileStatus::Clean,
        FileStatus::Ignored,
    ];
    
    for (i, status) in statuses.iter().enumerate() {
        for (j, other) in statuses.iter().enumerate() {
            if i == j {
                assert_eq!(status, other);
            } else {
                assert_ne!(status, other);
            }
        }
    }
}

/// SVC-05: Index service file search
#[test]
fn svc_05_index_file_search() {
    let mut index = IndexService::new();
    index.init(PathBuf::from("/workspace"));
    
    // Add various files
    index.add_file(PathBuf::from("src/main.rs"), 1000, 1234567890);
    index.add_file(PathBuf::from("src/lib.rs"), 500, 1234567890);
    index.add_file(PathBuf::from("src/utils/helper.rs"), 300, 1234567890);
    index.add_file(PathBuf::from("Cargo.toml"), 200, 1234567890);
    index.add_file(PathBuf::from("README.md"), 100, 1234567890);
    
    // Search by extension
    let rs_files = index.search_files(".rs");
    assert_eq!(rs_files.len(), 3);
    
    // Search by name
    let main_files = index.search_files("main");
    assert_eq!(main_files.len(), 1);
    
    // Search by partial name
    let readme = index.search_files("readme");
    assert_eq!(readme.len(), 1);
}

/// SVC-06: Index service indexing state
#[test]
fn svc_06_index_indexing_state() {
    let mut index = IndexService::new();
    
    assert!(!index.is_indexing());
    
    index.set_indexing(true);
    assert!(index.is_indexing());
    
    // Simulate adding files during indexing
    index.add_file(PathBuf::from("file1.rs"), 100, 0);
    index.add_file(PathBuf::from("file2.rs"), 200, 0);
    
    assert_eq!(index.file_count(), 2);
    
    index.set_indexing(false);
    assert!(!index.is_indexing());
}

/// SVC-07: Git hunk tracking
#[test]
fn svc_07_git_hunk_types() {
    let added_hunk = Hunk {
        start: 10,
        count: 5,
        kind: HunkKind::Added,
    };
    
    let removed_hunk = Hunk {
        start: 20,
        count: 3,
        kind: HunkKind::Removed,
    };
    
    let modified_hunk = Hunk {
        start: 30,
        count: 7,
        kind: HunkKind::Modified,
    };
    
    assert_eq!(added_hunk.kind, HunkKind::Added);
    assert_eq!(removed_hunk.kind, HunkKind::Removed);
    assert_eq!(modified_hunk.kind, HunkKind::Modified);
    
    assert_eq!(added_hunk.start, 10);
    assert_eq!(removed_hunk.count, 3);
}
