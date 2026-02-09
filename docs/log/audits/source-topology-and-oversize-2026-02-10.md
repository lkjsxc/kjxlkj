# Audit: Source Topology and Oversize Files (2026-02-10)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Goal

Capture post-Wave-3 source-structure hotspots. 8 new modules were added in Wave 3
(user_commands, events, registers, marks, search, session, contracts, editing_helpers)
and ex_commands.rs grew significantly.

## Files Exceeding 200 Lines (Wave 3 Snapshot)

- Scope: `src/**/*.rs`
- Result: 18 files over 200 lines

| Lines | File | Notes |
|---:|---|---|
| 755 | `kjxlkj-core-state/src/ex_commands.rs` | Ex command dispatch + map/user/autocmd handlers; candidate for split |
| 539 | `kjxlkj-core-state/src/editor.rs` | Core dispatch hub; grew with marks/events/user_commands fields |
| 373 | `kjxlkj-core-state/src/search.rs` | Search engine with smartcase, highlighting, 9 tests |
| 362 | `kjxlkj-core-state/src/editing_ops.rs` | Line/char editing operations |
| 358 | `kjxlkj-core-state/src/user_commands.rs` | User command registry with parse/expand, 9 tests |
| 325 | `kjxlkj-core-state/src/ex_parse.rs` | Ex command parser with range/address resolution |
| 316 | `kjxlkj-core-state/src/events.rs` | Autocmd/event system, 30+ EventKinds, 7 tests |
| 290 | `kjxlkj-core-state/src/window_tree.rs` | Window layout tree; single concern |
| 290 | `kjxlkj-core-edit/src/motion.rs` | Motion resolver; inherently large match |
| 279 | `kjxlkj-core-state/src/marks.rs` | Mark system with adjust_for_edit, 7 tests |
| 262 | `kjxlkj-core-state/src/registers.rs` | Named/numbered register file, 8 tests |
| 261 | `kjxlkj-render/src/grid.rs` | Grid builder |
| 253 | `kjxlkj-core-state/src/editing_helpers.rs` | Auto-pairs, comment toggle, surround, 7 tests |
| 251 | `kjxlkj-core-state/src/session.rs` | Session save/load, 3 tests |
| 242 | `kjxlkj-core-state/src/buffer_list.rs` | Buffer collection management |
| 230 | `kjxlkj-core-state/src/cursor_ops.rs` | Cursor motion methods |
| 221 | `kjxlkj-core-state/src/mappings.rs` | Key mapping engine, 5 tests |
| 211 | `kjxlkj/src/main.rs` | Entry point; startup/shutdown |

## Priority Split Candidates

1. **ex_commands.rs (755 lines)**: Split into:
   - `ex_dispatch.rs` — dispatch table and simple handlers
   - `ex_substitution.rs` — substitute_line helpers
   - `ex_map.rs` — map/unmap command handling
   - `ex_user_cmd.rs` — :command/:delcommand dispatch
   - `ex_autocmd.rs` — :autocmd dispatch
   - `ex_marks_regs.rs` — :mark/:delmarks/:registers dispatch

2. **editor.rs (539 lines)**: Split into:
   - `editor.rs` — EditorState struct, constructor, core dispatch
   - `editor_actions.rs` — action handlers
   - `editor_mode.rs` — mode transition helpers

## Summary

Wave 3 added 10 new oversized files. The ex_commands.rs file is the most critical
split candidate at 755 lines. Future waves should enforce the 200-line target more
aggressively by splitting modules before adding new features.
