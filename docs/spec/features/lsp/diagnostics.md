# Diagnostics and Lists

Back: [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)

## User intent

A consistent way to navigate "things to fix":

- LSP diagnostics
- Build errors
- Grep hits
- TODO/FIXME annotations

## Model (normative)

All listable items share a common structure:

| Field | Type | Meaning |
|---|---|---|
| `id` | stable ID | Unique identifier for deduplication |
| `kind` | enum | `Diagnostic`, `Build`, `Grep`, `Todo`, `Quickfix` |
| `severity` | enum | `Error` (1), `Warning` (2), `Info` (3), `Hint` (4) |
| `location` | struct | `file_path` + `line` + `col` + optional `end_line` + `end_col` |
| `message` | string | Display text |
| `source` | string | LSP server name, tool name, or `indexer` |
| `code` | string or null | Diagnostic code (e.g., `E0308` for Rust) |

## Async sourcing (normative)

| Source | Push/pull | Update semantics |
|---|---|---|
| LSP diagnostics | Push (`textDocument/publishDiagnostics`) | Replace all diagnostics for the file |
| Index service | Pull (grep results streamed) | Append incrementally |
| Build integration | Push (parsed from build output) | Replace per build run |

## List UI (normative)

| Aspect | Requirement |
|---|---|
| Rendering | Items displayed in a quickfix-style panel at the bottom |
| Navigation | `]d` / `[d` to jump to next/previous diagnostic in buffer |
| Sorting | By severity (errors first), then by file path, then by line number |
| Filtering | Filter by severity level or source |
| Incremental updates | New items MUST be merged without losing scroll position |
| Stable selection | If the selected item is still present after an update, it remains selected |
| Deduplication | Items with the same `id` MUST be coalesced (latest wins) |

## Commands (normative)

| Command | Description |
|---|---|
| `:copen` | Open the quickfix/diagnostics list |
| `:cclose` | Close the quickfix list |
| `:cnext` | Jump to next item |
| `:cprev` | Jump to previous item |
| `:cfirst` | Jump to first item |
| `:clast` | Jump to last item |
| `:lopen` | Open location list (per-window) |
| `:lclose` | Close location list |

## Inline rendering (normative)

Diagnostics MUST also be shown inline in the buffer:

| Display | Description |
|---|---|
| Underline | Diagnostic range underlined with severity-colored style |
| Virtual text | Message shown as virtual text at line end |
| Sign column | Severity icon in gutter (`E`, `W`, `I`, `H`) |

## Acceptance criteria

- The list MUST stay usable while sources stream updates.
- Jumping to an item MUST NOT require waiting for background work.
- Items MUST be deduplicated and coalesced by stable IDs.
- Diagnostic count MUST be shown in the statusline.
