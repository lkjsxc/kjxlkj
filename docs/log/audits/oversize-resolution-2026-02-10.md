# Audit: Oversize File Resolution (2026-02-10)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Goal

Resolve all source files exceeding the 200-line MUST limit from
`docs/spec/architecture/crates.md`.

## Pre-Split Violations

18 files exceeded 200 lines. After prior ex_commands.rs / editor.rs / search.rs
splits, the following remained:

| Lines | File |
|---:|---|
| 362 | editing_ops.rs |
| 325 | ex_parse.rs |
| 358 | user_commands.rs |
| 316 | events.rs |
| 290 | window_tree.rs |
| 290 | motion.rs |
| 279 | marks.rs |
| 262 | registers.rs |
| 261 | grid.rs |
| 253 | editing_helpers.rs |
| 251 | session.rs |
| 242 | buffer_list.rs |
| 230 | cursor_ops.rs |
| 221 | mappings.rs |
| 211 | main.rs |

All paths relative to `src/crates/*/src/`.

## Splits Performed

| Original | New Files | Strategy |
|---|---|---|
| editing_ops.rs (362) | editing_ops_insert.rs, editing_ops_modify.rs, editing_ops_yank.rs | Split by operation category |
| ex_parse.rs (325) | ex_parse.rs, ex_parse_substitute.rs, ex_parse_tests.rs | Extract SubstituteCmd + tests |
| user_commands.rs (358) | user_commands.rs, user_commands_parse.rs, user_commands_tests.rs | Extract parser + tests |
| events.rs (316) | events.rs, events_types.rs, events_tests.rs | Extract types + tests |
| window_tree.rs (290) | window_tree.rs, window_tree_layout.rs | Extract layout algorithms |
| motion.rs (290) | motion.rs, motion_helpers.rs | Extract helper functions |
| marks.rs (279) | marks.rs, marks_tests.rs | Extract tests |
| registers.rs (262) | registers.rs, registers_tests.rs | Extract tests |
| grid.rs (261) | grid.rs, grid_window.rs | Extract window rendering |
| editing_helpers.rs (253) | editing_helpers.rs, editing_helpers_surround.rs, editing_helpers_tests.rs | Extract surround ops + tests |
| session.rs (251) | session.rs, session_tests.rs | Extract tests |
| buffer_list.rs (242) | buffer.rs, buffer_list.rs | Extract Buffer struct |
| cursor_ops.rs (230) | cursor_ops.rs, cursor_ops_scroll.rs | Extract scroll/viewport ops |
| mappings.rs (221) | mappings.rs, mappings_tests.rs | Extract tests |
| main.rs (211) | main.rs, app.rs | Extract core event loop |

## Post-Split Verification

```
cargo build    → clean (0 errors)
cargo test     → 106 passed, 0 failed
cargo clippy   → 0 warnings
```

Maximum file size after all splits: **199 lines** (session.rs).

No source file exceeds the 200-line MUST limit.

## File Count Summary

| Crate | Source Files |
|---|---:|
| kjxlkj-core-state/src | 48 |
| kjxlkj-core-edit/src | 7 |
| kjxlkj-render/src | 5 |
| kjxlkj/src | 2 |
