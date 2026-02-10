# Source File Length Audit

Back: [README.md](README.md)

Files exceeding 200 lines are recorded here per policy.

## Current Audit (2026-02-10, wave-3)

All 116 source files are under 200 lines. No violations. Max file is 200 lines.

## Previously Over 200 (Now Resolved)

| File | Was | Now | Resolution |
|---|---|---|---|
| `editor_actions.rs` | 573 | 186 | Split to `editing_ops.rs`, `undo_ops.rs`, `cmdline_ops.rs`, `service_dispatch.rs` |
| `painter.rs` | 388 | ~60 | Split to `paint_window.rs`, `paint_chrome.rs`, `paint_flush.rs` |
| `window_tree.rs` | 259 | 127 | Extracted `tab_page.rs` |
| `motion.rs` | 241 | 145 | Extracted `word_motion.rs` |
| `normal.rs` | 229 | 171 | Extracted `normal_tests.rs` |
| `dispatch.rs` | 221 | 125 | Extracted `key_motion.rs` |
| `client.rs` | 250 | 169 | Extracted `response.rs` for LSP response types |
| `syntax.rs` | 208 | 163 | Extracted `lang_keywords.rs` for keyword tables |

## Directory Reorganization

| Directory | Was | Now | Action |
|---|---|---|---|
| `kjxlkj-core-state/src/` | 23 children | 8 children | Created `ops/` (11 files) and `tests/` (6 files) subdirs |
| `kjxlkj-render/src/` | 16 children | 12 children | Created `paint/` (5 files) subdir |

## Files to Watch

| File | Lines | Status | Action |
|---|---|---|---|
| `src/crates/kjxlkj-service-index/src/service.rs` | 200 | At limit | Extract if grows |
| `src/crates/kjxlkj-core-edit/src/edit_tests.rs` | 196 | Near limit | Monitor |
| `src/crates/kjxlkj-core-state/src/ops/editing_ops.rs` | 195 | Near limit | Monitor |
| `src/crates/kjxlkj-core-state/src/ops/editor_actions.rs` | 186 | OK | Monitor |
| `src/crates/kjxlkj-core-mode/src/mode_tests.rs` | 186 | OK | Monitor |
| `src/crates/kjxlkj-service-git/src/service.rs` | 182 | OK | Monitor |
