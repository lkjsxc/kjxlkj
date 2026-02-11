# File Size Audit

Source files exceeding 200 lines are listed here per project policy.
All files are now ≤ 200 lines.

## Current Audit (all clear)

| File | Lines | Status |
|------|-------|--------|
| `action.rs` (kjxlkj-core-types) | 200 | OK |
| `normal.rs` (kjxlkj-core-mode) | 199 | OK |
| `editor.rs` (kjxlkj-core-state) | 198 | OK |
| `other_modes.rs` (kjxlkj-core-mode) | 196 | OK |
| `motion_find.rs` (kjxlkj-core-edit) | 196 | OK |
| `layout_ops.rs` (kjxlkj-core-ui) | 191 | OK |
| `motion.rs` (kjxlkj-core-edit) | 190 | OK |
| `normal_g.rs` (kjxlkj-core-mode) | 177 | OK |
| `pending.rs` (kjxlkj-core-mode) | 175 | OK |
| `frame.rs` (kjxlkj-render) | 159 | OK |
| `buffer.rs` (kjxlkj-core-text) | 156 | OK |
| `editor_edit.rs` (kjxlkj-core-state) | 151 | OK |
| `editor_ext.rs` (kjxlkj-core-state) | 144 | OK |
| `register.rs` (kjxlkj-core-state) | 140 | OK |

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
- Monitor `action.rs` (200), `normal.rs` (199), `editor.rs` (198), `other_modes.rs` (196), `motion_find.rs` (196) as they approach the limit.
- New files added in wave-017: `motion_info.rs` (98), `editor_ext.rs` (144).
- New files added in wave-018: `register.rs` (140), `pending.rs` expanded to 175.
