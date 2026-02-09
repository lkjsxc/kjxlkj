# Audit: Spec-Code-Test Sync Matrix (2026-02-09)

Back: [/docs/log/reconstruction/audits/README.md](/docs/log/reconstruction/audits/README.md)

## Scope

Documentation-only reconciliation across spec, reference, TODO, and current code/test reality.

## Mismatch Matrix

| Requirement ID | Canonical Doc | Requirement Statement | Code Path(s) | Test Path(s) | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|---|---|
| R-TERM-01 | `/docs/spec/features/terminal/terminal.md` | `:terminal` creates real PTY-backed terminal window | `src/crates/kjxlkj-core-state/src/action_dispatch2.rs`, `src/crates/kjxlkj-service-terminal/src/pty.rs` | `src/crates/kjxlkj-core-state/tests/pty_e2e_tests.rs` | `partial` | `M2 missing feature` | `spec-update` + TODO carry-forward | Dispatch currently treats `SpawnTerminal` as no-op |
| R-WIN-01 | `/docs/spec/features/window/splits-windows.md` | Directional window focus is spatially correct | `src/crates/kjxlkj-core-state/src/editor_window_ops.rs` | `src/crates/kjxlkj-core-state/tests/headless_e2e_tests.rs` | `partial` | `M1 correctness` | `spec-update` + TODO carry-forward | Focus uses cyclic map order, not layout geometry |
| R-EXP-01 | `/docs/spec/features/navigation/file_explorer.md` | Explorer toggle/open/split is wired end-to-end | `src/crates/kjxlkj-core-state/src/file_explorer.rs` | Explorer unit tests only | `partial` | `M2 missing feature` | `spec-update` + TODO carry-forward | Explorer model exists, runtime wiring incomplete |
| R-KEY-01 | `/docs/spec/ux/keybindings/mode-entry.md` | `Shift+a` behaves as `A` append-at-EOL | `src/crates/kjxlkj-input/src/decode.rs`, `src/crates/kjxlkj-core-mode/src/normal_commands.rs` | Regression coverage incomplete | `partial` | `M1 correctness` | `spec-update` + TODO carry-forward | Shift modifier and uppercase matching can diverge |
| R-IME-01 | `/docs/spec/modes/insert/input/insert-japanese-ime.md` | IME composition intercept prevents leader leakage | `src/crates/kjxlkj-core-state/src/ime.rs`, input dispatch path | `reg06_unicode`, `pe03_leader_vs_ime` | `partial` | `M4 verification gap` | `spec-update` + test blueprint | Model tests exist, full terminal path is incomplete |
| R-FS-01 | `/docs/spec/commands/file/write-commands.md` | `:w` persists file bytes | `src/crates/kjxlkj-core-state/src/editor_file_ops.rs` | No file-backed assertion for `:w` | `partial` | `M2 missing feature` | `spec-update` + TODO carry-forward | `do_write` clears modified flag only |
| R-TEST-01 | `/docs/spec/technical/testing.md` | Baseline test mapping references real files | docs only | N/A | `contradiction resolved` | `M5 stale docs` | `spec-update` | Stale paths replaced with current test files |
| R-TODO-01 | `/docs/todo/doc-coverage/README.md` | TODO must link every documentation file directly | docs only | N/A | `resolved` | `M5 stale docs` | `spec-update` | Coverage regenerated from filesystem inventory |

## Closed Mismatches In This Wave

- Closed stale testing reference paths (`M5`) by rewriting testing conformance docs.
- Closed TODO coverage drift (`M5`) by regenerating direct-link checklist parts.
- Closed authority ambiguity (`M5`) by rewriting reference authority model.

## Deferred Mismatches

| ID | Reason Deferred | Next Action |
|---|---|---|
| R-TERM-01 | Requires source implementation, outside docs-only wave | Keep limitation open and prioritize in next reconstruction wave |
| R-WIN-01 | Requires layout graph refactor in runtime | Keep limitation open and require spatial E2E tests before closure |
| R-EXP-01 | Requires dispatch/render integration work | Keep limitation open and add explorer E2E scenarios |
| R-KEY-01 | Requires input normalization code change | Keep limitation open and add key normalization regression test |
| R-IME-01 | Requires IME/runtime path integration | Keep limitation open and add PTY composition tests |
| R-FS-01 | Requires real filesystem wiring | Keep limitation open and add file-backed integration tests |

## Verification Commands

- `cargo test --workspace`
- `cargo test --workspace -- --list`
- `find docs -name '*.md' | wc -l`
- `rg -n '\.\./' docs`
