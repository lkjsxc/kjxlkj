# Insert mode
Insert mode handles direct text entry and completion UI integration.

## Requirements
- Inserted text becomes core transactions (coalesced into one undo unit per session).
- Completion suggestions come from services (LSP/snippets/etc.) and are versioned.
- Leaving insert with `Esc` always returns to Normal.

## Responsibilities

- Text entry
- Inline delete helpers (backspace/word/bol)
- Completion accept/cancel

## Related

- Undo granularity: [docs/spec/editing/text-manipulation/undo.md](docs/spec/editing/text-manipulation/undo.md)
- LSP completion: [docs/spec/features/lsp/lsp.md](docs/spec/features/lsp/lsp.md)
