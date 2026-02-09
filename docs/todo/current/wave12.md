# Wave 12 — Jump List, Alternate Register, Count Macros, :noh, Nomagic, Visual Block Render, Modeline, Session Layout

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Wire jump list (Ctrl-O/Ctrl-I, :jumps), alternate file register (#),
count-prefixed macro playback (3@a), :nohlsearch command, nomagic
regex mode, visual block column highlighting in render, modeline
parsing, and session window layout persistence.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-JUMPLIST-01 | Ctrl-O/Ctrl-I navigate jump list; :jumps lists entries | `/docs/spec/editing/marks/jumplist.md` | `[x]` |
| REQ-ALTREG-01 | # register returns alternate (previous) buffer filename | `/docs/spec/editing/registers/special-registers.md` | `[x]` |
| REQ-COUNTMACRO-01 | Count-prefixed macro playback: 3@a executes macro a three times | `/docs/spec/editing/macros/recursive-macros.md` | `[x]` |
| REQ-NOH-01 | :nohlsearch / :noh clears search highlighting | `/docs/spec/editing/search/search-highlight.md` | `[x]` |
| REQ-NOMAGIC-01 | nomagic regex mode: only ^ and $ special; \m prefix | `/docs/spec/editing/regex/README.md` | `[x]` |
| REQ-VBLOCKRENDER-01 | Visual block selection renders column highlight in grid | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-MODELINE-01 | Parse vim modeline in first/last 5 lines of buffer | `/docs/spec/features/config/modeline.md` | `[x]` |
| REQ-SESSLAYOUT-01 | :mksession persists and restores window split layout | `/docs/spec/features/session/README.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 200 lines
- LIMITATIONS updated
