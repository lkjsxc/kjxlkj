# Wave 4 — Operator-Motion Composition, Visual Mode, Marks, Registers

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Core editing grammar and modal behavior wiring.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-OP-01 | Operator + motion composition (d{motion}, c{motion}, y{motion}) | `/docs/spec/editing/operators/operator-grammar.md` | `[ ]` |
| REQ-OP-02 | Doubled operators (dd, cc, yy, >>, <<, ==) | `/docs/spec/editing/operators/operator-grammar.md` | `[ ]` |
| REQ-OP-03 | Count multiplication ([count]op[count]motion) | `/docs/spec/editing/operators/operator-grammar.md` | `[ ]` |
| REQ-VIS-01 | Visual char selection (v) with motions | `/docs/spec/modes/visual.md` | `[ ]` |
| REQ-VIS-02 | Visual line selection (V) with motions | `/docs/spec/modes/visual.md` | `[ ]` |
| REQ-VIS-03 | Operators in visual mode (d, c, y, >, <) | `/docs/spec/modes/visual.md` | `[ ]` |
| REQ-VIS-04 | Visual `o` command (swap anchor/cursor) | `/docs/spec/modes/visual.md` | `[ ]` |
| REQ-MARK-01 | Set marks with m{char} | `/docs/spec/editing/marks/README.md` | `[ ]` |
| REQ-MARK-02 | Jump to mark with '{char} and `{char} | `/docs/spec/editing/marks/README.md` | `[ ]` |
| REQ-REG-01 | Register prefix "{char} before operations | `/docs/spec/editing/registers/README.md` | `[ ]` |
| REQ-SEARCH-01 | Search navigation with n/N | `/docs/spec/editing/search/README.md` | `[ ]` |
| REQ-PREFIX-01 | g-prefix commands (gg) | `/docs/spec/editing/motions/motions.md` | `[ ]` |
| REQ-PREFIX-02 | z-prefix commands (zz, zt, zb) | `/docs/spec/editing/motions/motions.md` | `[ ]` |

## Implementation Plan

1. Restructure `NormalDispatch` with unified pending prefix
2. Add visual/op-pending state fields to `EditorState`
3. Create `op_pending.rs` — operator-pending mode dispatch
4. Create `editing_ops_ranges.rs` — range-based delete/yank/change
5. Create `visual_ops.rs` — visual mode dispatch
6. Wire dispatch in `editor_modes.rs`
7. Handle mark/register actions in `editor_actions.rs`
8. Fix visual transitions in `transition.rs`
9. Add tests

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all new + existing tests
- `cargo clippy` zero warnings
- All files ≤ 199 lines
- CONFORMANCE and LIMITATIONS updated
