# Git UX

## User intent

Operate on version-controlled code without leaving the editor:

- See hunks inline
- Stage/reset hunks
- Blame lines
- Browse diffs and history

## Async model

All git operations MUST run off-core in a git service.

| Operation | Notes |
|---|---|
| Status | Cached; refresh on FS events and on demand. |
| Hunks | Incremental per buffer; computed from working tree vs index/HEAD. |
| Blame | On-demand with caching; cancellable. |
| Diff view | Stream large diffs; allow abort. |

## UX surface

| Surface | Requirement |
|---|---|
| Gutter signs | Added/changed/removed indicators. |
| Hunk actions | Stage/reset/preview. |
| Blame | Inline virtual text or dedicated view. |
| Commit UI | Minimal built-in commit message editor and staging view. |

## Acceptance criteria

- Opening a large repo MUST not stall startup; git status initializes in background.
- Hunk signs MUST stay in sync with edits (eventual consistency allowed).
- If git is unavailable, UI MUST show an actionable diagnostic.
