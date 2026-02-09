# Audit: Source Topology and Oversize Files (2026-02-09)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Goal

Capture current source-structure hotspots so next reconstruction can enforce 200-line file limits and
around-12 fan-out targets.

## Directory Fan-Out Snapshot

| Directory | Direct Children |
|---|---:|
| `src/crates` | 18 |
| `src/crates/kjxlkj-core-state/src` | 91 |
| `src/crates/kjxlkj-core-edit/src` | 21 |
| `src/crates/kjxlkj-core-mode/src` | 16 |
| `src/crates/kjxlkj-core-types/src` | 16 |

## Files Exceeding 200 Lines (Initial Snapshot)

| Lines | File |
|---:|---|
| 312 | `src/crates/kjxlkj-core-types/src/action.rs` |
| 296 | `src/crates/kjxlkj-core-state/src/editor.rs` |
| 273 | `src/crates/kjxlkj-core-state/tests/dispatch_tests.rs` |
| 267 | `src/crates/kjxlkj-core-state/src/editor_op_resolve.rs` |
| 248 | `src/crates/kjxlkj-core-state/src/completion.rs` |
| 234 | `src/crates/kjxlkj-core-state/src/floating.rs` |
| 232 | `src/crates/kjxlkj-core-state/src/snippets.rs` |
| 224 | `src/crates/kjxlkj-core-state/src/editor_tabs.rs` |
| 223 | `src/crates/kjxlkj-core-state/src/editor_window_adv.rs` |
| 222 | `src/crates/kjxlkj-core-state/tests/boundary_tests_1.rs` |
| 221 | `src/crates/kjxlkj-core-state/src/folds_advanced.rs` |
| 221 | `src/crates/kjxlkj-core-mode/src/normal_g_z.rs` |
| 218 | `src/crates/kjxlkj-core-state/src/session_features.rs` |
| 218 | `src/crates/kjxlkj-core-state/src/buffer_options.rs` |
| 217 | `src/crates/kjxlkj-core-state/tests/boundary_tests_2.rs` |
| 214 | `src/crates/kjxlkj-core-state/src/theming.rs` |
| 211 | `src/crates/kjxlkj-core-state/src/editor_auto_marks.rs` |
| 204 | `src/crates/kjxlkj-core-state/src/autocmd.rs` |
| 202 | `src/crates/kjxlkj-core-state/src/filetype.rs` |
| 202 | `src/crates/kjxlkj-core-edit/src/text_object_exec.rs` |
| 201 | `src/crates/kjxlkj-core-state/src/file_explorer.rs` |
| 201 | `src/crates/kjxlkj-core-state/src/auto_session.rs` |

## Files Exceeding 200 Lines (Post-Compaction Snapshot)

- Scope: `src/**/*.rs`
- Result: 7 files over 200 lines after editor.rs split

| Lines | File | Notes |
|---:|---|---|
| 362 | `src/crates/kjxlkj-core-state/src/editing_ops.rs` | Split from editor.rs; many small methods |
| 357 | `src/crates/kjxlkj-core-state/src/editor.rs` | Core dispatch hub; cannot reduce without losing cohesion |
| 290 | `src/crates/kjxlkj-core-state/src/window_tree.rs` | Window layout tree; single concern |
| 290 | `src/crates/kjxlkj-core-edit/src/motion.rs` | Motion resolver; inherently large match |
| 261 | `src/crates/kjxlkj-render/src/grid.rs` | Grid builder; single function with setup |
| 242 | `src/crates/kjxlkj-core-state/src/buffer_list.rs` | Buffer collection management |
| 230 | `src/crates/kjxlkj-core-state/src/cursor_ops.rs` | Split from editor.rs; cursor methods |
| 211 | `src/crates/kjxlkj/src/main.rs` | Entry point; startup/shutdown sequence |

## Required Decomposition Actions

- Split `core-state` into domain folders with approximately 8-14 files each.
- Split oversized dispatch and feature modules before feature expansion.
- Separate test suites by behavior domain to keep individual files below 200 lines.

## Canonical Follow-Up

- [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- [/docs/todo/current/areas/architecture.md](/docs/todo/current/areas/architecture.md)
