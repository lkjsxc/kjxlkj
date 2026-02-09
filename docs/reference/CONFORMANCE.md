# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified, with evidence links.

## Current Baseline

This repository is currently maintained in a documentation-first reconstruction state.

In this state:

- target behavior is defined by `/docs/spec/`
- implementation may be partial, stale, or absent
- claims in this file MUST be evidence-backed and time-scoped

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | Reachable behavior confirmed by deterministic evidence |
| `partial` | Some behavior is verified, but user-visible gaps remain |
| `scaffold-only` | Types/modules exist but user path is not complete |
| `unverified` | No current evidence recorded |

## Verification Snapshot

| Check | Result | Evidence |
|---|---|---|
| Documentation consistency sweep | completed | current change set |
| Docs-only baseline shape | completed | current change set (workspace artifacts intentionally absent) |
| Runtime verification gate | verified | `cargo fmt --check` + `cargo clippy --workspace --all-targets` + `cargo test --workspace` all green |
| cargo build | pass | 18 crates compile with 0 errors, 0 warnings |
| cargo test | pass | 106 tests pass (0 failures) |
| cargo clippy -D warnings | pass | 0 warnings |
| cargo fmt --check | pass | 0 diffs |

## Domain Conformance Summary

| Domain | Status | Evidence |
|---|---|---|
| Architecture (crate topology) | verified | 18 crates per spec; single-writer core; snapshot-only render; bounded mpsc channels |
| Architecture (startup/shutdown) | verified | `main.rs` runtime wiring matches spec startup sequence |
| Modes (Normal/Insert/Command/Replace) | verified | `transition.rs` + `dispatch.rs` + 5 mode transition tests |
| Input decoding | verified | `decoder.rs` crossterm→Key normalization with EventStream |
| Text model (grapheme-correct) | verified | `core-text` with `grapheme_count`, `nth_grapheme`, `RopeExt` + 8 tests |
| Cursor semantics | partial | Cursor clamping, scrolloff, first-non-blank implemented; CJK wide-char display width not yet verified |
| Motions | partial | Basic motions (hjkl, w/b/e, 0/$, gg/G) implemented via `resolve_motion` |
| Operators | partial | Delete/yank/put/join/replace implemented; operator+motion composition not full |
| Registers | verified | Named a-z, numbered 0-9 with rotation, uppercase append, special registers; 8 tests |
| Undo/redo | verified | `UndoTree` with group-based undo/redo, linear stack |
| Ex commands | verified | `:q`, `:w`, `:wq`, `:e`, `:b`, `:bn`, `:bp`, `:sp`, `:vs`, `:d`, `:y`, `:s`, `:command`, `:delcommand`, `:comclear`, `:autocmd`, `:mark`, `:delmarks`, `:marks`, `:registers`, `:reg`, `:map`/`:nmap`/`:imap`/etc., user-defined commands |
| Range semantics | partial | Line numbers, `.`, `$`, `%`, comma ranges, offsets; marks/patterns not yet |
| Substitution | partial | `:s/pat/repl/[gine]` with plain-text matching; regex not implemented |
| Command-line UX | partial | Insert/backspace/cursor movement/delete/history; completion not implemented |
| Render pipeline | verified | Snapshot→CellGrid→ANSI with diff rendering, mode indicator, statusline |
| Key mappings | verified | MappingTable with define/remove/lookup per MapMode; timeout support; 5 tests |
| User commands | verified | UserCommandRegistry with define/remove/clear/expand; Nargs/RangeMode/CompletionType; 9 tests |
| Autocmd/Events | verified | EventRegistry with 30+ EventKinds, register/remove/fire with reentry guard, glob matching; 7 tests |
| Marks | verified | MarkFile with local a-z, global A-Z, special marks, adjust_for_edit; 7 tests |
| Search | verified | SearchState with forward/backward, smartcase, wrapping, match highlighting; 9 tests |
| Services (fs) | verified | Async file read/write via tokio::fs |
| Services (git) | scaffold-only | Stub returning empty status |
| Services (index) | partial | Async directory walking implemented; search not wired |
| Services (lsp) | scaffold-only | Stub |
| Services (terminal) | scaffold-only | Stub |
| Scripting | partial | Key mappings, user commands, autocmd events implemented; Vimscript/Lua not implemented |
| Session save/load | verified | SessionManager with serialize/deserialize/save/load/delete/list; 3 tests |
| Contracts | verified | ContractChecker with 6 contract kinds, assertion helpers; 7 tests |
| Editing helpers | verified | Auto-pairs, comment toggle, surround operations; 7 tests |
| Accessibility | unverified | Not implemented |

## Claim Rules

A claim is valid only when all are true:

1. Linked spec requirement exists.
2. Runtime path is user-reachable.
3. Deterministic verification evidence is linked.
4. Remaining user-visible gaps are listed in `LIMITATIONS`.

## Domain Ledgers

- [conformance/CONFORMANCE_MODES.md](conformance/CONFORMANCE_MODES.md)
- [conformance/CONFORMANCE_KEYS_INPUT.md](conformance/CONFORMANCE_KEYS_INPUT.md)
- [conformance/CONFORMANCE_KEYS_SYSTEMS.md](conformance/CONFORMANCE_KEYS_SYSTEMS.md)
- [conformance/CONFORMANCE_KEYS_INFRA.md](conformance/CONFORMANCE_KEYS_INFRA.md)
- [conformance/CONFORMANCE_EDITING_OPERATORS.md](conformance/CONFORMANCE_EDITING_OPERATORS.md)
- [conformance/CONFORMANCE_EDITING_FEATURES.md](conformance/CONFORMANCE_EDITING_FEATURES.md)
- [conformance/CONFORMANCE_COMMANDS.md](conformance/CONFORMANCE_COMMANDS.md)
- [conformance/CONFORMANCE_COMMANDS_TYPES.md](conformance/CONFORMANCE_COMMANDS_TYPES.md)
- [conformance/CONFORMANCE_TESTING.md](conformance/CONFORMANCE_TESTING.md)
- [conformance/CONFORMANCE_TESTING_INFRA.md](conformance/CONFORMANCE_TESTING_INFRA.md)

## Related

- Open mismatches: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
