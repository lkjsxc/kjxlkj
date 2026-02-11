# File Size Audit

Source files exceeding 200 lines are listed here per project policy.

## Current Audit

| File | Lines | Status | Action |
|------|-------|--------|--------|
| `src/crates/core/kjxlkj-core-state/src/editor.rs` | ~310 | OVER | Split planned |
| `src/crates/core/kjxlkj-core-ui/src/layout.rs` | ~280 | OVER | Acceptable (tree logic) |
| `src/crates/core/kjxlkj-core-edit/src/motion.rs` | ~180 | OK | Monitor |
| `src/crates/core/kjxlkj-core-types/src/action.rs` | ~140 | OK | Monitor |

## Notes

- editor.rs will grow as more actions are implemented; plan to split action handlers into submodules
- layout.rs tree algorithms are self-contained; splitting risks coherence
