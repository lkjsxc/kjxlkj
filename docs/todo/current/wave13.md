# Wave 13 — Very-Nomagic, Session Buffers, Block Insert, Arg Text Obj, Fuzzy Completion, Expr Register, Cross-Buffer Jumps, Incsearch

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Wire very-nomagic (\V) regex mode, per-window buffer assignments in session,
block insert (I/A in visual block mode), argument text objects (ia/aa), fuzzy
matching for command-line completion, expression register evaluation (Ctrl-R =),
cross-buffer jump list navigation, and incremental search preview.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-VNOMAGIC-01 | \V very-nomagic regex: all chars literal except backslash | `/docs/spec/editing/regex/README.md` | `[x]` |
| REQ-SESSBUF-01 | :mksession saves per-window buffer assignments | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-BLOCKINSERT-01 | I/A in visual block mode inserts on each line | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-ARGTOBJ-01 | ia/aa argument text objects (inside/around comma-delimited) | `/docs/spec/editing/text-objects/argument.md` | `[x]` |
| REQ-FUZZCOMPL-01 | Fuzzy matching for command-line completion candidates | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-EXPRREG-01 | Expression register (=): evaluate expression, insert result | `/docs/spec/editing/registers/expression-register.md` | `[x]` |
| REQ-XBUFJUMP-01 | Jump list entries track buffer ID; Ctrl-O/I switch buffers | `/docs/spec/editing/marks/jumplist.md` | `[x]` |
| REQ-INCSEARCH-01 | Incremental search: highlight first match while typing | `/docs/spec/editing/search/incsearch.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 200 lines
- LIMITATIONS updated
