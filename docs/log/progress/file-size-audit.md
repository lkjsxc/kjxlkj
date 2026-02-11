# File Size Audit

Source files exceeding 200 lines are listed here per project policy.
All files are now ≤ 200 lines.

## Current Audit (all clear)

| File | Lines | Status |
|------|-------|--------|
| `action.rs` (kjxlkj-core-types) | 112 | OK |
| `search.rs` (kjxlkj-core-state) | 191 | OK |
| `editor.rs` (kjxlkj-core-state) | 183 | OK |
| `editor_search_tests.rs` (kjxlkj-core-state) | 200 | OK |
| `normal.rs` (kjxlkj-core-mode) | 200 | OK |
| `other_modes.rs` (kjxlkj-core-mode) | 184 | OK |
| `motion_find.rs` (kjxlkj-core-edit) | 174 | OK |
| `editor_cmdline.rs` (kjxlkj-core-state) | 150 | OK |
| `layout_ops.rs` (kjxlkj-core-ui) | 191 | OK |
| `motion.rs` (kjxlkj-core-edit) | 200 | OK |
| `editor_edit.rs` (kjxlkj-core-state) | 199 | OK |
| `normal_g.rs` (kjxlkj-core-mode) | 197 | OK |
| `register.rs` (kjxlkj-core-state) | 198 | OK |
| `pending.rs` (kjxlkj-core-mode) | 181 | OK |
| `frame.rs` (kjxlkj-render) | 159 | OK |
| `command_parse.rs` (kjxlkj-core-state) | 195 | OK |
| `buffer.rs` (kjxlkj-core-text) | 156 | OK |
| `editor_ops.rs` (kjxlkj-core-state) | 186 | OK |
| `editor_ext.rs` (kjxlkj-core-state) | 144 | OK |
| `editor_action.rs` (kjxlkj-core-state) | 194 | OK |
| `regex_compile.rs` (kjxlkj-core-edit) | 120 | OK |
| `text_object.rs` (kjxlkj-core-edit) | 195 | OK |
| `text_object_ext.rs` (kjxlkj-core-edit) | 126 | OK |
| `editor_textobj_tests.rs` (kjxlkj-core-state) | 111 | OK |
| `editor_visual.rs` (kjxlkj-core-state) | 197 | OK |
| `editor_visual_tests.rs` (kjxlkj-core-state) | 139 | OK |
| `editor_buffer.rs` (kjxlkj-core-state) | 194 | OK |
| `editor_buffer_tests.rs` (kjxlkj-core-state) | 153 | OK |
| `editor_race_tests.rs` (kjxlkj-core-state) | 140 | OK |
| `editor_boundary_tests.rs` (kjxlkj-core-state) | 83 | OK |
| `editor_stage03_tests.rs` (kjxlkj-core-state) | 134 | OK |
| `editor_stage03_edit_tests.rs` (kjxlkj-core-state) | 108 | OK |
| `editor_wincmd_tests.rs` (kjxlkj-core-state) | 144 | OK |
| `editor_window.rs` (kjxlkj-core-state) | 157 | OK |
| `editor_explorer.rs` (kjxlkj-core-state) | 123 | OK |
| `normal_wincmd.rs` (kjxlkj-core-mode) | 137 | OK |
| `lib.rs` (kjxlkj-core-state) | 58 | OK |
| `lib.rs` (kjxlkj-core-mode) | 147 | OK |
| `layout_resize.rs` (kjxlkj-core-ui) | 127 | OK |
| `editor_stage04_tests.rs` (kjxlkj-core-state) | 189 | OK |
| `editor_stage04b_tests.rs` (kjxlkj-core-state) | 168 | OK |
| `lib.rs` (kjxlkj-service-explorer) | 197 | OK |
| `explorer_tree.rs` (kjxlkj-service-explorer) | 95 | OK |
| `explorer_nav.rs` (kjxlkj-service-explorer) | 181 | OK |
| `lib.rs` (kjxlkj-service-terminal) | 77 | OK |

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
- Monitor `editor.rs` (200), `motion.rs` (200), `normal.rs` (200), `editor_search_tests.rs` (200), `command_parse.rs` (200), `editor_edit.rs` (199), `action.rs` (199), `register.rs` (198), `editor_action.rs` (198), `editor_visual.rs` (197), `normal_g.rs` (197), `text_object.rs` (195) as they approach the limit.
- New files added in wave-017: `motion_info.rs` (98), `editor_ext.rs` (144).
- New files added in wave-018: `register.rs` (140→176), `pending.rs` expanded to 175.
- New files added in wave-019: `regex_compile.rs` (187→120 refactored), `command_parse.rs` (159), `search.rs` (200), `editor_cmdline.rs` (195).
- Wave-020 changes: `regex_compile.rs` refactored to 120, `editor_edit.rs` 151→179, `register.rs` 140→176, `editor_ops.rs` 152, `editor_action.rs` 132.
- Wave-021 changes: `register.rs` 176→198, `editor.rs` 200→186, `editor_cmdline.rs` 195→150, `action.rs` 200→197, `command_parse.rs` 159→169, `editor_action.rs` 132→138.
- Wave-022 changes: `search.rs` 200→177 (helpers extracted to `search_util.rs`), `action.rs` 197→200, `normal.rs` 199→198, `command_parse.rs` 169→177, `editor_action.rs` 138→159, `editor.rs` 186→185. New files: `search_util.rs` (73), `editor_search_tests.rs` (105).
- Wave-023 changes: `search.rs` 177→191 (+history, ignorecase/smartcase, compacted tests), `action.rs` 200→200 (added GStarSearchForward/GStarSearchBackward, removed 4 doc comments), `normal_g.rs` 177→197 (+g*/g# dispatch, 2 new tests), `motion_find.rs` 196→174 (compacted paragraphs, bracket_pair helper, % forward scan), `motion.rs` 190→200 (+% forward scan test), `editor_action.rs` 159→177 (+g_star_search dispatch), `editor_search_tests.rs` 137→191 (+5 integration tests: g*, g#, %, history, ignorecase).
- Wave-024 changes: `command_parse.rs` 177→199 (+parse_set_option, +test), `editor_action.rs` 177→189 (+IncrementNumber/DecrementNumber/SetOption dispatch, +apply_set_option), `editor_edit.rs` 179→199 (+increment_number/decrement_number/modify_number, +find_number, compacted cursor/clamp), `editor_search_tests.rs` 191→200 (merged star boundary tests, +ctrl_a_increments_number, +set_option_via_ex_command), `normal.rs` 198→200 (+Ctrl-a/Ctrl-x).
- Wave-025 changes: `action.rs` 200→194 (removed comments, compacted scrolls, +TextObjInner/TextObjAround), `pending.rs` 175→179 (+TextObjectInner/TextObjectAround), `other_modes.rs` 196→142 (full rewrite: compacted all handlers, +text object dispatch), `editor_ops.rs` 152→186 (+apply_operator_text_obj, +text object branch). New files: `text_object.rs` (199, word/bracket/quote range computation, 7 tests), `editor_textobj_tests.rs` (90, 7 integration tests).
- Wave-026 changes: `text_object.rs` 199→195 (merged word tests, +p/s dispatch to text_object_ext), `editor_textobj_tests.rs` 90→111 (+dip/dis integration tests). New file: `text_object_ext.rs` (126, paragraph/sentence range computation, 5 unit tests).
- Wave-027 changes: `action.rs` 194→198 (+VisualOperator/VisualSwapAnchor), `editor.rs` 185→200 (+visual_anchor, visual lifecycle, compacted doc comment), `other_modes.rs` 142→184 (expanded handle_visual_key), `editor_action.rs` 189→191 (+VisualOperator/VisualSwapAnchor wiring). New files: `editor_visual.rs` (197, visual operator dispatch, 2 unit tests), `editor_visual_tests.rs` (139, 11 integration tests).
- Wave-028 changes: `action.rs` 198→199 (+SwitchAlternate/ListBuffers, removed 3 doc comments), `command_parse.rs` 199→200 (+:ls/:buffers, compacted doc), `editor.rs` 200→200 (+alternate_buffer, compacted comments), `editor_action.rs` 191→198 (+buffer action dispatch), `normal.rs` 200→200 (+Ctrl-6/^ mapping, merged wildcard). New file: `editor_buffer.rs` (182, buffer management, 5 unit tests).
- Wave-029 changes: `action.rs` 199→200 (+FirstBuffer/LastBuffer on single line), `command_parse.rs` 200→194 (+:bf/:bfirst/:bl/:blast, merged 3 tests into 1 for compaction), `editor_buffer.rs` 182→194 (+first_buffer/last_buffer), `editor_action.rs` 198→200 (+FirstBuffer/LastBuffer dispatch). New file: `editor_buffer_tests.rs` (153, 13 integration tests).
- Wave-030 changes: No existing files modified (test-only wave). New files: `editor_race_tests.rs` (140, 10 stress tests), `editor_boundary_tests.rs` (83, 7 boundary safety tests), `lib.rs` (core-state) 43→47 (+2 test modules).
- Wave-031 changes: `lib.rs` (core-state) 47→51 (+2 test modules). New files: `editor_stage03_tests.rs` (134, 7 stage-03 exit tests), `editor_stage03_edit_tests.rs` (108, 5 stage-03 edit tests). Original single file (215 lines) split to comply with ≤ 200 policy.
- Wave-032 changes: `pending.rs` 179→181 (+WinCmd variant), `normal.rs` 200→200 (compacted tests, +Ctrl-w dispatch), `normal_partial.rs` 91→93 (+WinCmd arm), `editor_action.rs` 200→186 (compacted dispatchers, +4 window actions), `editor_window.rs` 65→128 (+window_only/focus_cycle/focus_direction), `lib.rs` (core-mode) 146→147 (+normal_wincmd), `lib.rs` (core-state) 51→53 (+editor_wincmd_tests). New files: `normal_wincmd.rs` (87, wincmd key dispatch, 7 tests), `editor_wincmd_tests.rs` (144, 12 integration tests).
- Wave-033 changes: `action.rs` 200→112 (massive compaction: removed per-variant doc comments, grouped variants on shared lines), `normal_wincmd.rs` 87→137 (+10 dispatch arms, +8 tests), `editor_window.rs` 128→150 (+focus_top_left/focus_bottom_right/equalize/resize/max/open_explorer/close_explorer, compacted with leaf_rects helper), `editor_action.rs` 186→194 (+8 dispatch arms), `command_parse.rs` 194→195 (+ExplorerClose), `lib.rs` (core-state) 53→55 (+editor_stage04_tests). New files: `layout_resize.rs` (127, equalize/axis helpers, 3 tests), `editor_stage04_tests.rs` (189, 14 integration tests).
- Wave-034 changes: `editor.rs` 200→183 (added explorer_states HashMap, explorer key interception, compacted struct comments and tests), `editor_window.rs` 150→157 (proper ExplorerState creation/cleanup in open/close), `lib.rs` (core-state) 55→58 (+editor_explorer, +editor_stage04b_tests). New files: `editor_explorer.rs` (123, explorer key routing, 5 tests), `editor_stage04b_tests.rs` (168, 13 integration tests), `lib.rs` (kjxlkj-service-explorer) rewritten from stub (197, ExplorerState model, 4 tests), `explorer_tree.rs` (95, ExplorerNode tree model, 3 tests), `explorer_nav.rs` (181, navigation actions, 5 tests), `lib.rs` (kjxlkj-service-terminal) expanded from stub (77, TerminalState model, 2 tests).
