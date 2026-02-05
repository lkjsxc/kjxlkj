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

- [x] Placeholder scaffolding: define git service APIs and UI hooks.
- [x] Minimal slice: implement one visible git surface end-to-end with tests.
  - GitService with find_repo_root, current_branch, file_status
  - GitStatus enum (Untracked, Modified, Staged, Unchanged, Ignored)
  - BranchInfo for statusline display
  - FileIndicator for explorer integration
  - RepoStats for summary view
- [x] Full conformance: implement all git feature documents.
  - [x] Branch indicator in statusline (BranchInfo.display())
  - [x] File change indicators in file explorer (FileIndicator)
  - [x] Hunks and gutter signs (Hunk, HunkType, GutterSign, BufferHunks)
  - [x] Blame (BlameInfo, BufferBlame with toggle visibility)
  - [x] Merge conflict detection/navigation (Conflict, ConflictMarker, ConflictChoice, BufferConflicts)
  - [x] Diff viewer (DiffAlgorithm, DiffOptions, DiffLayout, DiffView)
- [x] Update conformance and limitations docs when user-visible.

