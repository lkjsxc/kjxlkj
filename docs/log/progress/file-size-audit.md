# File Size Audit

Source files exceeding 200 lines are listed here per project policy.
All files are now ≤ 200 lines.

## Current Audit (all clear)

| File | Lines | Status |
|------|-------|--------|
| `action.rs` (kjxlkj-core-types) | 200 | OK |
| `search.rs` (kjxlkj-core-state) | 191 | OK |
| `editor.rs` (kjxlkj-core-state) | 185 | OK |
| `normal.rs` (kjxlkj-core-mode) | 198 | OK |
| `other_modes.rs` (kjxlkj-core-mode) | 196 | OK |
| `motion_find.rs` (kjxlkj-core-edit) | 174 | OK |
| `editor_cmdline.rs` (kjxlkj-core-state) | 150 | OK |
| `layout_ops.rs` (kjxlkj-core-ui) | 191 | OK |
| `motion.rs` (kjxlkj-core-edit) | 200 | OK |
| `editor_edit.rs` (kjxlkj-core-state) | 179 | OK |
| `normal_g.rs` (kjxlkj-core-mode) | 197 | OK |
| `register.rs` (kjxlkj-core-state) | 198 | OK |
| `pending.rs` (kjxlkj-core-mode) | 175 | OK |
| `frame.rs` (kjxlkj-render) | 159 | OK |
| `command_parse.rs` (kjxlkj-core-state) | 177 | OK |
| `buffer.rs` (kjxlkj-core-text) | 156 | OK |
| `editor_ops.rs` (kjxlkj-core-state) | 152 | OK |
| `editor_ext.rs` (kjxlkj-core-state) | 144 | OK |
| `editor_action.rs` (kjxlkj-core-state) | 177 | OK |
| `regex_compile.rs` (kjxlkj-core-edit) | 120 | OK |

## Splits Performed

| Original | Lines | Split Into |
|----------|-------|------------|
| `editor.rs` | 499 | `editor.rs` (155), `editor_action.rs` (82), `editor_edit.rs` (151), `editor_snapshot.rs` (125), `editor_window.rs` (65) |
| `layout.rs` | 322 | `layout.rs` (149), `layout_ops.rs` (191) |
| `normal.rs` | 240 | `normal.rs` (190), `normal_motions.rs` (95) |
| `motion.rs` | 220 | `motion.rs` (190), `motion_word.rs` (147), `motion_find.rs` (196), `motion_big_word.rs` (100) |
| `buffer.rs` | 206 | `buffer.rs` (129), `buffer_edit.rs` (91) |
| `lib.rs` (core-mode) | 267 | `lib.rs` (146), `normal_partial.rs` (85), `other_modes.rs` (168) |

## Notes

- All source files now comply with the ≤ 200 line policy.
- Monitor `action.rs` (200), `motion.rs` (200), `register.rs` (198), `normal.rs` (198), `normal_g.rs` (197), `other_modes.rs` (196) as they approach the limit.
- New files added in wave-017: `motion_info.rs` (98), `editor_ext.rs` (144).
- New files added in wave-018: `register.rs` (140→176), `pending.rs` expanded to 175.
- New files added in wave-019: `regex_compile.rs` (187→120 refactored), `command_parse.rs` (159), `search.rs` (200), `editor_cmdline.rs` (195).
- Wave-020 changes: `regex_compile.rs` refactored to 120, `editor_edit.rs` 151→179, `register.rs` 140→176, `editor_ops.rs` 152, `editor_action.rs` 132.
- Wave-021 changes: `register.rs` 176→198, `editor.rs` 200→186, `editor_cmdline.rs` 195→150, `action.rs` 200→197, `command_parse.rs` 159→169, `editor_action.rs` 132→138.
- Wave-022 changes: `search.rs` 200→177 (helpers extracted to `search_util.rs`), `action.rs` 197→200, `normal.rs` 199→198, `command_parse.rs` 169→177, `editor_action.rs` 138→159, `editor.rs` 186→185. New files: `search_util.rs` (73), `editor_search_tests.rs` (105).
- Wave-023 changes: `search.rs` 177→191 (+history, ignorecase/smartcase, compacted tests), `action.rs` 200→200 (added GStarSearchForward/GStarSearchBackward, removed 4 doc comments), `normal_g.rs` 177→197 (+g*/g# dispatch, 2 new tests), `motion_find.rs` 196→174 (compacted paragraphs, bracket_pair helper, % forward scan), `motion.rs` 190→200 (+% forward scan test), `editor_action.rs` 159→177 (+g_star_search dispatch), `editor_search_tests.rs` 137→191 (+5 integration tests: g*, g#, %, history, ignorecase).
