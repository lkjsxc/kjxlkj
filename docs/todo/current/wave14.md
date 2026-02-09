# Wave 14 — Wildmenu, Block Change, Search Count, Macro Edit, Global Marks, Range Prompt, Class Text Obj, Expr Prompt

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Wildmenu display for command-line completion, block change replication,
search match count display, macro editing via register put/yank, global marks
across files, range confirmation prompt for backwards ranges, class/function
text objects, and expression register mini-prompt evaluation.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-WILDMENU-01 | Show completion candidates in status bar (wildmenu) | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-BLOCKCHG-01 | Block change replicates typed text to all lines | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-SRCHCOUNT-01 | Show [N/M] search match count in status bar | `/docs/spec/editing/search/README.md` | `[x]` |
| REQ-MACROEDIT-01 | Macro editing: yank/put macro register content | `/docs/spec/editing/macros/README.md` | `[x]` |
| REQ-GLOBALMARKS-01 | Global marks (A-Z) navigate across buffers | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-RANGEPROMPT-01 | Backwards range confirmation prompt (E493) | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-CLASSTOBJ-01 | Class/function text objects (ic/ac/if/af) stub | `/docs/spec/editing/text-objects/README.md` | `[x]` |
| REQ-EXPRPROMPT-01 | Expression register mini-prompt (Ctrl-R = expr) | `/docs/spec/editing/registers/expression-register.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 200 lines
- LIMITATIONS updated
