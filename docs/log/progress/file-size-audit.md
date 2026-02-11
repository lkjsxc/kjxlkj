# File Size Audit

Source files exceeding 200 lines are listed here per project policy.
All files are now ≤ 200 lines.

## Current Audit (all clear)

| File | Lines | Status |
|------|-------|--------|
| `normal.rs` (kjxlkj-core-mode) | 199 | OK |
| `action.rs` (kjxlkj-core-types) | 196 | OK |
| `motion_find.rs` (kjxlkj-core-edit) | 196 | OK |
| `layout_ops.rs` (kjxlkj-core-ui) | 191 | OK |
| `motion.rs` (kjxlkj-core-edit) | 190 | OK |
| `editor_ops.rs` (kjxlkj-core-state) | 173 | OK |
| `other_modes.rs` (kjxlkj-core-mode) | 168 | OK |
| `editor.rs` (kjxlkj-core-state) | 163 | OK |
| `frame.rs` (kjxlkj-render) | 159 | OK |
| `editor_edit.rs` (kjxlkj-core-state) | 151 | OK |

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
- Monitor `normal.rs` (199), `action.rs` (196), `motion_find.rs` (196), `layout_ops.rs` (191), `motion.rs` (190) as they approach the limit.
