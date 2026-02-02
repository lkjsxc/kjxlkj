# Diagnostics + Lists (Trouble/quickfix class)

## User intent

A consistent way to navigate “things to fix”:

- LSP diagnostics
- Build errors
- Grep hits
- TODO/FIXME annotations

## Model

All listable items share a common structure.

| Field | Meaning |
|---|---|
| `kind` | Diagnostic, build, grep, todo, etc. |
| `severity` | Error/warn/info/hint. |
| `location` | File + range. |
| `message` | Display text. |
| `source` | LSP server, tool name, indexer. |

## Async sourcing

- LSP pushes diagnostics.
- Index service streams grep hits.
- Build integration produces error items.

List UI MUST support incremental updates and stable selection.

## Acceptance criteria

- The list MUST stay usable while sources stream updates.
- Jumping to an item MUST not require waiting for background work.
- Items MUST be deduplicated and coalesced by stable IDs.
