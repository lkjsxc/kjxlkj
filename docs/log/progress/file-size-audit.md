# File Size Audit

Source files exceeding 200 lines are listed here per project policy.
All files are now ≤ 200 lines.

## Current Audit (all clear)

| File | Lines | Status |
|------|-------|--------|
| `layout_ops.rs` (kjxlkj-core-ui) | 191 | OK |
| `normal.rs` (kjxlkj-core-mode) | 190 | OK |
| `frame.rs` (kjxlkj-render) | 159 | OK |
| `editor.rs` (kjxlkj-core-state) | 155 | OK |
| `action.rs` (kjxlkj-core-types) | 154 | OK |
| `editor_edit.rs` (kjxlkj-core-state) | 151 | OK |
| `motion.rs` (kjxlkj-core-edit) | 151 | OK |

## Splits Performed

| Original | Lines | Split Into |
|----------|-------|------------|
| `editor.rs` | 499 | `editor.rs` (155), `editor_action.rs` (82), `editor_edit.rs` (151), `editor_snapshot.rs` (125), `editor_window.rs` (65) |
| `layout.rs` | 322 | `layout.rs` (149), `layout_ops.rs` (191) |
| `normal.rs` | 240 | `normal.rs` (190), `normal_motions.rs` (95) |
| `motion.rs` | 220 | `motion.rs` (151), `motion_word.rs` (80) |
| `buffer.rs` | 206 | `buffer.rs` (129), `buffer_edit.rs` (91) |

## Notes

- All source files now comply with the ≤ 200 line policy.
- Monitor `layout_ops.rs` (191) and `normal.rs` (190) as they approach the limit.
