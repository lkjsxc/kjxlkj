# Syntax Engine (Tree-sitter class)

## User intent

High-quality syntax highlighting and structured editing primitives.

## Capabilities

| Capability | Requirement |
|---|---|
| Highlighting | Token-level styling based on syntax tree. |
| Incremental parse | Updates on edits without reparsing whole file. |
| Text objects | Tree-based selections (function, class, block). |
| Folding | Structural folds based on syntax nodes. |

## Async model

Parsing work MUST be isolated from the core:

- The core emits edit deltas.
- A syntax service consumes deltas and computes parse updates.
- The core receives summarized results suitable for snapshots.

If a dedicated syntax service is not present, syntax work MUST run as a bounded Tokio task that yields.

## Interfaces (conceptual)

| Input to syntax | Output from syntax |
|---|---|
| Document bytes + edit deltas | Highlight spans |
| Language id | Fold regions |
| Cursor position | Node-at-cursor metadata |

## Acceptance criteria

- Large files MUST remain editable with stable frame times.
- Highlighting MUST eventually converge after bursts of edits.
- Syntax failures MUST not crash rendering; show degraded styling.
