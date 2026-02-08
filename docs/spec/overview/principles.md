# Principles

Back: [/docs/spec/overview/README.md](/docs/spec/overview/README.md)

Core design principles governing all specifications and implementations.

## Principles (normative)

| Principle | Meaning | Consequence |
|---|---|---|
| Deterministic core | All edits are serialized through a single-writer task. | No data races, no locking in the editing path. |
| Async-first | Tokio services handle IO/heavy compute; core stays responsive. | File I/O, LSP, git, indexing never block the editor. |
| No plugins | Integrations are built-in and share a unified UX. | Features like LSP, git, file explorer share list UIs, keymaps, and diagnostics. |
| Snapshot rendering | Rendering consumes immutable snapshots only. | Renderer can never corrupt editor state; UI is always consistent. |
| Recoverable failures | Services can fail and restart without losing editing capability. | Service supervision, health reporting, automatic restart. |
| Grapheme-based text | All cursor and editing operations work on grapheme clusters. | Correct behavior for CJK, emoji, combining marks. |
| Keyboard-only | All workflows MUST be keyboard-driven. | No mouse support, no GUI dependencies. |
| Single binary | The entire application ships as one executable. | No runtime dependencies, no dynamic loading, no plugin downloads. |

## Design consequences

- Features that "feel like plugins" (LSP, git, finder) are **first-class** and share list UIs, keymaps, and diagnostics.
- The system is designed for overload: backpressure is explicit, and overload is visible in the statusline.
- Every feature MUST be reachable from a key binding or ex command; no hidden functionality.
- Configuration is read once at startup; changes require restart or explicit `:source` command.
- Error messages MUST be actionable; never silently swallow errors.

## Related

- Architecture: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- Glossary: [/docs/spec/overview/glossary.md](/docs/spec/overview/glossary.md)
