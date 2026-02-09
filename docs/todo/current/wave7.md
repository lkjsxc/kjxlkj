# Wave 7 — Text Objects, Mark Motions, Register Rotation

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Text objects (iw/aw/ip/ap), marks as op-pending motions,
numbered register rotation on delete, macro→register unification,
\/ and \? last-search shorthand in ranges.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-TO-01 | iw inner word text object | `/docs/spec/editing/text-objects/README.md` | `[x]` |
| REQ-TO-02 | aw a word text object | `/docs/spec/editing/text-objects/README.md` | `[x]` |
| REQ-TO-03 | ip inner paragraph text object | `/docs/spec/editing/text-objects/README.md` | `[x]` |
| REQ-TO-04 | ap a paragraph text object | `/docs/spec/editing/text-objects/README.md` | `[x]` |
| REQ-MARKMOT-01 | '{char}/`{char} as motions in op-pending | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-REGROTA-01 | Numbered register rotation on delete (1→2→...→9) | `/docs/spec/editing/registers/README.md` | `[x]` |
| REQ-MACROREG-01 | Macro store shared with register file | `/docs/spec/editing/macros/README.md` | `[x]` |
| REQ-MACROAPP-01 | q{A-Z} appends to macro register | `/docs/spec/editing/macros/README.md` | `[x]` |
| REQ-LASTSEARCH-01 | \/ and \? last-search shorthand in ranges | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-FMT-01 | gq{motion} format operator (line wrap) | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 199 lines
- LIMITATIONS updated
