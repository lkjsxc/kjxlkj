# Oversized Source Files Registry

Files exceeding the 200-line threshold per docs/policy/STRUCTURE.md.

| File | Lines | Notes |
|---|---:|---|
| src/crates/kjxlkj-core-state/src/editor.rs | 1086 | Central editor state. Contains handle_action, handle_key, cursor movement, editing, undo/redo, ex commands, window management. Split candidate: extract cursor_movement.rs, editing_ops.rs, ex_commands.rs. |
| src/crates/kjxlkj-core-state/src/window_tree.rs | 290 | Window tree layout. Single logical unit, acceptable. |
| src/crates/kjxlkj-core-edit/src/motion.rs | 290 | Motion resolution. Single logical unit, acceptable. |
| src/crates/kjxlkj-render/src/grid.rs | 261 | Grid building from snapshots. Single logical unit, acceptable. |
| src/crates/kjxlkj-core-state/src/buffer_list.rs | 242 | Buffer list management. Single logical unit, acceptable. |
| src/crates/kjxlkj/src/main.rs | 211 | Binary entry point. Borderline, acceptable. |
