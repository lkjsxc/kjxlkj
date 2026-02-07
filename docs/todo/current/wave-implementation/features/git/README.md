# Features: Git Integration (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement built-in git integration as a native feature (no plugins).

## Defining documents (direct, normative)

- Git features index:
  - [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

## Coverage traversal

- Git subtree:
  - [/docs/todo/doc-coverage/spec/features/git/README.md](/docs/todo/doc-coverage/spec/features/git/README.md)

## Checklist

- [ ] Placeholder scaffolding: define git service APIs and UI hooks.
- [ ] Minimal slice: implement one visible git surface end-to-end with tests.
  - GitService with find_repo_root, current_branch, file_status
  - FileStatus enum (Untracked, Modified, Added, Deleted, Renamed, Unmodified)
  - RepoStatus with branch, ahead/behind, staged/unstaged/untracked
  - FileChange, DiffHunk, BlameLine, GitSign types
- [ ] Full conformance: implement all git feature documents.
  - [ ] Branch indicator in statusline (BranchInfo.display())
  - [ ] File change indicators in file explorer (FileIndicator)
    - FileIndicator enum (Modified/Added/Deleted/Renamed/Untracked/Conflicted/Ignored) with symbol()
  - [ ] Hunks and gutter signs (Hunk, HunkType, GutterSign, BufferHunks)
  - [ ] Blame (BlameInfo, BufferBlame with toggle visibility)
  - [ ] Merge conflict detection/navigation (Conflict, ConflictMarker, ConflictChoice, BufferConflicts)
    - BufferConflicts.detect() scans for <<<<<<< / ======= / >>>>>>> markers
  - [ ] Diff viewer (DiffAlgorithm, DiffOptions, DiffLayout, DiffView)
    - DiffAlgorithm (Myers/Patience/Histogram), DiffLayout (Unified/SideBySide/Inline)
    - DiffView with count_added()/count_removed()
  - [ ] git_full.rs: DiffHunk/DiffLine/LogEntry/BlameEntry/BranchInfo, parse_diff, parse_log, GitSign, compute_signs
- [ ] Update conformance and limitations docs when user-visible.

