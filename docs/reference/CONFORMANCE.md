# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger captures the verified implementation surface for the current repository state.

## Verification Snapshot

| Check | Result | Evidence |
|---|---|---|
| `cargo test --workspace` | pass | Local run on 2026-02-09 |
| `cargo test --workspace -- --list` | 523 tests listed | Local run on 2026-02-09 |

## Status Vocabulary

| Status | Meaning |
|---|---|
| `implemented` | Reachable and covered by deterministic tests |
| `partial` | Implemented in part, with known wiring gaps |
| `scaffold-only` | Data types or modules exist, but user path is not fully wired |
| `planned` | Target only, no meaningful implementation |

## Current Surface (High-Level)

| Area | Status | Evidence |
|---|---|---|
| Core modal editing loop | `implemented` | `src/crates/kjxlkj-core-mode/src/`, `src/crates/kjxlkj-core-state/tests/dispatch_tests.rs` |
| Basic motions/operators/registers | `implemented` | `src/crates/kjxlkj-core-edit/src/`, `src/crates/kjxlkj-core-state/tests/regression_tests.rs` |
| CJK cursor and wrap primitives | `implemented` | `src/crates/kjxlkj-core-state/src/cjk_support.rs`, `src/crates/kjxlkj-core-state/src/line_wrap.rs` |
| Window split/cycle/close primitives | `implemented` | `src/crates/kjxlkj-core-state/src/editor_file_ops.rs`, `src/crates/kjxlkj-core-state/tests/headless_e2e_tests.rs` |
| Advanced window graph semantics | `partial` | `src/crates/kjxlkj-core-state/src/editor_window_ops.rs` uses cyclic ordering, not spatial graph |
| File explorer integration | `partial` | `src/crates/kjxlkj-core-state/src/file_explorer.rs` exists; dispatch wiring is incomplete |
| Terminal as real user-facing window | `partial` | `Action::SpawnTerminal` currently no-op in `src/crates/kjxlkj-core-state/src/action_dispatch2.rs` |
| PTY runtime implementation | `scaffold-only` | `src/crates/kjxlkj-service-terminal/src/pty.rs` exists but not integrated into main action path |
| Session command wiring | `partial` | session model exists; `SessionSave/SessionLoad` are no-op in dispatch |
| File write/read real persistence | `partial` | `do_write` clears modified flag; does not persist file bytes |
| Leader key UX parity with target docs | `partial` | default leader is `\\` in `src/crates/kjxlkj-core-state/src/keybinding_dsl.rs` |
| Japanese IME full input-pipeline behavior | `partial` | IME state exists in `src/crates/kjxlkj-core-state/src/ime.rs`; end-to-end terminal composition flow incomplete |

## Claim Rules

A claim in this ledger is valid only when all are true:

1. Target behavior is defined in `/docs/spec/...`.
2. User-facing path is reachable from runtime entrypoint.
3. Deterministic tests cover success and boundary behavior.
4. Any remaining user-visible gap is recorded in `LIMITATIONS`.

## Sub-Ledgers

These files provide domain-focused detail and MUST be interpreted under this top-level ledger:

- [CONFORMANCE_MODES.md](CONFORMANCE_MODES.md)
- [CONFORMANCE_KEYS_INPUT.md](CONFORMANCE_KEYS_INPUT.md)
- [CONFORMANCE_KEYS_SYSTEMS.md](CONFORMANCE_KEYS_SYSTEMS.md)
- [CONFORMANCE_KEYS_INFRA.md](CONFORMANCE_KEYS_INFRA.md)
- [CONFORMANCE_EDITING_OPERATORS.md](CONFORMANCE_EDITING_OPERATORS.md)
- [CONFORMANCE_EDITING_FEATURES.md](CONFORMANCE_EDITING_FEATURES.md)
- [CONFORMANCE_COMMANDS.md](CONFORMANCE_COMMANDS.md)
- [CONFORMANCE_COMMANDS_TYPES.md](CONFORMANCE_COMMANDS_TYPES.md)
- [CONFORMANCE_TESTING.md](CONFORMANCE_TESTING.md)
- [CONFORMANCE_TESTING_INFRA.md](CONFORMANCE_TESTING_INFRA.md)

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Target spec: [/docs/spec/README.md](/docs/spec/README.md)
