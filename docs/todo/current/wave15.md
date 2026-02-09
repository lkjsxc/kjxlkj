# Wave 15 — Regex Search Count, Session Marks, Block Paste, Wildmenu Scroll, Expr Cmdline, Formatprg, Regex Branches, Snippets Stub

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Regex-aware search count, persist global marks in sessions, block visual paste,
wildmenu scrolling for large completion lists, full expression = cmdline prompt,
formatprg external formatter support, regex branch alternation, and snippets
engine stub.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-REGEXCOUNT-01 | Search count uses regex for \v patterns | `/docs/spec/editing/search/README.md` | `[x]` |
| REQ-SESSMARKS-01 | Persist global marks (A-Z) in session save/load | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-BLOCKPASTE-01 | Block visual paste inserts column-wise | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-WILDSCROLL-01 | Wildmenu scrolling when completions exceed width | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-EXPRCMD-01 | Expression = opens mini cmdline prompt | `/docs/spec/editing/registers/expression-register.md` | `[x]` |
| REQ-FORMATPRG-01 | gq uses formatprg option if set | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-REGEXBRANCH-01 | Regex branch alternation \| in magic mode | `/docs/spec/editing/regex/README.md` | `[x]` |
| REQ-SNIPPETS-01 | Snippets engine stub with expand trigger | `/docs/spec/features/editing/README.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 200 lines
- LIMITATIONS updated
