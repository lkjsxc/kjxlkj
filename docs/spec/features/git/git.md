# Git UX

Back: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

## User intent

Operate on version-controlled code without leaving the editor:

- See hunks inline
- Stage/reset hunks
- Blame lines
- Browse diffs and history

## Async model (normative)

All git operations MUST run off-core in the git service. The git service spawns `git` as a subprocess.

| Operation | Subprocess command | Caching | Notes |
|---|---|---|---|
| Status | `git status --porcelain=v2` | Cached; refresh on FS events and on demand | Background refresh every 5 seconds when idle |
| Hunks | `git diff --no-color -U0` | Per buffer; recomputed on buffer change | Compared against HEAD or index |
| Blame | `git blame --porcelain` | On-demand with LRU cache; cancellable | Cache keyed by file path + HEAD commit |
| Diff view | `git diff --no-color` | Streamed for large diffs; allow abort | Presented in a diff-mode buffer |
| Log | `git log --format=...` | On-demand; cancellable | Used for history browser |

## UX surface (normative)

| Surface | Requirement |
|---|---|
| Gutter signs | Added (`+`), changed (`~`), removed (`-`) indicators in the sign column. Color: `DiffAdd`, `DiffChange`, `DiffDelete` highlight groups. |
| Hunk preview | `]c` / `[c` to navigate between hunks. Preview hunk content in a floating window. |
| Hunk stage | Command `:GitStageHunk` stages the hunk under cursor. |
| Hunk reset | Command `:GitResetHunk` resets the hunk under cursor to HEAD. |
| Blame | `:GitBlame` toggles inline blame as virtual text (author, date, commit message). |
| Status view | `:GitStatus` opens a buffer showing status with stage/unstage actions. |
| Commit | `:GitCommit` opens a commit message buffer; saving commits. |

## Commands (normative)

| Command | Description |
|---|---|
| `:GitStatus` | Open git status view |
| `:GitDiff` | Open diff of current file vs HEAD |
| `:GitBlame` | Toggle inline blame |
| `:GitStageHunk` | Stage hunk under cursor |
| `:GitResetHunk` | Reset hunk under cursor |
| `:GitCommit` | Open commit message editor |
| `:GitLog` | Browse commit history |

## Acceptance criteria

- Opening a large repo MUST NOT stall startup; git status initializes in background.
- Hunk signs MUST stay in sync with edits (eventual consistency allowed, converge within 500ms).
- If `git` binary is unavailable, UI MUST show an actionable diagnostic and all git features MUST be gracefully disabled.
- Blame requests MUST be cancellable; navigating away cancels pending blame.

## Related

- Gitsigns: [/docs/spec/features/git/gitsigns.md](/docs/spec/features/git/gitsigns.md)
- Diff mode: [/docs/spec/features/git/diff-mode.md](/docs/spec/features/git/diff-mode.md)
- Service architecture: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
