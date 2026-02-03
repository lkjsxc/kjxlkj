# Command history
Command history is core-owned state exposed via the command line and list UI.

## Requirements
- History updates are transactional.
- History browsing never blocks on disk.
- Optional persistence is versioned.

## Surfaces

- `↑` / `↓` navigate history
- `q:` open command history list
- `q/` open search history list

## Related

- Unified list model: [docs/spec/features/lsp/diagnostics.md](/docs/spec/features/lsp/diagnostics.md)
