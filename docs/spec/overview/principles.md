# Principles

## Principles

| Principle | Meaning |
|---|---|
| Deterministic core | All edits are serialized through a single-writer task. |
| Async-first | Tokio services handle IO/heavy compute; core stays responsive. |
| No plugins | Integrations are built-in and share a unified UX. |
| Snapshot rendering | Rendering consumes immutable snapshots only. |
| Recoverable failures | Services can fail and restart without losing editing capability. |

## Consequences

- Features that “feel like plugins” (LSP, git, finder) are **first-class** and share list UIs, keymaps, and diagnostics.
- The system is designed for overload: backpressure is explicit, and overload is visible.
