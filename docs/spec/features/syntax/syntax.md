# Syntax Engine (Tree-sitter class)

Back: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

## User intent

High-quality syntax highlighting and structured editing primitives.

## Capabilities (normative)

| Capability | Requirement |
|---|---|
| Highlighting | Token-level styling based on syntax tree |
| Incremental parse | Updates on edits without reparsing whole file |
| Text objects | Tree-based selections (function, class, block, argument) |
| Folding | Structural folds based on syntax nodes |
| Indentation | Automatic indent level computed from syntax context |
| Injection | Embedded languages (e.g., JS inside HTML) parsed separately |

## Async model (normative)

Parsing work MUST be isolated from the core:

- The core emits edit deltas (byte range + new text) on every buffer change.
- A syntax service consumes deltas and computes incremental parse updates.
- The core receives summarized results suitable for snapshots.

| Aspect | Requirement |
|---|---|
| Thread model | Parsing runs in a dedicated Tokio blocking task |
| Cancellation | Parse MUST be cancellable when a newer delta arrives |
| Coalescing | Multiple rapid edits MAY be batched into one reparse |
| Timeout | Individual parse calls MUST have a time budget (default: 50ms) |
| Fallback | If timeout is exceeded, return partial highlights for the visible range |

## Interfaces (normative)

| Input to syntax | Output from syntax |
|---|---|
| Document bytes + edit deltas | `Vec<HighlightSpan>` where span = `{ byte_range, highlight_group }` |
| Language id | `Vec<FoldRegion>` where region = `{ start_line, end_line, kind }` |
| Cursor position | `NodeInfo { kind, named, parent_kind, depth }` |
| Query (text objects) | `Vec<Range>` matching a capture name (e.g., `@function.outer`) |

## Language detection (normative)

| Method | Priority |
|---|---|
| File extension | Primary (e.g., `.rs` → Rust) |
| Shebang line | Secondary (e.g., `#!/usr/bin/env python3`) |
| Modeline | Tertiary (e.g., `vim: ft=python`) |
| Content sniffing | Last resort (heuristic) |
| Manual override | `:set filetype=X` always wins |

## Highlight group mapping (normative)

Tree-sitter capture names MUST map to editor highlight groups:

| Capture | Highlight group |
|---|---|
| `@keyword` | `Keyword` |
| `@function` | `Function` |
| `@type` | `Type` |
| `@string` | `String` |
| `@comment` | `Comment` |
| `@variable` | `Identifier` |
| `@constant` | `Constant` |
| `@operator` | `Operator` |
| `@punctuation` | `Delimiter` |
| `@property` | `Field` |

## Acceptance criteria

- Large files (100K+ lines) MUST remain editable with stable frame times.
- Highlighting MUST eventually converge after bursts of edits.
- Syntax failures MUST not crash rendering; show plain text fallback.
- Language switch (`:set filetype=X`) MUST trigger full reparse.
- Injected languages MUST highlight correctly within their ranges.
