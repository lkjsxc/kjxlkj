# File Size Audit

Source files exceeding 200 lines are listed here per project policy.
All files are now ≤ 200 lines.

## Current Audit (all clear)

| File | Lines | Status |
|------|-------|--------|
| `action.rs` (kjxlkj-core-types) | 200 | OK |
| `search.rs` (kjxlkj-core-state) | 200 | OK |
| `editor.rs` (kjxlkj-core-state) | 200 | OK |
| `normal.rs` (kjxlkj-core-mode) | 199 | OK |
| `other_modes.rs` (kjxlkj-core-mode) | 196 | OK |
| `motion_find.rs` (kjxlkj-core-edit) | 196 | OK |
| `editor_cmdline.rs` (kjxlkj-core-state) | 195 | OK |
| `layout_ops.rs` (kjxlkj-core-ui) | 191 | OK |
| `motion.rs` (kjxlkj-core-edit) | 190 | OK |
| `editor_edit.rs` (kjxlkj-core-state) | 179 | OK |
| `normal_g.rs` (kjxlkj-core-mode) | 177 | OK |
| `register.rs` (kjxlkj-core-state) | 176 | OK |
| `pending.rs` (kjxlkj-core-mode) | 175 | OK |
| `frame.rs` (kjxlkj-render) | 159 | OK |
| `command_parse.rs` (kjxlkj-core-state) | 159 | OK |
| `buffer.rs` (kjxlkj-core-text) | 156 | OK |
| `editor_ops.rs` (kjxlkj-core-state) | 152 | OK |
| `editor_ext.rs` (kjxlkj-core-state) | 144 | OK |
| `editor_action.rs` (kjxlkj-core-state) | 132 | OK |
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
- Monitor `action.rs` (200), `search.rs` (200), `editor.rs` (200), `normal.rs` (199), `other_modes.rs` (196), `motion_find.rs` (196), `editor_cmdline.rs` (195) as they approach the limit.
- New files added in wave-017: `motion_info.rs` (98), `editor_ext.rs` (144).
- New files added in wave-018: `register.rs` (140→176), `pending.rs` expanded to 175.
- New files added in wave-019: `regex_compile.rs` (187→120 refactored), `command_parse.rs` (159), `search.rs` (200), `editor_cmdline.rs` (195).
- Wave-020 changes: `regex_compile.rs` refactored to 120, `editor_edit.rs` 151→179, `register.rs` 140→176, `editor_ops.rs` 152, `editor_action.rs` 132.
