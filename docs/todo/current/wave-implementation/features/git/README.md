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
  - GitStatus enum (Untracked, Modified, Staged, Unchanged, Ignored)
  - BranchInfo for statusline display
  - FileIndicator for explorer integration
  - RepoStats for summary view
- [ ] Full conformance: implement all git feature documents.
  - [ ] Branch indicator in statusline (BranchInfo.display())
  - [ ] File change indicators in file explorer (FileIndicator)
  - [ ] Hunks and gutter signs (Hunk, HunkType, GutterSign, BufferHunks)
  - [ ] Blame (BlameInfo, BufferBlame with toggle visibility)
  - [ ] Merge conflict detection/navigation (Conflict, ConflictMarker, ConflictChoice, BufferConflicts)
  - [ ] Diff viewer (DiffAlgorithm, DiffOptions, DiffLayout, DiffView)
- [ ] Update conformance and limitations docs when user-visible.

