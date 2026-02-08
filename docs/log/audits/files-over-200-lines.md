# Files Exceeding 200-Line Limit

Per [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md), individual source files
should stay under 200 lines. The following files currently exceed that limit.

## Current violations (14 files)

| Lines | File | Reason |
|------:|------|--------|
| 450 | `src/crates/kjxlkj-core-mode/src/normal.rs` | Normal mode dispatch covers ~35 motion keys + operators + counts + g-prefix + z-prefix |
| 360 | `src/crates/kjxlkj-core-edit/src/text_object_exec.rs` | 10+ text-object types (word, WORD, sentence, paragraph, bracket, quote) each need match arms + tests |
| 343 | `src/crates/kjxlkj-core-edit/src/motion_exec.rs` | ~30 motion variants each need execution logic + tests |
| 288 | `src/crates/kjxlkj-core-mode/src/command.rs` | Command-line mode state machine: editing, history, char events, completion stub |
| 278 | `src/crates/kjxlkj-core-undo/src/tree.rs` | Arena-based undo tree with branching, group boundaries, persistence, and tests |
| 264 | `src/crates/kjxlkj-core-edit/src/operator_exec.rs` | 9 operators (delete, change, yank, indent, etc.) with linewise/characterwise variants |
| 248 | `src/crates/kjxlkj-render/src/renderer.rs` | Full render pipeline: grid build, gutter, statusline, diff, flush |
| 242 | `src/crates/kjxlkj-core-types/src/action.rs` | Action enum with ~70+ variants covering all editor actions |
| 240 | `src/crates/kjxlkj-core-state/src/editor.rs` | EditorState: buffer/window management, action dispatch, snapshot production |
| 240 | `src/crates/kjxlkj-core-mode/src/visual.rs` | Visual mode state for char/line/block variants + selection logic |
| 234 | `src/crates/kjxlkj-core-text/src/grapheme.rs` | Grapheme decomposition, display width, cursor mapping with tests |
| 224 | `src/crates/kjxlkj-core-text/src/buffer_content.rs` | Rope wrapper: insert, delete, line ops, char_at, len methods |
| 216 | `src/crates/kjxlkj-service-terminal/src/escape_parser.rs` | 5-state escape sequence parser (Ground/Escape/CSI/OSC/Param) |
| 201 | `src/crates/kjxlkj-core-text/src/word.rs` | Word/WORD boundary detection with classify + tests |

## Mitigation

These files are at the boundary of what can be split without harming cohesion.
Each contains a single logical unit (one state machine, one large enum, or one
dispatch table) that does not split cleanly. Future refactoring could:

1. Extract sub-dispatchers (e.g., normal.rs → normal_motions.rs + normal_operators.rs)
2. Move tests to separate test modules/files
3. Use code generation for large enum variants

## Tracking

Created: reconstruction wave 3
Status: documented, accepted for now
