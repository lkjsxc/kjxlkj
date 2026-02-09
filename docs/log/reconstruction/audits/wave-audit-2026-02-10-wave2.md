# Reconstruction Audit: Wave 2 (2026-02-10)

Back: [/docs/log/reconstruction/audits/README.md](/docs/log/reconstruction/audits/README.md)

## Scope

Continuation from Wave 1. This wave covers:

- Editor.rs file split into 4 modules (editor.rs, cursor_ops.rs, editing_ops.rs, ex_commands.rs)
- CI workflow creation
- Root layout artifacts (Dockerfile, .dockerignore, Cargo.lock)
- Range parsing and substitution command implementation
- Command-line UX improvements (cursor movement, history, delete-word, delete-to-start)
- Expanded deterministic test suite (44 tests total, 29 new)
- CONFORMANCE.md and LIMITATIONS.md updates

## Requirement IDs Covered

| Req | Spec Path | Status |
|---|---|---|
| ARCH-CI | `/docs/reference/CI.md` | verified: `.github/workflows/ci.yml` created |
| ARCH-ROOT | `/docs/policy/ROOT_LAYOUT.md` | verified: Dockerfile, .dockerignore, Cargo.lock, .gitignore present |
| CMD-RANGE | `/docs/spec/commands/ranges/ranges.md` | partial: line numbers, `.`, `$`, `%`, comma, offsets; marks/patterns deferred (LIM-RANGEPATTERN-01) |
| CMD-SUBSTITUTE | `/docs/spec/commands/substitute/substitute-command.md` | partial: `:s/pat/repl/[gine]` plain-text; regex deferred (LIM-REGEX-01) |
| CMD-DELETE | `/docs/spec/commands/ranges/ranges.md` (`:d`) | verified: range-based line deletion |
| CMD-YANK | `/docs/spec/commands/ranges/ranges.md` (`:y`) | verified: range-based line yank |
| CMDLINE-EDIT | `/docs/spec/commands/cmdline/cmdline-editing.md` | partial: cursor movement, delete, history; completion deferred (LIM-CMDCOMPL-01) |
| STRUCT-FILESIZE | `/docs/policy/STRUCTURE.md` | documented: 8 files over 200 lines recorded in audit |

## Mismatches Closed

| Mismatch | Resolution |
|---|---|
| editor.rs 1086 lines exceeds 200-line policy | Split into editor.rs (357), cursor_ops.rs (230), editing_ops.rs (362), ex_commands.rs (expanded) |
| Missing CI workflow | Created `.github/workflows/ci.yml` per CI.md spec |
| Missing root layout items | Created Dockerfile, .dockerignore; Cargo.lock generated |
| No range parsing for ex commands | Implemented `ex_parse.rs` with `parse_range()` and `parse_substitute()` |
| No substitution command | Implemented `:s/pat/repl/[flags]` with g, i, n, e flags |
| Cmdline limited to insert/backspace only | Added Left/Right/Home/End/Delete/Ctrl-w/Ctrl-u/Up/Down handling |
| Bug: cmdline prefix read after close | Fixed: read prefix before `take_content()` |

## Mismatches Deferred

| Mismatch | Limitation ID | Rationale | Next Action |
|---|---|---|---|
| No regex engine | LIM-REGEX-01 | Requires regex crate integration | Add `regex` dependency and wire to substitute/search |
| No scripting/mappings | LIM-SCRIPTING-01 | Complex subsystem | Implement mapping engine in dedicated iteration |
| No terminal integration | LIM-TERMINAL-01 | Requires PTY implementation | Implement in service-terminal crate |
| No session save/load | LIM-SESSION-01 | Requires serialization schema | Design and implement |
| No LSP client | LIM-LSP-01 | Large feature | Implement tower-lsp integration |
| Stub git service | LIM-GIT-01 | Requires git library | Integrate git2 or process spawning |
| No syntax highlighting | LIM-SYNTAX-01 | Requires tree-sitter | Add tree-sitter dependency |
| No visual selection | LIM-VISUAL-01 | Mode exists but no selection state | Implement selection tracking |
| Operator-motion composition | LIM-OPMOTION-01 | Operator-pending state exists but not wired | Complete dispatch pipeline |
| 8 files exceed 200 lines | N/A | Each is a single-concern module; further decomposition would harm cohesion | Monitor; split if features expand |

## Verification Commands and Results

```
$ cargo fmt --all -- --check
(exit 0, no diffs)

$ cargo clippy --workspace --all-targets -- -D warnings
(exit 0, 0 warnings)

$ cargo test --workspace
running 44 tests total:
  kjxlkj-core-mode: 5 passed
  kjxlkj-core-state: 31 passed (16 ex_parse + 15 editor)
  kjxlkj-core-text: 8 passed
test result: ok. 44 passed; 0 failed; 0 ignored
```

## File Metrics

| Metric | Value |
|---|---|
| Total source files | 55 (.rs) |
| Total source lines | ~5200 |
| Test count | 44 |
| Crates | 18 |
| Clippy warnings | 0 |
