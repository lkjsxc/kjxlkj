# Wave 9 — Sentence/Tag Text Objects, Visual Range, Option/Buffer Completion, Change Marks, Search Highlight, Macro-Register Unification

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Extend text objects (is/as, it/at), wire visual range shorthand ('<,'>),
add option/buffer completion, wire change marks, hlsearch support
in render snapshot, and unify macro store with register file.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-SENTENCE-01 | is/as sentence text objects | `/docs/spec/editing/text-objects/text_objects.md` | `[x]` |
| REQ-TAGTOBJ-01 | it/at tag text objects (HTML/XML) | `/docs/spec/editing/text-objects/tag-text-objects.md` | `[x]` |
| REQ-VISRANGE-01 | '<,'> visual range shorthand in ex commands | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-OPTCOMPL-01 | Option-name completion for :set in cmdline | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-BUFCOMPL-01 | Buffer-name completion for :b in cmdline | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-CHGMARKS-01 | [ ] marks wired into change/delete operations | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-HLSEARCH-01 | hlsearch matches included in render snapshot | `/docs/spec/editing/search/README.md` | `[x]` |
| REQ-MACREG-01 | Macro store reads unified with register file | `/docs/spec/editing/macros/README.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 200 lines
- LIMITATIONS updated
