# Principles

Back: [/docs/spec/overview/README.md](/docs/spec/overview/README.md)

Core design principles governing all specifications and implementations. These are normative constraints, not suggestions.

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

## Deterministic core (detail)

The single-writer task pattern means:

| Rule | Requirement |
|---|---|
| Ownership | One Tokio task exclusively owns `EditorState`. |
| Access | No other task, service, or renderer may hold a mutable reference. |
| Communication | All external state changes arrive as messages via bounded channels. |
| Ordering | Messages are processed in receive order within a single `select!` loop. |
| Snapshots | After processing a batch of messages, the core produces an immutable `EditorSnapshot` and publishes it on a `watch` channel. |

## Async-first (detail)

Services run as independent Tokio tasks. The core task never performs blocking I/O directly.

| Blocking operation | Delegated to |
|---|---|
| File read/write | FS service task (uses `tokio::fs`) |
| LSP communication | LSP service task (JSON-RPC over stdio/TCP) |
| Git operations | Git service task (subprocess spawn) |
| Terminal PTY I/O | Terminal service task (async PTY read/write) |
| File indexing | Index service task (background scan) |

If a service is slow or unresponsive, the core task continues processing input and editing operations. Overdue service responses are surfaced in the statusline.

## Grapheme-based text (detail)

All text operations index by grapheme cluster, not byte offset or code point.

| Operation | Indexed by |
|---|---|
| Cursor position | `(line, grapheme_offset)` |
| Motion `l`/`h` | +1/-1 grapheme |
| Word boundary detection | Grapheme category transitions |
| Display width calculation | Per-grapheme width via `unicode-width` |
| Text object boundaries | Grapheme-level start/end positions |

This ensures correct handling of:

- CJK fullwidth characters (display width 2, grapheme count 1)
- Emoji ZWJ sequences (e.g., family emoji: 1 grapheme, width 2)
- Combining marks (e.g., `e` + combining acute = 1 grapheme, width 1)
- Tab characters (1 grapheme, variable display width)

## No-plugin rationale (detail)

External plugin systems introduce:

| Concern | Risk |
|---|---|
| Version skew | Plugins and editor evolve independently, causing breakage. |
| Sandboxing complexity | Untrusted code execution in the editor process. |
| UX fragmentation | Each plugin invents its own UI patterns. |
| Performance variance | Poorly written plugins can freeze the editor. |
| Distribution burden | Users must find, install, and update plugins. |

Instead, kjxlkj provides all integrations as built-in features configured through a unified TOML file. Features can be individually enabled or disabled.

## Snapshot rendering (detail)

| Rule | Requirement |
|---|---|
| Immutability | The snapshot MUST be a read-only deep copy (or shared-ownership clone) of relevant state. |
| Completeness | The snapshot MUST contain all data needed to render a complete frame without querying core. |
| Coalescing | If multiple snapshots are produced faster than the renderer can consume, only the latest is rendered. |
| Monotonicity | Each snapshot has a strictly increasing sequence number. Stale snapshots MUST be discarded. |

## Related

- Architecture: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- Glossary: [/docs/spec/overview/glossary.md](/docs/spec/overview/glossary.md)
- Runtime model: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Render pipeline: [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
