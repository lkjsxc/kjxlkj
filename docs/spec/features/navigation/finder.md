# Fuzzy Finder

## User intent

Fast navigation across a large workspace:

- Open files by approximate name
- Switch buffers
- Live grep across project
- Jump to symbols (from LSP + syntax index)

## UX surface

| Interaction | Requirement |
|---|---|
| Picker UI | A modal, keyboard-driven list with preview. |
| Incremental query | Each keystroke updates results with debounce. |
| Multi-source | Sources include files, buffers, recent, symbols, grep hits. |
| Actions | Open, split, vsplit, tab, reveal location. |

## Async model

| Work | Location | Notes |
|---|---|---|
| Workspace indexing | `kjxlkj-svc-index` | Maintains file list + token index. |
| Grep execution | `kjxlkj-svc-index` or dedicated grep service | MUST stream results. |
| Symbol provider | `kjxlkj-svc-lsp` + syntax | LSP preferred; fallback to syntax. |

Core requirements:

- The core MUST never synchronously scan the filesystem.
- Result updates MUST be cancellable as the query changes.

## Data model (conceptual)

| Entity | Fields |
|---|---|
| `FinderSource` | `kind`, `scope`, `ranking_model` |
| `FinderItem` | `label`, `location`, `preview_hint`, `score` |
| `FinderQuery` | `text`, `filters`, `limit` |

## Acceptance criteria

- Given a workspace with 100k files, typing a 10-char query MUST keep input responsive.
- When the query changes, stale results MUST not flash after newer results.
- Grep results MUST appear incrementally; the UI MUST show progress.
- Any picker MUST be closeable without waiting for background tasks.
